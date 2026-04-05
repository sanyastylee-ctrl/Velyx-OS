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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppRuntimeSnapshot {
    pub app_id: String,
    pub required: bool,
    pub autostart: bool,
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
