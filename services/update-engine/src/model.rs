use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateState {
    Idle,
    Checking,
    Ready,
    VerifyingSignature,
    CreatingSnapshot,
    Applying,
    VerifyingPostApply,
    Committed,
    RollbackRequired,
    RolledBack,
    Failed,
}

impl UpdateState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Checking => "checking",
            Self::Ready => "ready",
            Self::VerifyingSignature => "verifying_signature",
            Self::CreatingSnapshot => "creating_snapshot",
            Self::Applying => "applying",
            Self::VerifyingPostApply => "verifying_post_apply",
            Self::Committed => "committed",
            Self::RollbackRequired => "rollback_required",
            Self::RolledBack => "rolled_back",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationState {
    Unknown,
    SignaturePending,
    SignatureOk,
    SignatureFailed,
    PostApplyPending,
    PostApplyOk,
    PostApplyFailed,
}

impl VerificationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::SignaturePending => "signature_pending",
            Self::SignatureOk => "signature_ok",
            Self::SignatureFailed => "signature_failed",
            Self::PostApplyPending => "post_apply_pending",
            Self::PostApplyOk => "post_apply_ok",
            Self::PostApplyFailed => "post_apply_failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SnapshotState {
    Created,
    Registered,
    Consumed,
    RollbackUsed,
    Failed,
}

impl SnapshotState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Registered => "registered",
            Self::Consumed => "consumed",
            Self::RollbackUsed => "rollback_used",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePackage {
    pub update_id: String,
    pub version: String,
    pub channel: String,
    pub description: String,
    pub signed: bool,
    pub payload_kind: String,
    pub simulated_apply_supported: bool,
    pub created_at: String,
}

impl UpdatePackage {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("update_id".to_string(), self.update_id.clone());
        map.insert("version".to_string(), self.version.clone());
        map.insert("channel".to_string(), self.channel.clone());
        map.insert("description".to_string(), self.description.clone());
        map.insert("signed".to_string(), self.signed.to_string());
        map.insert("payload_kind".to_string(), self.payload_kind.clone());
        map.insert(
            "simulated_apply_supported".to_string(),
            self.simulated_apply_supported.to_string(),
        );
        map.insert("created_at".to_string(), self.created_at.clone());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureCheckResult {
    pub valid: bool,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotRecord {
    pub snapshot_id: String,
    pub update_id: String,
    pub attempt_id: String,
    pub kind: String,
    pub created_at: String,
    pub bootable: bool,
    pub restore_registered: bool,
    pub simulated: bool,
    pub state: SnapshotState,
}

impl SnapshotRecord {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("snapshot_id".to_string(), self.snapshot_id.clone());
        map.insert("update_id".to_string(), self.update_id.clone());
        map.insert("attempt_id".to_string(), self.attempt_id.clone());
        map.insert("kind".to_string(), self.kind.clone());
        map.insert("created_at".to_string(), self.created_at.clone());
        map.insert("bootable".to_string(), self.bootable.to_string());
        map.insert(
            "restore_registered".to_string(),
            self.restore_registered.to_string(),
        );
        map.insert("simulated".to_string(), self.simulated.to_string());
        map.insert("state".to_string(), self.state.as_str().to_string());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAttempt {
    pub attempt_id: String,
    pub update_id: String,
    pub snapshot_id: String,
    pub state: UpdateState,
    pub verification_state: VerificationState,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub failure_reason: Option<String>,
    pub rollback_required: bool,
    pub committed: bool,
}

impl UpdateAttempt {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("attempt_id".to_string(), self.attempt_id.clone());
        map.insert("update_id".to_string(), self.update_id.clone());
        map.insert("snapshot_id".to_string(), self.snapshot_id.clone());
        map.insert("state".to_string(), self.state.as_str().to_string());
        map.insert(
            "verification_state".to_string(),
            self.verification_state.as_str().to_string(),
        );
        map.insert("started_at".to_string(), self.started_at.clone());
        map.insert(
            "finished_at".to_string(),
            self.finished_at.clone().unwrap_or_default(),
        );
        map.insert(
            "failure_reason".to_string(),
            self.failure_reason.clone().unwrap_or_default(),
        );
        map.insert(
            "rollback_required".to_string(),
            self.rollback_required.to_string(),
        );
        map.insert("committed".to_string(), self.committed.to_string());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppliedMarker {
    pub update_id: String,
    pub attempt_id: String,
    pub snapshot_id: String,
    pub payload_kind: String,
    pub simulated: bool,
    pub applied_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateStatus {
    pub current_state: UpdateState,
    pub last_attempt_id: Option<String>,
    pub active_update: Option<String>,
    pub rollback_reason: Option<String>,
    pub last_committed_update_id: Option<String>,
    pub last_rollback_snapshot_id: Option<String>,
    pub last_rollback_attempt_id: Option<String>,
    pub last_rollback_at: Option<String>,
    pub last_recovery_result: Option<String>,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self {
            current_state: UpdateState::Idle,
            last_attempt_id: None,
            active_update: None,
            rollback_reason: None,
            last_committed_update_id: None,
            last_rollback_snapshot_id: None,
            last_rollback_attempt_id: None,
            last_rollback_at: None,
            last_recovery_result: None,
        }
    }
}
