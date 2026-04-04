use crate::audit::RecoveryAuditLogger;
use crate::errors::RecoveryError;
use crate::model::{RecoveryState, RestorePoint};
use crate::rollback::{complete_restore_point, validate_snapshot_exists};
use crate::store::RecoveryStore;
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::MutexGuard;

pub fn register_restore_point(
    store: &mut RecoveryStore,
    snapshot_id: &str,
    update_id: &str,
    reason: &str,
) -> Result<bool, RecoveryError> {
    let attempt_id = reason
        .split("attempt_id=")
        .nth(1)
        .unwrap_or("unknown-attempt")
        .to_string();
    let point = RestorePoint {
        snapshot_id: snapshot_id.to_string(),
        update_id: update_id.to_string(),
        attempt_id,
        kind: "btrfs-pre-update".to_string(),
        created_at: Utc::now().to_rfc3339(),
        bootable: true,
        reason: reason.to_string(),
        rollback_state: "available".to_string(),
    };
    store.add_restore_point(point).map_err(RecoveryError::Store)?;
    Ok(true)
}

pub fn list_restore_points(store: &mut RecoveryStore) -> Result<Vec<HashMap<String, String>>, RecoveryError> {
    store
        .set_state(RecoveryState::ListingRestorePoints)
        .map_err(RecoveryError::Store)?;
    let points = store.restore_points();
    store.set_state(RecoveryState::Idle).map_err(RecoveryError::Store)?;
    Ok(points.into_iter().map(|point| point.to_map()).collect())
}

pub fn status_payload(store: &RecoveryStore) -> HashMap<String, String> {
    let status = store.status();
    let mut payload = HashMap::new();
    payload.insert("state".to_string(), status.current_state.as_str().to_string());
    payload.insert(
        "current_state".to_string(),
        status.current_state.as_str().to_string(),
    );
    payload.insert(
        "snapshot_id".to_string(),
        status.last_snapshot_id.unwrap_or_default(),
    );
    payload.insert("update_id".to_string(), status.last_update_id.unwrap_or_default());
    payload.insert("attempt_id".to_string(), status.last_attempt_id.unwrap_or_default());
    payload.insert("rollback_state".to_string(), status.last_result);
    payload.insert(
        "recovery_mode_available".to_string(),
        status.recovery_mode_available.to_string(),
    );
    payload
}

pub fn rollback_snapshot(
    store: &mut MutexGuard<'_, RecoveryStore>,
    audit: &RecoveryAuditLogger,
    base_dir: &Path,
    snapshot_id: &str,
) -> Result<HashMap<String, String>, RecoveryError> {
    store
        .set_state(RecoveryState::RollbackPending)
        .map_err(RecoveryError::Store)?;
    let mut point = store
        .get_restore_point(snapshot_id)
        .ok_or_else(|| RecoveryError::UnknownSnapshot(format!("unknown snapshot_id {}", snapshot_id)))?;

    validate_snapshot_exists(base_dir, snapshot_id)?;
    store
        .set_last_result(
            Some(point.snapshot_id.clone()),
            Some(point.update_id.clone()),
            Some(point.attempt_id.clone()),
            "rollback_pending".to_string(),
        )
        .map_err(RecoveryError::Store)?;
    let _ = audit.log(
        "rollback_begin",
        "started",
        &format!(
            "snapshot_id={} update_id={} attempt_id={}",
            point.snapshot_id, point.update_id, point.attempt_id
        ),
    );

    store
        .set_state(RecoveryState::RollbackInProgress)
        .map_err(RecoveryError::Store)?;
    complete_restore_point(&mut point);
    store.update_restore_point(point.clone()).map_err(RecoveryError::Store)?;
    store
        .set_last_result(
            Some(point.snapshot_id.clone()),
            Some(point.update_id.clone()),
            Some(point.attempt_id.clone()),
            "rollback_completed".to_string(),
        )
        .map_err(RecoveryError::Store)?;
    store
        .set_state(RecoveryState::RollbackCompleted)
        .map_err(RecoveryError::Store)?;
    let _ = audit.log(
        "rollback_completed",
        "ok",
        &format!(
            "snapshot_id={} update_id={} attempt_id={}",
            point.snapshot_id, point.update_id, point.attempt_id
        ),
    );

    let mut payload = HashMap::new();
    payload.insert("snapshot_id".to_string(), point.snapshot_id.clone());
    payload.insert("update_id".to_string(), point.update_id.clone());
    payload.insert("attempt_id".to_string(), point.attempt_id.clone());
    payload.insert(
        "state".to_string(),
        RecoveryState::RollbackCompleted.as_str().to_string(),
    );
    payload.insert("rollback_state".to_string(), point.rollback_state.clone());
    Ok(payload)
}

pub async fn notify_update_engine_rollback_completed(
    audit: &RecoveryAuditLogger,
    snapshot_id: &str,
    result: &str,
) -> Result<(), RecoveryError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|err| RecoveryError::Store(format!("update-engine bus unavailable: {err}")))?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.UpdateEngine",
        "/com/velyx/UpdateEngine",
        "com.velyx.UpdateEngine1",
    )
    .await
    .map_err(|err| RecoveryError::Store(format!("update-engine proxy init failed: {err}")))?;
    let applied: bool = proxy
        .call("OnRollbackCompleted", &(snapshot_id, result))
        .await
        .map_err(|err| RecoveryError::Store(format!("rollback callback failed: {err}")))?;
    if applied {
        let _ = audit.log(
            "rollback_callback_sent",
            "ok",
            &format!("snapshot_id={} result={}", snapshot_id, result),
        );
        Ok(())
    } else {
        let _ = audit.log(
            "rollback_callback_send_failed",
            "failed",
            &format!("snapshot_id={} result={}", snapshot_id, result),
        );
        Err(RecoveryError::Store(
            "update-engine rejected rollback callback".to_string(),
        ))
    }
}
