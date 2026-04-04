use crate::audit::UpdateAuditLogger;
use crate::errors::UpdateEngineError;
use crate::model::{SnapshotState, UpdateState, VerificationState};
use crate::orchestration::on_rollback_completed;
use crate::store::UpdateStore;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
struct ConsistencyIssue {
    issue_type: &'static str,
    severity: &'static str,
    attempt_id: String,
    snapshot_id: String,
    expected: String,
    actual: String,
    repairable: bool,
}

impl ConsistencyIssue {
    fn summary(&self) -> String {
        format!(
            "{}:{} attempt={} snapshot={} expected={} actual={} repairable={}",
            self.issue_type,
            self.severity,
            self.attempt_id,
            self.snapshot_id,
            self.expected,
            self.actual,
            self.repairable
        )
    }
}

#[derive(Clone, Debug, Default)]
struct ReconcileReport {
    repaired: bool,
    action_count: usize,
    actions: Vec<String>,
    final_state: String,
    result: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct RecoveryStatusView {
    current_state: String,
    last_snapshot_id: Option<String>,
    last_update_id: Option<String>,
    last_attempt_id: Option<String>,
    last_result: String,
    recovery_mode_available: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct RestorePointView {
    snapshot_id: String,
    update_id: String,
    attempt_id: String,
    kind: String,
    created_at: String,
    bootable: bool,
    reason: String,
    rollback_state: String,
}

pub fn validate_consistency(base_dir: &Path, store: &UpdateStore) -> HashMap<String, String> {
    let issues = collect_consistency_issues(base_dir, store);
    build_consistency_report(&issues)
}

pub fn reconcile_state(
    base_dir: &Path,
    store: &mut UpdateStore,
    audit: &UpdateAuditLogger,
) -> Result<HashMap<String, String>, UpdateEngineError> {
    let issues = collect_consistency_issues(base_dir, store);
    if issues.is_empty() {
        let _ = audit.log("consistency_validation_ok", "ok", "no repair needed");
        return Ok(HashMap::from([
            ("repaired".to_string(), "false".to_string()),
            ("action_count".to_string(), "0".to_string()),
            ("actions_summary".to_string(), "noop".to_string()),
            (
                "final_state".to_string(),
                store.status().current_state.as_str().to_string(),
            ),
            ("result".to_string(), "ok".to_string()),
        ]));
    }

    for issue in &issues {
        let _ = audit.log("consistency_issue_detected", "warning", &issue.summary());
    }
    let _ = audit.log(
        "consistency_validation_failed",
        "failed",
        &issues.iter().map(ConsistencyIssue::summary).collect::<Vec<_>>().join(" | "),
    );
    let _ = audit.log("consistency_repair_begin", "started", "attempting safe owner-side repair");

    let mut report = ReconcileReport {
        repaired: false,
        action_count: 0,
        actions: Vec::new(),
        final_state: store.status().current_state.as_str().to_string(),
        result: "failed".to_string(),
    };

    if try_repair_rollback_mismatch(base_dir, store, audit, &issues, &mut report)? {
        report.repaired = true;
    }
    if try_repair_snapshot_registration(base_dir, store, audit, &issues, &mut report)? {
        report.repaired = true;
    }
    if try_repair_snapshot_rollback_used(base_dir, store, audit, &issues, &mut report)? {
        report.repaired = true;
    }

    report.final_state = store.status().current_state.as_str().to_string();
    report.result = if report.repaired {
        if issues.iter().filter(|issue| !issue.repairable).count() > 0 {
            "partial".to_string()
        } else {
            "ok".to_string()
        }
    } else if issues.iter().all(|issue| !issue.repairable) {
        "failed".to_string()
    } else {
        "partial".to_string()
    };

    if report.repaired {
        let _ = audit.log(
            "consistency_repair_applied",
            "ok",
            &report.actions.join(" | "),
        );
    } else {
        let _ = audit.log(
            "consistency_repair_failed",
            "failed",
            "no safe repair path applied",
        );
    }

    Ok(HashMap::from([
        ("repaired".to_string(), report.repaired.to_string()),
        ("action_count".to_string(), report.action_count.to_string()),
        (
            "actions_summary".to_string(),
            if report.actions.is_empty() {
                "none".to_string()
            } else {
                report.actions.join(" | ")
            },
        ),
        ("final_state".to_string(), report.final_state),
        ("result".to_string(), report.result),
    ]))
}

fn collect_consistency_issues(base_dir: &Path, store: &UpdateStore) -> Vec<ConsistencyIssue> {
    let status = store.status();
    let attempts = store.attempts();
    let snapshots = store.snapshots();
    let restore_points = read_restore_points(base_dir);
    let recovery_status = read_recovery_status(base_dir);
    let mut issues = Vec::new();

    for attempt in &attempts {
        if attempt.snapshot_id.is_empty() {
            continue;
        }
        match store.get_snapshot(&attempt.snapshot_id) {
            Some(snapshot) => {
                if snapshot.attempt_id != attempt.attempt_id || snapshot.update_id != attempt.update_id {
                    issues.push(ConsistencyIssue {
                        issue_type: "ATTEMPT_SNAPSHOT_MISMATCH",
                        severity: "high",
                        attempt_id: attempt.attempt_id.clone(),
                        snapshot_id: attempt.snapshot_id.clone(),
                        expected: format!(
                            "snapshot.attempt_id={} snapshot.update_id={}",
                            attempt.attempt_id, attempt.update_id
                        ),
                        actual: format!(
                            "snapshot.attempt_id={} snapshot.update_id={}",
                            snapshot.attempt_id, snapshot.update_id
                        ),
                        repairable: false,
                    });
                }
                if snapshot.restore_registered
                    && !restore_points
                        .iter()
                        .any(|point| point.snapshot_id == snapshot.snapshot_id)
                {
                    issues.push(ConsistencyIssue {
                        issue_type: "SNAPSHOT_RESTORE_POINT_MISMATCH",
                        severity: "medium",
                        attempt_id: attempt.attempt_id.clone(),
                        snapshot_id: snapshot.snapshot_id.clone(),
                        expected: "restore point exists".to_string(),
                        actual: "restore point missing".to_string(),
                        repairable: false,
                    });
                }
                if !snapshot.restore_registered
                    && restore_points
                        .iter()
                        .any(|point| point.snapshot_id == snapshot.snapshot_id)
                {
                    issues.push(ConsistencyIssue {
                        issue_type: "SNAPSHOT_RESTORE_POINT_MISMATCH",
                        severity: "medium",
                        attempt_id: attempt.attempt_id.clone(),
                        snapshot_id: snapshot.snapshot_id.clone(),
                        expected: "snapshot registered=true state=registered".to_string(),
                        actual: format!(
                            "restore_registered={} state={}",
                            snapshot.restore_registered,
                            snapshot.state.as_str()
                        ),
                        repairable: true,
                    });
                }
                if snapshot.state == SnapshotState::RollbackUsed
                    && attempt.state != UpdateState::RolledBack
                    && recovery_status.current_state == "RollbackCompleted"
                {
                    issues.push(ConsistencyIssue {
                        issue_type: "ROLLBACK_MISMATCH",
                        severity: "high",
                        attempt_id: attempt.attempt_id.clone(),
                        snapshot_id: snapshot.snapshot_id.clone(),
                        expected: "attempt.state=rolled_back".to_string(),
                        actual: format!("attempt.state={}", attempt.state.as_str()),
                        repairable: true,
                    });
                }
            }
            None => issues.push(ConsistencyIssue {
                issue_type: "ATTEMPT_SNAPSHOT_MISMATCH",
                severity: "high",
                attempt_id: attempt.attempt_id.clone(),
                snapshot_id: attempt.snapshot_id.clone(),
                expected: "snapshot exists".to_string(),
                actual: "snapshot missing".to_string(),
                repairable: false,
            }),
        }

        if attempt.state == UpdateState::Committed
            && (status.current_state != UpdateState::Committed
                || attempt.verification_state != VerificationState::PostApplyOk
                || attempt.rollback_required
                || !attempt.committed)
        {
            issues.push(ConsistencyIssue {
                issue_type: "COMMIT_MISMATCH",
                severity: "high",
                attempt_id: attempt.attempt_id.clone(),
                snapshot_id: attempt.snapshot_id.clone(),
                expected: "status=committed verification=post_apply_ok rollback_required=false committed=true".to_string(),
                actual: format!(
                    "status={} verification={} rollback_required={} committed={}",
                    status.current_state.as_str(),
                    attempt.verification_state.as_str(),
                    attempt.rollback_required,
                    attempt.committed
                ),
                repairable: false,
            });
        }

        if recovery_status.current_state == "Failed"
            && (status.current_state == UpdateState::RolledBack
                || status.current_state == UpdateState::Committed)
        {
            issues.push(ConsistencyIssue {
                issue_type: "FAILURE_MISMATCH",
                severity: "high",
                attempt_id: attempt.attempt_id.clone(),
                snapshot_id: attempt.snapshot_id.clone(),
                expected: "update state not rolled_back/committed after recovery failure".to_string(),
                actual: format!("update_status={}", status.current_state.as_str()),
                repairable: false,
            });
        }
    }

    if recovery_status.current_state == "RollbackCompleted" {
        let snapshot_id = recovery_status.last_snapshot_id.clone().unwrap_or_default();
        let attempt = store.find_attempt_by_snapshot(&snapshot_id);
        let snapshot = store.get_snapshot(&snapshot_id);
        let mismatch = status.current_state != UpdateState::RolledBack
            || attempt.as_ref().map(|entry| entry.state != UpdateState::RolledBack).unwrap_or(true)
            || attempt.as_ref().map(|entry| entry.rollback_required).unwrap_or(true)
            || attempt.as_ref().map(|entry| entry.committed).unwrap_or(true)
            || snapshot
                .as_ref()
                .map(|entry| entry.state != SnapshotState::RollbackUsed)
                .unwrap_or(true);
        if mismatch {
            issues.push(ConsistencyIssue {
                issue_type: "ROLLBACK_MISMATCH",
                severity: "high",
                attempt_id: attempt.map(|entry| entry.attempt_id).unwrap_or_default(),
                snapshot_id,
                expected: "rolled_back owner state".to_string(),
                actual: format!("update_status={}", status.current_state.as_str()),
                repairable: true,
            });
        }
    }

    if status.current_state == UpdateState::Committed && recovery_status.current_state == "RollbackCompleted" {
        issues.push(ConsistencyIssue {
            issue_type: "TERMINAL_STATE_CONFLICT",
            severity: "high",
            attempt_id: recovery_status.last_attempt_id.unwrap_or_default(),
            snapshot_id: recovery_status.last_snapshot_id.unwrap_or_default(),
            expected: "single terminal truth".to_string(),
            actual: "update=committed and recovery=rollback_completed".to_string(),
            repairable: true,
        });
    }

    issues
}

fn try_repair_rollback_mismatch(
    base_dir: &Path,
    store: &mut UpdateStore,
    audit: &UpdateAuditLogger,
    issues: &[ConsistencyIssue],
    report: &mut ReconcileReport,
) -> Result<bool, UpdateEngineError> {
    let recovery_status = read_recovery_status(base_dir);
    let restore_points = read_restore_points(base_dir);
    if recovery_status.current_state != "RollbackCompleted" {
        return Ok(false);
    }
    let snapshot_id = match recovery_status.last_snapshot_id.clone() {
        Some(snapshot_id) if !snapshot_id.is_empty() => snapshot_id,
        _ => return Ok(false),
    };
    let relevant = issues.iter().any(|issue| {
        issue.issue_type == "ROLLBACK_MISMATCH" || issue.issue_type == "TERMINAL_STATE_CONFLICT"
    });
    if !relevant {
        return Ok(false);
    }
    if !restore_points.iter().any(|point| point.snapshot_id == snapshot_id) {
        return Ok(false);
    }
    on_rollback_completed(store, audit, &snapshot_id, "success")?;
    report.action_count += 1;
    report
        .actions
        .push(format!("rollback_callback_replayed snapshot_id={}", snapshot_id));
    Ok(true)
}

fn try_repair_snapshot_rollback_used(
    base_dir: &Path,
    store: &mut UpdateStore,
    audit: &UpdateAuditLogger,
    issues: &[ConsistencyIssue],
    report: &mut ReconcileReport,
) -> Result<bool, UpdateEngineError> {
    if store.status().current_state == UpdateState::RolledBack {
        return Ok(false);
    }
    let recovery_status = read_recovery_status(base_dir);
    if recovery_status.current_state != "RollbackCompleted" {
        return Ok(false);
    }
    let issue = match issues.iter().find(|issue| {
        issue.issue_type == "ROLLBACK_MISMATCH" && issue.actual.contains("attempt.state=")
    }) {
        Some(issue) => issue,
        None => return Ok(false),
    };
    on_rollback_completed(store, audit, &issue.snapshot_id, "success")?;
    report.action_count += 1;
    report.actions.push(format!(
        "snapshot_rollback_used_promoted_attempt snapshot_id={}",
        issue.snapshot_id
    ));
    Ok(true)
}

fn try_repair_snapshot_registration(
    base_dir: &Path,
    store: &mut UpdateStore,
    _audit: &UpdateAuditLogger,
    issues: &[ConsistencyIssue],
    report: &mut ReconcileReport,
) -> Result<bool, UpdateEngineError> {
    let restore_points = read_restore_points(base_dir);
    let mut repaired = false;
    for issue in issues.iter().filter(|issue| {
        issue.issue_type == "SNAPSHOT_RESTORE_POINT_MISMATCH" && issue.repairable
    }) {
        if let Some(snapshot) = store.get_snapshot(&issue.snapshot_id) {
            let linkage_valid = restore_points.iter().any(|point| {
                point.snapshot_id == snapshot.snapshot_id
                    && point.attempt_id == snapshot.attempt_id
                    && point.update_id == snapshot.update_id
            });
            if linkage_valid {
                let _ = store
                    .set_snapshot_registered(&issue.snapshot_id)
                    .map_err(UpdateEngineError::Store)?;
                report.action_count += 1;
                report.actions.push(format!(
                    "snapshot_registered_repaired snapshot_id={}",
                    issue.snapshot_id
                ));
                repaired = true;
            }
        }
    }
    Ok(repaired)
}

fn build_consistency_report(issues: &[ConsistencyIssue]) -> HashMap<String, String> {
    let mut payload = HashMap::new();
    if issues.is_empty() {
        payload.insert("status".to_string(), "ok".to_string());
        payload.insert("issue_count".to_string(), "0".to_string());
        payload.insert("issues_summary".to_string(), String::new());
        payload.insert("repair_possible".to_string(), "false".to_string());
        return payload;
    }

    payload.insert("status".to_string(), "inconsistent".to_string());
    payload.insert("issue_count".to_string(), issues.len().to_string());
    payload.insert(
        "issues_summary".to_string(),
        issues
            .iter()
            .map(ConsistencyIssue::summary)
            .collect::<Vec<_>>()
            .join(" | "),
    );
    payload.insert(
        "repair_possible".to_string(),
        issues.iter().any(|issue| issue.repairable).to_string(),
    );
    payload
}

fn read_restore_points(base_dir: &Path) -> Vec<RestorePointView> {
    fs::read_to_string(base_dir.join("restore_points.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<Vec<RestorePointView>>(&raw).ok())
        .unwrap_or_default()
}

fn read_recovery_status(base_dir: &Path) -> RecoveryStatusView {
    fs::read_to_string(base_dir.join("recovery_status.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<RecoveryStatusView>(&raw).ok())
        .unwrap_or_default()
}
