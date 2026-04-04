use crate::errors::RecoveryError;
use crate::model::RestorePoint;
use std::fs;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
struct UpdateSnapshotRecord {
    snapshot_id: String,
    update_id: String,
    attempt_id: String,
    kind: String,
    created_at: String,
    bootable: bool,
    restore_registered: bool,
    simulated: bool,
    state: String,
}

pub fn validate_snapshot_exists(base_dir: &Path, snapshot_id: &str) -> Result<(), RecoveryError> {
    let path = base_dir.join("update_snapshots.json");
    let snapshots = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Vec<UpdateSnapshotRecord>>(&raw).ok())
        .unwrap_or_default();
    snapshots
        .into_iter()
        .find(|snapshot| snapshot.snapshot_id == snapshot_id)
        .map(|_| ())
        .ok_or_else(|| RecoveryError::SnapshotLinkage(format!("snapshot {} not found in update store", snapshot_id)))
}

pub fn complete_restore_point(point: &mut RestorePoint) {
    point.rollback_state = "rollback_completed".to_string();
}
