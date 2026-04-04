use crate::model::{
    AppliedMarker, SnapshotRecord, SnapshotState, UpdateAttempt, UpdatePackage, UpdateState,
    UpdateStatus, VerificationState,
};
use std::fs;
use std::path::{Path, PathBuf};

pub struct UpdateStore {
    status_path: PathBuf,
    snapshots_path: PathBuf,
    attempts_path: PathBuf,
    apply_marker_path: PathBuf,
    status: UpdateStatus,
    snapshots: Vec<SnapshotRecord>,
    attempts: Vec<UpdateAttempt>,
}

impl UpdateStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let status_path = base_dir.join("update_status.json");
        let snapshots_path = base_dir.join("update_snapshots.json");
        let attempts_path = base_dir.join("update_attempts.json");
        let apply_marker_path = base_dir.join("update_apply_marker.json");
        let status = fs::read_to_string(&status_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<UpdateStatus>(&raw).ok())
            .unwrap_or_default();
        let snapshots = fs::read_to_string(&snapshots_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<Vec<SnapshotRecord>>(&raw).ok())
            .unwrap_or_default();
        let attempts = fs::read_to_string(&attempts_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<Vec<UpdateAttempt>>(&raw).ok())
            .unwrap_or_default();

        Self {
            status_path,
            snapshots_path,
            attempts_path,
            apply_marker_path,
            status,
            snapshots,
            attempts,
        }
    }

    pub fn status(&self) -> UpdateStatus {
        self.status.clone()
    }

    pub fn snapshots(&self) -> Vec<SnapshotRecord> {
        self.snapshots.clone()
    }

    pub fn attempts(&self) -> Vec<UpdateAttempt> {
        self.attempts.clone()
    }

    pub fn get_attempt(&self, attempt_id: &str) -> Option<UpdateAttempt> {
        self.attempts
            .iter()
            .find(|attempt| attempt.attempt_id == attempt_id)
            .cloned()
    }

    pub fn get_package_registry() -> Vec<UpdatePackage> {
        vec![
            UpdatePackage {
                update_id: "update-2026.04.01-core".to_string(),
                version: "2026.04.01".to_string(),
                channel: "stable".to_string(),
                description: "Core runtime rollup with security fixes.".to_string(),
                signed: true,
                payload_kind: "core".to_string(),
                simulated_apply_supported: true,
                created_at: "2026-04-01T10:00:00Z".to_string(),
            },
            UpdatePackage {
                update_id: "update-2026.04.02-shell".to_string(),
                version: "2026.04.02".to_string(),
                channel: "stable".to_string(),
                description: "Shell polish and startup fixes.".to_string(),
                signed: true,
                payload_kind: "shell".to_string(),
                simulated_apply_supported: true,
                created_at: "2026-04-02T09:00:00Z".to_string(),
            },
            UpdatePackage {
                update_id: "update-2026.04.03-unsigned-test".to_string(),
                version: "2026.04.03".to_string(),
                channel: "testing".to_string(),
                description: "Unsigned test package for deny path.".to_string(),
                signed: false,
                payload_kind: "core".to_string(),
                simulated_apply_supported: true,
                created_at: "2026-04-03T08:00:00Z".to_string(),
            },
            UpdatePackage {
                update_id: "update-2026.04.03-shell-post-verify-fail".to_string(),
                version: "2026.04.03".to_string(),
                channel: "testing".to_string(),
                description: "Simulated package that fails post-apply verification.".to_string(),
                signed: true,
                payload_kind: "shell".to_string(),
                simulated_apply_supported: true,
                created_at: "2026-04-03T08:30:00Z".to_string(),
            },
        ]
    }

    pub fn set_state(&mut self, state: UpdateState) -> Result<(), String> {
        self.status.current_state = state;
        self.persist_status()
    }

    pub fn set_active_update(&mut self, update_id: Option<String>) -> Result<(), String> {
        self.status.active_update = update_id;
        self.persist_status()
    }

    pub fn set_last_attempt_id(&mut self, attempt_id: Option<String>) -> Result<(), String> {
        self.status.last_attempt_id = attempt_id;
        self.persist_status()
    }

    pub fn set_rollback_reason(&mut self, reason: Option<String>) -> Result<(), String> {
        self.status.rollback_reason = reason;
        self.persist_status()
    }

    pub fn set_last_committed_update_id(&mut self, update_id: Option<String>) -> Result<(), String> {
        self.status.last_committed_update_id = update_id;
        self.persist_status()
    }

    pub fn set_rollback_metadata(
        &mut self,
        snapshot_id: Option<String>,
        attempt_id: Option<String>,
        rollback_at: Option<String>,
        recovery_result: Option<String>,
    ) -> Result<(), String> {
        self.status.last_rollback_snapshot_id = snapshot_id;
        self.status.last_rollback_attempt_id = attempt_id;
        self.status.last_rollback_at = rollback_at;
        self.status.last_recovery_result = recovery_result;
        self.persist_status()
    }

    pub fn get_restore_link_status(&self, snapshot_id: &str) -> Option<bool> {
        self.get_snapshot(snapshot_id)
            .map(|snapshot| snapshot.restore_registered)
    }

    pub fn add_attempt(&mut self, attempt: UpdateAttempt) -> Result<(), String> {
        self.status.last_attempt_id = Some(attempt.attempt_id.clone());
        self.status.active_update = Some(attempt.update_id.clone());
        self.attempts.push(attempt);
        self.persist_attempts()?;
        self.persist_status()
    }

    pub fn update_attempt(&mut self, updated: UpdateAttempt) -> Result<(), String> {
        if let Some(existing) = self
            .attempts
            .iter_mut()
            .find(|attempt| attempt.attempt_id == updated.attempt_id)
        {
            *existing = updated;
            self.persist_attempts()
        } else {
            Err(format!("attempt {} not found", updated.attempt_id))
        }
    }

    pub fn set_attempt_state(
        &mut self,
        attempt_id: &str,
        state: UpdateState,
        verification_state: Option<VerificationState>,
        failure_reason: Option<String>,
        rollback_required: Option<bool>,
        committed: Option<bool>,
        finished_at: Option<Option<String>>,
    ) -> Result<UpdateAttempt, String> {
        let mut attempt = self
            .get_attempt(attempt_id)
            .ok_or_else(|| format!("attempt {} not found", attempt_id))?;
        attempt.state = state;
        if let Some(value) = verification_state {
            attempt.verification_state = value;
        }
        if let Some(value) = failure_reason {
            attempt.failure_reason = Some(value);
        }
        if let Some(value) = rollback_required {
            attempt.rollback_required = value;
        }
        if let Some(value) = committed {
            attempt.committed = value;
        }
        if let Some(value) = finished_at {
            attempt.finished_at = value;
        }
        self.update_attempt(attempt.clone())?;
        Ok(attempt)
    }

    pub fn set_attempt_rolled_back(
        &mut self,
        attempt_id: &str,
        finished_at: String,
    ) -> Result<UpdateAttempt, String> {
        self.set_attempt_state(
            attempt_id,
            UpdateState::RolledBack,
            Some(VerificationState::PostApplyFailed),
            None,
            Some(false),
            Some(false),
            Some(Some(finished_at)),
        )
    }

    pub fn add_snapshot(&mut self, snapshot: SnapshotRecord) -> Result<(), String> {
        self.snapshots.push(snapshot);
        self.persist_snapshots()
    }

    pub fn update_snapshot(&mut self, updated: SnapshotRecord) -> Result<(), String> {
        if let Some(existing) = self
            .snapshots
            .iter_mut()
            .find(|snapshot| snapshot.snapshot_id == updated.snapshot_id)
        {
            *existing = updated;
            self.persist_snapshots()
        } else {
            Err(format!("snapshot {} not found", updated.snapshot_id))
        }
    }

    pub fn set_snapshot_registered(&mut self, snapshot_id: &str) -> Result<SnapshotRecord, String> {
        let mut snapshot = self
            .snapshots
            .iter()
            .find(|entry| entry.snapshot_id == snapshot_id)
            .cloned()
            .ok_or_else(|| format!("snapshot {} not found", snapshot_id))?;
        snapshot.restore_registered = true;
        snapshot.state = SnapshotState::Registered;
        self.update_snapshot(snapshot.clone())?;
        Ok(snapshot)
    }

    pub fn set_snapshot_state(
        &mut self,
        snapshot_id: &str,
        state: SnapshotState,
    ) -> Result<SnapshotRecord, String> {
        let mut snapshot = self
            .snapshots
            .iter()
            .find(|entry| entry.snapshot_id == snapshot_id)
            .cloned()
            .ok_or_else(|| format!("snapshot {} not found", snapshot_id))?;
        snapshot.state = state;
        self.update_snapshot(snapshot.clone())?;
        Ok(snapshot)
    }

    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<SnapshotRecord> {
        self.snapshots
            .iter()
            .find(|entry| entry.snapshot_id == snapshot_id)
            .cloned()
    }

    pub fn find_attempt_by_snapshot(&self, snapshot_id: &str) -> Option<UpdateAttempt> {
        self.attempts
            .iter()
            .find(|attempt| attempt.snapshot_id == snapshot_id)
            .cloned()
    }

    pub fn set_update_status_rolled_back(
        &mut self,
        snapshot_id: &str,
        attempt_id: &str,
        rollback_at: &str,
        recovery_result: &str,
    ) -> Result<(), String> {
        self.status.current_state = UpdateState::RolledBack;
        self.status.active_update = None;
        self.status.rollback_reason = None;
        self.status.last_rollback_snapshot_id = Some(snapshot_id.to_string());
        self.status.last_rollback_attempt_id = Some(attempt_id.to_string());
        self.status.last_rollback_at = Some(rollback_at.to_string());
        self.status.last_recovery_result = Some(recovery_result.to_string());
        self.persist_status()
    }

    pub fn set_update_status_failed(
        &mut self,
        rollback_reason: Option<String>,
        snapshot_id: Option<String>,
        attempt_id: Option<String>,
        recovery_result: Option<String>,
    ) -> Result<(), String> {
        self.status.current_state = UpdateState::Failed;
        self.status.rollback_reason = rollback_reason;
        self.status.last_rollback_snapshot_id = snapshot_id;
        self.status.last_rollback_attempt_id = attempt_id;
        self.status.last_rollback_at = None;
        self.status.last_recovery_result = recovery_result;
        self.persist_status()
    }

    pub fn write_consistent_owner_state(&mut self, status: UpdateStatus) -> Result<(), String> {
        self.status = status;
        self.persist_status()
    }

    pub fn write_apply_marker(&self, marker: &AppliedMarker) -> Result<(), String> {
        persist_json(&self.apply_marker_path, marker)
    }

    pub fn read_apply_marker(&self) -> Option<AppliedMarker> {
        fs::read_to_string(&self.apply_marker_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<AppliedMarker>(&raw).ok())
    }

    fn persist_status(&self) -> Result<(), String> {
        persist_json(&self.status_path, &self.status)
    }

    fn persist_snapshots(&self) -> Result<(), String> {
        persist_json(&self.snapshots_path, &self.snapshots)
    }

    fn persist_attempts(&self) -> Result<(), String> {
        persist_json(&self.attempts_path, &self.attempts)
    }
}

fn persist_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(value)
        .map_err(|err| format!("update store serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("update store temp write failed: {err}"))?;
    fs::rename(&tmp, path).map_err(|err| format!("update store rename failed: {err}"))
}
