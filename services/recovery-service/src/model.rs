use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RecoveryState {
    Idle,
    ListingRestorePoints,
    RollbackPending,
    RollbackInProgress,
    RollbackCompleted,
    RecoveryModeReady,
    Failed,
}

impl RecoveryState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::ListingRestorePoints => "listing_restore_points",
            Self::RollbackPending => "rollback_pending",
            Self::RollbackInProgress => "rollback_in_progress",
            Self::RollbackCompleted => "rollback_completed",
            Self::RecoveryModeReady => "recovery_mode_ready",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestorePoint {
    pub snapshot_id: String,
    pub update_id: String,
    pub attempt_id: String,
    pub kind: String,
    pub created_at: String,
    pub bootable: bool,
    pub reason: String,
    pub rollback_state: String,
}

impl RestorePoint {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("snapshot_id".to_string(), self.snapshot_id.clone());
        map.insert("update_id".to_string(), self.update_id.clone());
        map.insert("attempt_id".to_string(), self.attempt_id.clone());
        map.insert("kind".to_string(), self.kind.clone());
        map.insert("created_at".to_string(), self.created_at.clone());
        map.insert("bootable".to_string(), self.bootable.to_string());
        map.insert("reason".to_string(), self.reason.clone());
        map.insert("rollback_state".to_string(), self.rollback_state.clone());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryStatus {
    pub current_state: RecoveryState,
    pub last_snapshot_id: Option<String>,
    pub last_update_id: Option<String>,
    pub last_attempt_id: Option<String>,
    pub last_result: String,
    pub recovery_mode_available: bool,
}

impl Default for RecoveryStatus {
    fn default() -> Self {
        Self {
            current_state: RecoveryState::Idle,
            last_snapshot_id: None,
            last_update_id: None,
            last_attempt_id: None,
            last_result: "none".to_string(),
            recovery_mode_available: true,
        }
    }
}
