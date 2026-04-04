use crate::model::{SnapshotRecord, SnapshotState};
use chrono::Utc;

pub fn create_pre_update_snapshot(update_id: &str, attempt_id: &str) -> SnapshotRecord {
    SnapshotRecord {
        snapshot_id: format!("snapshot-{}-{}", update_id, Utc::now().timestamp_millis()),
        update_id: update_id.to_string(),
        attempt_id: attempt_id.to_string(),
        kind: "pre_update_btrfs".to_string(),
        created_at: Utc::now().to_rfc3339(),
        bootable: true,
        restore_registered: false,
        simulated: true,
        state: SnapshotState::Created,
    }
}
