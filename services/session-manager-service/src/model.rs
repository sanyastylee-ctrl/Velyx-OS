use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Bootstrapping,
    LaunchingCoreServices,
    WaitingForCoreServices,
    HealthChecking,
    StartingShell,
    Ready,
    Degraded,
    Failed,
}

impl SessionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Bootstrapping => "bootstrapping",
            Self::LaunchingCoreServices => "launching_core_services",
            Self::WaitingForCoreServices => "waiting_for_core_services",
            Self::HealthChecking => "health_checking",
            Self::StartingShell => "starting_shell",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionHealthStatus {
    Healthy,
    Degraded,
    Failed,
}

impl SessionHealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
        }
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
pub struct ServiceHealth {
    pub service_name: String,
    pub required: bool,
    pub status: String,
    pub startup_order: u32,
    pub restart_policy: String,
}

impl ServiceHealth {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("service_name".to_string(), self.service_name.clone());
        map.insert("required".to_string(), self.required.to_string());
        map.insert("status".to_string(), self.status.clone());
        map.insert("startup_order".to_string(), self.startup_order.to_string());
        map.insert("restart_policy".to_string(), self.restart_policy.clone());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppRegistryEntry {
    pub app_id: String,
    pub required: bool,
    pub autostart: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceSource {
    System,
    User,
}

impl SpaceSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceStatus {
    Active,
    Inactive,
    Broken,
}

impl SpaceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Broken => "broken",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceRuntimeState {
    Ready,
    Degraded,
    Failed,
}

impl SpaceRuntimeState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpaceRegistryEntry {
    pub space_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub apps: Vec<String>,
    pub autostart_apps: Vec<String>,
    pub required_apps: Vec<String>,
    pub preferred_active_app: Option<String>,
    pub security_mode: String,
    pub permissions_profile: Option<String>,
    pub focus_policy: String,
    pub ui_layout: Option<String>,
    pub status: SpaceStatus,
    pub created_at: String,
    pub updated_at: String,
    pub source: SpaceSource,
}

impl SpaceRegistryEntry {
    pub fn to_map(&self, runtime_state: &str, reason: &str, active: bool) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("space_id".to_string(), self.space_id.clone());
        map.insert("display_name".to_string(), self.display_name.clone());
        map.insert(
            "description".to_string(),
            self.description.clone().unwrap_or_default(),
        );
        map.insert("apps".to_string(), self.apps.join(","));
        map.insert("autostart_apps".to_string(), self.autostart_apps.join(","));
        map.insert("required_apps".to_string(), self.required_apps.join(","));
        map.insert(
            "preferred_active_app".to_string(),
            self.preferred_active_app.clone().unwrap_or_default(),
        );
        map.insert("security_mode".to_string(), self.security_mode.clone());
        map.insert(
            "permissions_profile".to_string(),
            self.permissions_profile.clone().unwrap_or_default(),
        );
        map.insert("focus_policy".to_string(), self.focus_policy.clone());
        map.insert("ui_layout".to_string(), self.ui_layout.clone().unwrap_or_default());
        map.insert("status".to_string(), self.status.as_str().to_string());
        map.insert("source".to_string(), self.source.as_str().to_string());
        map.insert("created_at".to_string(), self.created_at.clone());
        map.insert("updated_at".to_string(), self.updated_at.clone());
        map.insert("runtime_state".to_string(), runtime_state.to_string());
        map.insert("reason".to_string(), reason.to_string());
        map.insert("active".to_string(), active.to_string());
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpaceRuntimeSnapshot {
    pub space_id: String,
    pub display_name: String,
    pub source: String,
    pub status: String,
    pub runtime_state: String,
    pub security_mode: String,
    pub permissions_profile: Option<String>,
    pub focus_policy: String,
    pub preferred_active_app: Option<String>,
    pub apps: Vec<String>,
    pub autostart_apps: Vec<String>,
    pub required_apps: Vec<String>,
    pub active: bool,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpacesRegistryFile {
    pub active_space_id: Option<String>,
    #[serde(default)]
    pub spaces: Vec<SpaceRegistryEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppRuntimeSnapshot {
    pub app_id: String,
    pub required: bool,
    pub autostart: bool,
    pub in_active_space: bool,
    pub state: String,
    pub pid: Option<u32>,
    pub launched_at: Option<String>,
    pub exited_at: Option<String>,
    pub exit_code: Option<i32>,
    pub launch_status: String,
    pub sandbox_id: Option<String>,
    pub failure_reason: Option<String>,
    pub retry_count: u32,
    pub stop_requested: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FirstBootSnapshot {
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

impl Default for FirstBootSnapshot {
    fn default() -> Self {
        Self {
            state: FirstBootState::None,
            install_id: String::new(),
            target_id: String::new(),
            requested_username: String::new(),
            requested_locale: String::new(),
            baseline_settings_applied: false,
            user_created: false,
            handoff_ready: false,
            completed_at: None,
            failed_reason: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBootstrapRecord {
    pub user_id: String,
    pub username: String,
    pub locale: String,
    pub created_via_first_boot: bool,
    pub home_state_initialized: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ShellRuntime {
    pub shell_pid: Option<u32>,
    pub shell_started_at: Option<String>,
    pub shell_state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub current_state: SessionState,
    pub last_successful_start: Option<String>,
    pub last_failed_reason: Option<String>,
    pub degraded_reason: Option<String>,
    pub active_user_id: String,
    pub shell: ShellRuntime,
    pub required_services: Vec<ServiceHealth>,
    pub optional_services: Vec<ServiceHealth>,
    pub apps: Vec<AppRuntimeSnapshot>,
    pub spaces: Vec<SpaceRuntimeSnapshot>,
    pub active_space_id: Option<String>,
    pub active_space_name: Option<String>,
    pub active_space_state: Option<String>,
    pub active_space_security_mode: Option<String>,
    pub active_space_preferred_active_app: Option<String>,
    pub active_space_apps: Vec<String>,
    pub startup_deadline_epoch_ms: Option<u64>,
    pub retry_count: u32,
}

impl Default for SessionSnapshot {
    fn default() -> Self {
        Self {
            current_state: SessionState::Idle,
            last_successful_start: None,
            last_failed_reason: None,
            degraded_reason: None,
            active_user_id: "user".to_string(),
            shell: ShellRuntime {
                shell_state: "stopped".to_string(),
                ..ShellRuntime::default()
            },
            required_services: Vec::new(),
            optional_services: Vec::new(),
            apps: Vec::new(),
            spaces: Vec::new(),
            active_space_id: None,
            active_space_name: None,
            active_space_state: None,
            active_space_security_mode: None,
            active_space_preferred_active_app: None,
            active_space_apps: Vec::new(),
            startup_deadline_epoch_ms: None,
            retry_count: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StartupOutcome {
    pub state: SessionState,
    pub health: SessionHealthStatus,
    pub required_services: Vec<ServiceHealth>,
    pub optional_services: Vec<ServiceHealth>,
    pub shell: ShellRuntime,
    pub degraded_reason: Option<String>,
    pub failed_reason: Option<String>,
}
