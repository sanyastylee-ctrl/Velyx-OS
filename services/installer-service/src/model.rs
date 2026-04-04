use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiskTarget {
    pub target_id: String,
    pub device_path: String,
    pub capacity_gb: u64,
    pub scheme: String,
    pub supports_encryption: bool,
    pub supports_rollback_layout: bool,
}

impl DiskTarget {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("target_id".to_string(), self.target_id.clone());
        map.insert("device_path".to_string(), self.device_path.clone());
        map.insert("capacity_gb".to_string(), self.capacity_gb.to_string());
        map.insert("scheme".to_string(), self.scheme.clone());
        map.insert(
            "supports_encryption".to_string(),
            self.supports_encryption.to_string(),
        );
        map.insert(
            "supports_rollback_layout".to_string(),
            self.supports_rollback_layout.to_string(),
        );
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallProfile {
    pub profile_id: String,
    pub display_name: String,
    pub description: String,
    pub gaming_ready: bool,
    pub developer_ready: bool,
    pub baseline_ai_mode: String,
}

impl InstallProfile {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("profile_id".to_string(), self.profile_id.clone());
        map.insert("display_name".to_string(), self.display_name.clone());
        map.insert("description".to_string(), self.description.clone());
        map.insert("gaming_ready".to_string(), self.gaming_ready.to_string());
        map.insert("developer_ready".to_string(), self.developer_ready.to_string());
        map.insert("baseline_ai_mode".to_string(), self.baseline_ai_mode.clone());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FirstBootState {
    None,
    Pending,
    InitialSetupStarted,
    UserCreationPending,
    BaselineConfigPending,
    ServiceBootstrapPending,
    HandoffToSessionPending,
    Completed,
    Failed,
}

impl FirstBootState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::InitialSetupStarted => "initial_setup_started",
            Self::UserCreationPending => "user_creation_pending",
            Self::BaselineConfigPending => "baseline_config_pending",
            Self::ServiceBootstrapPending => "service_bootstrap_pending",
            Self::HandoffToSessionPending => "handoff_to_session_pending",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallPlan {
    pub target_id: String,
    pub profile_id: String,
    pub encryption_enabled: bool,
    pub username: String,
    pub locale: String,
    pub bootloader_target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallerState {
    pub current_state: FirstBootState,
    pub prepared_plan: Option<InstallPlan>,
    pub post_install_config_written: bool,
    pub recovery_hook_registered: bool,
}

impl Default for InstallerState {
    fn default() -> Self {
        Self {
            current_state: FirstBootState::None,
            prepared_plan: None,
            post_install_config_written: false,
            recovery_hook_registered: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallHandoffState {
    pub install_id: String,
    pub target_id: String,
    pub profile_id: String,
    pub encryption_enabled: bool,
    pub requested_username: String,
    pub requested_locale: String,
    pub first_boot_pending: bool,
    pub baseline_settings_pending: bool,
    pub session_start_pending: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FirstBootMarker {
    pub state: FirstBootState,
    pub install_id: String,
    pub target_id: String,
    pub requested_username: String,
    pub requested_locale: String,
    pub baseline_settings_applied: bool,
    pub user_created: bool,
    pub handoff_ready: bool,
    pub completed_at: Option<String>,
    pub failed_reason: Option<String>,
}

impl FirstBootMarker {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("state".to_string(), self.state.as_str().to_string());
        map.insert("install_id".to_string(), self.install_id.clone());
        map.insert("target_id".to_string(), self.target_id.clone());
        map.insert("requested_username".to_string(), self.requested_username.clone());
        map.insert("requested_locale".to_string(), self.requested_locale.clone());
        map.insert(
            "baseline_settings_applied".to_string(),
            self.baseline_settings_applied.to_string(),
        );
        map.insert("user_created".to_string(), self.user_created.to_string());
        map.insert("handoff_ready".to_string(), self.handoff_ready.to_string());
        map.insert(
            "completed_at".to_string(),
            self.completed_at.clone().unwrap_or_default(),
        );
        map.insert(
            "failed_reason".to_string(),
            self.failed_reason.clone().unwrap_or_default(),
        );
        map
    }
}
