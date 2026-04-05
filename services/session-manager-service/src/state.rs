use crate::model::{AppRuntimeSnapshot, ServiceHealth, SessionSnapshot, SessionState, ShellRuntime};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SessionStateStore {
    path: PathBuf,
    snapshot: SessionSnapshot,
}

impl SessionStateStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let path = base_dir.join("session_state.json");
        let snapshot = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<SessionSnapshot>(&raw).ok())
            .unwrap_or_default();
        Self { path, snapshot }
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        self.snapshot.clone()
    }

    pub fn update_state(&mut self, state: SessionState) -> Result<(), String> {
        self.snapshot.current_state = state;
        self.persist()
    }

    pub fn set_active_user(&mut self, user_id: &str) -> Result<(), String> {
        self.snapshot.active_user_id = user_id.to_string();
        self.persist()
    }

    pub fn set_startup_deadline_epoch_ms(&mut self, deadline: Option<u64>) -> Result<(), String> {
        self.snapshot.startup_deadline_epoch_ms = deadline;
        self.persist()
    }

    pub fn set_service_snapshots(
        &mut self,
        required: Vec<ServiceHealth>,
        optional: Vec<ServiceHealth>,
    ) -> Result<(), String> {
        self.snapshot.required_services = required;
        self.snapshot.optional_services = optional;
        self.persist()
    }

    pub fn set_app_snapshots(&mut self, apps: Vec<AppRuntimeSnapshot>) -> Result<(), String> {
        self.snapshot.apps = apps;
        self.persist()
    }

    pub fn set_shell_runtime(&mut self, shell: ShellRuntime) -> Result<(), String> {
        self.snapshot.shell = shell;
        self.persist()
    }

    pub fn mark_ready(&mut self) -> Result<(), String> {
        self.snapshot.last_successful_start = Some(Utc::now().to_rfc3339());
        self.snapshot.last_failed_reason = None;
        self.snapshot.degraded_reason = None;
        self.persist()
    }

    pub fn mark_degraded(&mut self, reason: String) -> Result<(), String> {
        self.snapshot.degraded_reason = Some(reason);
        self.snapshot.last_failed_reason = None;
        self.persist()
    }

    pub fn mark_failed(&mut self, reason: String) -> Result<(), String> {
        self.snapshot.last_failed_reason = Some(reason);
        self.persist()
    }

    pub fn clear_runtime(&mut self) -> Result<(), String> {
        self.snapshot.current_state = SessionState::Idle;
        self.snapshot.degraded_reason = None;
        self.snapshot.shell = ShellRuntime {
            shell_state: "stopped".to_string(),
            ..ShellRuntime::default()
        };
        self.snapshot.startup_deadline_epoch_ms = None;
        self.snapshot.required_services.clear();
        self.snapshot.optional_services.clear();
        self.snapshot.apps.clear();
        self.persist()
    }

    pub fn increment_retry(&mut self) -> Result<(), String> {
        self.snapshot.retry_count += 1;
        self.persist()
    }

    fn persist(&self) -> Result<(), String> {
        let tmp = self.path.with_extension("json.tmp");
        let raw = serde_json::to_string_pretty(&self.snapshot)
            .map_err(|err| format!("session state serialization failed: {err}"))?;
        fs::write(&tmp, raw).map_err(|err| format!("session state temp write failed: {err}"))?;
        fs::rename(&tmp, &self.path)
            .map_err(|err| format!("session state rename failed: {err}"))
    }
}
