use crate::apply::simulate_apply;
use crate::audit::UpdateAuditLogger;
use crate::errors::UpdateEngineError;
use crate::model::{SnapshotState, UpdateAttempt, UpdateState, UpdateStatus, VerificationState};
use crate::snapshot::create_pre_update_snapshot;
use crate::store::UpdateStore;
use crate::verification::{verify_post_apply, verify_signature};
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::MutexGuard;

pub fn available_updates(store: &mut UpdateStore) -> Result<Vec<HashMap<String, String>>, UpdateEngineError> {
    store
        .set_state(UpdateState::Checking)
        .map_err(UpdateEngineError::Store)?;
    let packages = UpdateStore::get_package_registry();
    store
        .set_state(UpdateState::Ready)
        .map_err(UpdateEngineError::Store)?;
    Ok(packages.into_iter().map(|package| package.to_map()).collect())
}

pub async fn apply_update(
    store: &mut MutexGuard<'_, UpdateStore>,
    audit: &UpdateAuditLogger,
    base_dir: &Path,
    update_id: &str,
) -> Result<HashMap<String, String>, UpdateEngineError> {
    let package = UpdateStore::get_package_registry()
        .into_iter()
        .find(|entry| entry.update_id == update_id)
        .ok_or_else(|| UpdateEngineError::UnknownUpdate(format!("unknown update_id {}", update_id)))?;

    let attempt_id = format!("attempt-{}-{}", update_id, Utc::now().timestamp_millis());
    let mut attempt = UpdateAttempt {
        attempt_id: attempt_id.clone(),
        update_id: package.update_id.clone(),
        snapshot_id: String::new(),
        state: UpdateState::VerifyingSignature,
        verification_state: VerificationState::SignaturePending,
        started_at: Utc::now().to_rfc3339(),
        finished_at: None,
        failure_reason: None,
        rollback_required: false,
        committed: false,
    };

    store.set_state(UpdateState::VerifyingSignature).map_err(UpdateEngineError::Store)?;
    store
        .set_active_update(Some(package.update_id.clone()))
        .map_err(UpdateEngineError::Store)?;
    store.add_attempt(attempt.clone()).map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "apply_begin",
        "started",
        &format!("update_id={} attempt_id={}", package.update_id, attempt.attempt_id),
    );

    let signature = verify_signature(&package).map_err(UpdateEngineError::VerificationFailed)?;
    if !signature.valid {
        attempt = store
            .set_attempt_state(
                &attempt.attempt_id,
                UpdateState::Failed,
                Some(VerificationState::SignatureFailed),
                Some(signature.reason.clone()),
                Some(false),
                Some(false),
                Some(Some(Utc::now().to_rfc3339())),
            )
            .map_err(UpdateEngineError::Store)?;
        store.set_state(UpdateState::Failed).map_err(UpdateEngineError::Store)?;
        store
            .set_rollback_reason(Some(signature.reason.clone()))
            .map_err(UpdateEngineError::Store)?;
        let _ = audit.log(
            "signature_verification_failed",
            "failed",
            &format!("update_id={} attempt_id={} reason={}", package.update_id, attempt.attempt_id, signature.reason),
        );
        return Err(UpdateEngineError::SignatureDenied(signature.reason));
    }
    attempt = store
        .set_attempt_state(
            &attempt.attempt_id,
            UpdateState::VerifyingSignature,
            Some(VerificationState::SignatureOk),
            None,
            None,
            None,
            None,
        )
        .map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "signature_verification_ok",
        "ok",
        &format!("update_id={} attempt_id={}", package.update_id, attempt.attempt_id),
    );

    store.set_state(UpdateState::CreatingSnapshot).map_err(UpdateEngineError::Store)?;
    let snapshot = create_pre_update_snapshot(&package.update_id, &attempt.attempt_id);
    store.add_snapshot(snapshot.clone()).map_err(UpdateEngineError::Store)?;
    attempt.snapshot_id = snapshot.snapshot_id.clone();
    attempt.state = UpdateState::CreatingSnapshot;
    store.update_attempt(attempt.clone()).map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "snapshot_created",
        "ok",
        &format!(
            "update_id={} attempt_id={} snapshot_id={}",
            package.update_id, attempt.attempt_id, snapshot.snapshot_id
        ),
    );

    if let Err(err) = register_restore_point(&snapshot.snapshot_id, &package.update_id, &attempt.attempt_id).await {
        let _ = store.set_state(UpdateState::Failed);
        let _ = store.set_rollback_reason(Some(err.clone()));
        let _ = store.set_attempt_state(
            &attempt.attempt_id,
            UpdateState::Failed,
            Some(VerificationState::SignatureOk),
            Some(err.clone()),
            Some(false),
            Some(false),
            Some(Some(Utc::now().to_rfc3339())),
        );
        let _ = audit.log(
            "update_failed",
            "failed",
            &format!("update_id={} attempt_id={} reason={}", package.update_id, attempt.attempt_id, err),
        );
        return Err(UpdateEngineError::RecoveryRegistration(err));
    }
    let snapshot = store
        .set_snapshot_registered(&snapshot.snapshot_id)
        .map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "restore_point_registered",
        "ok",
        &format!(
            "update_id={} attempt_id={} snapshot_id={}",
            package.update_id, attempt.attempt_id, snapshot.snapshot_id
        ),
    );

    store.set_state(UpdateState::Applying).map_err(UpdateEngineError::Store)?;
    attempt = store
        .set_attempt_state(
            &attempt.attempt_id,
            UpdateState::Applying,
            Some(VerificationState::PostApplyPending),
            None,
            None,
            None,
            None,
        )
        .map_err(UpdateEngineError::Store)?;
    let marker = match simulate_apply(base_dir, &package, &attempt) {
        Ok(marker) => marker,
        Err(err) => {
            let _ = store.set_state(UpdateState::Failed);
            let _ = store.set_rollback_reason(Some(err.clone()));
            let _ = store.set_attempt_state(
                &attempt.attempt_id,
                UpdateState::Failed,
                Some(VerificationState::PostApplyPending),
                Some(err.clone()),
                Some(false),
                Some(false),
                Some(Some(Utc::now().to_rfc3339())),
            );
            let _ = audit.log(
                "update_failed",
                "failed",
                &format!("update_id={} attempt_id={} reason={}", package.update_id, attempt.attempt_id, err),
            );
            return Err(UpdateEngineError::ApplyFailed(err));
        }
    };
    store.write_apply_marker(&marker).map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "apply_simulated_ok",
        "ok",
        &format!(
            "update_id={} attempt_id={} snapshot_id={}",
            package.update_id, attempt.attempt_id, attempt.snapshot_id
        ),
    );

    store
        .set_state(UpdateState::VerifyingPostApply)
        .map_err(UpdateEngineError::Store)?;
    let verification = verify_post_apply(&package, &attempt, store.read_apply_marker())
        .map_err(UpdateEngineError::VerificationFailed)?;

    let mut payload = HashMap::new();
    payload.insert("update_id".to_string(), package.update_id.clone());
    payload.insert("attempt_id".to_string(), attempt.attempt_id.clone());
    payload.insert("snapshot_id".to_string(), attempt.snapshot_id.clone());

    if verification.valid {
        let attempt = store
            .set_attempt_state(
                &attempt.attempt_id,
                UpdateState::Committed,
                Some(VerificationState::PostApplyOk),
                None,
                Some(false),
                Some(true),
                Some(Some(Utc::now().to_rfc3339())),
            )
            .map_err(UpdateEngineError::Store)?;
        store
            .set_snapshot_state(&attempt.snapshot_id, SnapshotState::Consumed)
            .map_err(UpdateEngineError::Store)?;
        store.set_state(UpdateState::Committed).map_err(UpdateEngineError::Store)?;
        store
            .set_last_committed_update_id(Some(package.update_id.clone()))
            .map_err(UpdateEngineError::Store)?;
        store.set_rollback_reason(None).map_err(UpdateEngineError::Store)?;
        let _ = audit.log(
            "post_apply_verification_ok",
            "ok",
            &format!("update_id={} attempt_id={}", package.update_id, attempt.attempt_id),
        );
        let _ = audit.log(
            "update_committed",
            "ok",
            &format!(
                "update_id={} attempt_id={} snapshot_id={}",
                package.update_id, attempt.attempt_id, attempt.snapshot_id
            ),
        );
        payload.insert("state".to_string(), UpdateState::Committed.as_str().to_string());
        payload.insert(
            "verification_state".to_string(),
            VerificationState::PostApplyOk.as_str().to_string(),
        );
        payload.insert("rollback_required".to_string(), "false".to_string());
        payload.insert("failure_reason".to_string(), String::new());
        payload.insert("committed".to_string(), "true".to_string());
        payload.insert("restore_registered".to_string(), snapshot.restore_registered.to_string());
        return Ok(payload);
    }

    let attempt = store
        .set_attempt_state(
            &attempt.attempt_id,
            UpdateState::RollbackRequired,
            Some(VerificationState::PostApplyFailed),
            Some(verification.reason.clone()),
            Some(true),
            Some(false),
            Some(Some(Utc::now().to_rfc3339())),
        )
        .map_err(UpdateEngineError::Store)?;
    store
        .set_state(UpdateState::RollbackRequired)
        .map_err(UpdateEngineError::Store)?;
    store
        .set_rollback_reason(Some(verification.reason.clone()))
        .map_err(UpdateEngineError::Store)?;
    let _ = audit.log(
        "post_apply_verification_failed",
        "failed",
        &format!("update_id={} attempt_id={} reason={}", package.update_id, attempt.attempt_id, verification.reason),
    );
    let _ = audit.log(
        "rollback_required",
        "required",
        &format!(
            "update_id={} attempt_id={} snapshot_id={}",
            package.update_id, attempt.attempt_id, attempt.snapshot_id
        ),
    );
    payload.insert("state".to_string(), UpdateState::RollbackRequired.as_str().to_string());
    payload.insert(
        "verification_state".to_string(),
        VerificationState::PostApplyFailed.as_str().to_string(),
    );
    payload.insert("rollback_required".to_string(), "true".to_string());
    payload.insert("failure_reason".to_string(), verification.reason);
    payload.insert("committed".to_string(), "false".to_string());
    payload.insert("restore_registered".to_string(), snapshot.restore_registered.to_string());
    Ok(payload)
}

pub fn status_payload(store: &UpdateStore) -> HashMap<String, String> {
    let status: UpdateStatus = store.status();
    let mut payload = HashMap::new();
    payload.insert("state".to_string(), status.current_state.as_str().to_string());
    payload.insert(
        "current_state".to_string(),
        status.current_state.as_str().to_string(),
    );
    payload.insert(
        "update_id".to_string(),
        status.active_update.clone().unwrap_or_default(),
    );
    payload.insert(
        "active_update".to_string(),
        status.active_update.clone().unwrap_or_default(),
    );
    payload.insert(
        "attempt_id".to_string(),
        status.last_attempt_id.clone().unwrap_or_default(),
    );
    payload.insert(
        "rollback_reason".to_string(),
        status.rollback_reason.unwrap_or_default(),
    );
    payload.insert(
        "last_committed_update_id".to_string(),
        status.last_committed_update_id.unwrap_or_default(),
    );
    payload.insert(
        "last_rollback_snapshot_id".to_string(),
        status.last_rollback_snapshot_id.unwrap_or_default(),
    );
    payload.insert(
        "last_rollback_attempt_id".to_string(),
        status.last_rollback_attempt_id.unwrap_or_default(),
    );
    payload.insert(
        "last_rollback_at".to_string(),
        status.last_rollback_at.unwrap_or_default(),
    );
    payload.insert(
        "last_recovery_result".to_string(),
        status.last_recovery_result.unwrap_or_default(),
    );
    if let Some(attempt_id) = status.last_attempt_id {
        if let Some(attempt) = store.get_attempt(&attempt_id) {
            payload.insert("snapshot_id".to_string(), attempt.snapshot_id.clone());
            payload.insert(
                "verification_state".to_string(),
                attempt.verification_state.as_str().to_string(),
            );
            payload.insert(
                "rollback_required".to_string(),
                attempt.rollback_required.to_string(),
            );
            payload.insert("committed".to_string(), attempt.committed.to_string());
            payload.insert(
                "failure_reason".to_string(),
                attempt.failure_reason.unwrap_or_default(),
            );
        }
    }
    payload
}

pub fn on_rollback_completed(
    store: &mut UpdateStore,
    audit: &UpdateAuditLogger,
    snapshot_id: &str,
    result: &str,
) -> Result<bool, UpdateEngineError> {
    let attempt = store
        .find_attempt_by_snapshot(snapshot_id)
        .ok_or_else(|| UpdateEngineError::UnknownUpdate(format!("unknown rollback snapshot_id {}", snapshot_id)))?;
    let _ = audit.log(
        "rollback_callback_received",
        "ok",
        &format!(
            "snapshot_id={} attempt_id={} result={}",
            snapshot_id, attempt.attempt_id, result
        ),
    );

    match result {
        "success" => {
            let rollback_at = Utc::now().to_rfc3339();
            let updated_attempt = store
                .set_attempt_rolled_back(&attempt.attempt_id, rollback_at.clone())
                .map_err(UpdateEngineError::Store)?;
            store
                .set_snapshot_state(snapshot_id, SnapshotState::RollbackUsed)
                .map_err(UpdateEngineError::Store)?;
            store
                .set_update_status_rolled_back(
                    snapshot_id,
                    &updated_attempt.attempt_id,
                    &rollback_at,
                    "rollback_completed",
                )
                .map_err(UpdateEngineError::Store)?;
            let _ = audit.log(
                "rollback_applied_to_attempt",
                "ok",
                &format!(
                    "snapshot_id={} attempt_id={} state={}",
                    snapshot_id,
                    updated_attempt.attempt_id,
                    UpdateState::RolledBack.as_str()
                ),
            );
            Ok(true)
        }
        "failed" => {
            let updated_attempt = store
                .set_attempt_state(
                    &attempt.attempt_id,
                    UpdateState::Failed,
                    Some(VerificationState::PostApplyFailed),
                    Some("rollback execution failed".to_string()),
                    Some(true),
                    Some(false),
                    Some(Some(Utc::now().to_rfc3339())),
                )
                .map_err(UpdateEngineError::Store)?;
            store
                .set_update_status_failed(
                    Some("rollback execution failed".to_string()),
                    Some(snapshot_id.to_string()),
                    Some(updated_attempt.attempt_id.clone()),
                    Some("rollback_failed".to_string()),
                )
                .map_err(UpdateEngineError::Store)?;
            let _ = audit.log(
                "rollback_callback_failed",
                "failed",
                &format!(
                    "snapshot_id={} attempt_id={} result=failed",
                    snapshot_id, updated_attempt.attempt_id
                ),
            );
            Ok(true)
        }
        _ => Err(UpdateEngineError::VerificationFailed(format!(
            "unknown rollback callback result {}",
            result
        ))),
    }
}

async fn register_restore_point(
    snapshot_id: &str,
    update_id: &str,
    attempt_id: &str,
) -> Result<(), String> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|err| format!("recovery bus unavailable: {err}"))?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Recovery",
        "/com/velyx/Recovery",
        "com.velyx.Recovery1",
    )
    .await
    .map_err(|err| format!("recovery proxy init failed: {err}"))?;
    let result: bool = proxy
        .call(
            "RegisterRestorePoint",
            &(snapshot_id, update_id, format!("pre_update attempt_id={attempt_id}")),
        )
        .await
        .map_err(|err| format!("restore point registration failed: {err}"))?;
    if result {
        Ok(())
    } else {
        Err("recovery returned false during restore point registration".to_string())
    }
}
