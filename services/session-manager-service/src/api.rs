use crate::audit::SessionAuditLogger;
use crate::errors::SessionManagerError;
use crate::first_boot::{has_pending_first_boot, run_first_boot_flow};
use crate::health::{check_optional_services, check_required_services, compute_session_health};
use crate::model::{SessionHealthStatus, SessionSnapshot, SessionState};
use crate::shell_launch::build_shell_runtime;
use crate::startup::bootstrap_session;
use crate::state::SessionStateStore;
use crate::units::{is_active, restart_unit, sorted_runtime_units, stop_target};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SessionManagerApi {
    state: Arc<Mutex<SessionStateStore>>,
    audit: SessionAuditLogger,
    base_dir: PathBuf,
}

impl SessionManagerApi {
    pub fn new(state: SessionStateStore, audit: SessionAuditLogger, base_dir: PathBuf) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
            audit,
            base_dir,
        }
    }

    fn status_payload(snapshot: &SessionSnapshot) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        let (computed_health, _, _) =
            compute_session_health(&snapshot.required_services, &snapshot.optional_services);
        let health = match snapshot.current_state {
            SessionState::Failed => SessionHealthStatus::Failed,
            SessionState::Degraded => SessionHealthStatus::Degraded,
            SessionState::Ready if snapshot.shell.shell_state == "running" => computed_health,
            SessionState::Ready => SessionHealthStatus::Failed,
            _ => computed_health,
        };
        payload.insert("state".to_string(), snapshot.current_state.as_str().to_string());
        payload.insert("health".to_string(), health.as_str().to_string());
        payload.insert("active_user_id".to_string(), snapshot.active_user_id.clone());
        payload.insert(
            "last_successful_start".to_string(),
            snapshot.last_successful_start.clone().unwrap_or_default(),
        );
        payload.insert(
            "last_failed_reason".to_string(),
            snapshot.last_failed_reason.clone().unwrap_or_default(),
        );
        payload.insert(
            "degraded_reason".to_string(),
            snapshot.degraded_reason.clone().unwrap_or_default(),
        );
        payload.insert(
            "shell_status".to_string(),
            snapshot.shell.shell_state.clone(),
        );
        payload.insert(
            "shell_pid".to_string(),
            snapshot
                .shell
                .shell_pid
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );
        payload.insert(
            "required_services_status".to_string(),
            snapshot
                .required_services
                .iter()
                .map(|service| format!("{}={}", service.service_name, service.status))
                .collect::<Vec<_>>()
                .join(","),
        );
        payload.insert(
            "optional_services_status".to_string(),
            snapshot
                .optional_services
                .iter()
                .map(|service| format!("{}={}", service.service_name, service.status))
                .collect::<Vec<_>>()
                .join(","),
        );
        payload
    }
}

#[zbus::interface(name = "com.velyx.SessionManager1")]
impl SessionManagerApi {
    async fn start_session(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let user_id = {
            let state = self.state.lock().await;
            state.snapshot().active_user_id
        };
        self.start_user_session(&user_id).await
    }

    async fn start_user_session(&self, user_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut state = self.state.lock().await;
        let outcome = if has_pending_first_boot(&self.base_dir) {
            run_first_boot_flow(&self.base_dir, &mut state, &self.audit, user_id)
                .await
                .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?
        } else {
            bootstrap_session(&mut state, &self.audit, user_id, "manual_start")
                .await
                .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?
        };
        let snapshot = state.snapshot();
        let mut payload = Self::status_payload(&snapshot);
        payload.insert("health".to_string(), outcome.health.as_str().to_string());
        payload.insert(
            "session_target_status".to_string(),
            is_active("velyx-session.target")
                .await
                .unwrap_or_else(|_| "unknown".to_string()),
        );
        Ok(payload)
    }

    async fn get_session_status(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut state = self.state.lock().await;
        if let Ok(shell) = build_shell_runtime().await {
            let _ = state.set_shell_runtime(shell);
        }
        let mut payload = Self::status_payload(&state.snapshot());
        payload.insert(
            "session_target_status".to_string(),
            is_active("velyx-session.target")
                .await
                .unwrap_or_else(|_| "unknown".to_string()),
        );
        Ok(payload)
    }

    async fn get_session_health(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut state = self.state.lock().await;
        if let Ok(shell) = build_shell_runtime().await {
            let _ = state.set_shell_runtime(shell);
        }
        let snapshot = state.snapshot();
        let (computed_health, degraded_reason, failed_reason) =
            compute_session_health(&snapshot.required_services, &snapshot.optional_services);
        let health = match snapshot.current_state {
            SessionState::Failed => SessionHealthStatus::Failed,
            SessionState::Degraded => SessionHealthStatus::Degraded,
            SessionState::Ready if snapshot.shell.shell_state == "running" => computed_health,
            SessionState::Ready => SessionHealthStatus::Failed,
            _ if snapshot.shell.shell_state == "running" => computed_health,
            _ => computed_health,
        };
        let mut payload = HashMap::new();
        payload.insert("state".to_string(), snapshot.current_state.as_str().to_string());
        payload.insert("health".to_string(), health.as_str().to_string());
        payload.insert("shell_status".to_string(), snapshot.shell.shell_state);
        payload.insert(
            "session_target_status".to_string(),
            is_active("velyx-session.target")
                .await
                .unwrap_or_else(|_| "unknown".to_string()),
        );
        payload.insert(
            "degraded_reason".to_string(),
            degraded_reason
                .or(snapshot.degraded_reason)
                .unwrap_or_default(),
        );
        payload.insert(
            "failed_reason".to_string(),
            failed_reason
                .or(snapshot.last_failed_reason)
                .unwrap_or_default(),
        );
        Ok(payload)
    }

    async fn retry_failed_startup(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut state = self.state.lock().await;
        state
            .increment_retry()
            .map_err(zbus::fdo::Error::Failed)?;
        self.audit
            .log_transition(
                "retry_startup",
                state.snapshot().current_state.as_str(),
                SessionState::Bootstrapping.as_str(),
                &state.snapshot().active_user_id,
                "",
                "ok",
                "retry requested",
            )
            .map_err(zbus::fdo::Error::Failed)?;
        let user_id = state.snapshot().active_user_id.clone();
        let outcome = if has_pending_first_boot(&self.base_dir) {
            run_first_boot_flow(&self.base_dir, &mut state, &self.audit, &user_id)
                .await
                .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?
        } else {
            bootstrap_session(&mut state, &self.audit, &user_id, "retry")
            .await
            .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?
        };
        let snapshot = state.snapshot();
        let mut payload = Self::status_payload(&snapshot);
        payload.insert("health".to_string(), outcome.health.as_str().to_string());
        payload.insert(
            "session_target_status".to_string(),
            is_active("velyx-session.target")
                .await
                .unwrap_or_else(|_| "unknown".to_string()),
        );
        Ok(payload)
    }

    async fn restart_service(&self, service_name: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        restart_unit(service_name)
            .await
            .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?;
        self.audit
            .log_transition(
                "restart_service",
                "",
                "",
                "",
                service_name,
                "ok",
                "systemctl --user restart",
            )
            .map_err(zbus::fdo::Error::Failed)?;
        let state = self.state.lock().await;
        let mut payload = Self::status_payload(&state.snapshot());
        payload.insert("service_name".to_string(), service_name.to_string());
        payload.insert("result".to_string(), "restarted".to_string());
        payload.insert(
            "service_status".to_string(),
            is_active(service_name)
                .await
                .unwrap_or_else(|_| "unknown".to_string()),
        );
        Ok(payload)
    }

    async fn stop_user_session(&self) -> zbus::fdo::Result<bool> {
        let mut state = self.state.lock().await;
        let from = state.snapshot().current_state.as_str().to_string();
        stop_target("velyx-session.target")
            .await
            .map_err(|err: SessionManagerError| zbus::fdo::Error::Failed(err.message()))?;
        state.clear_runtime().map_err(zbus::fdo::Error::Failed)?;
        self.audit
            .log_transition(
                "stop_user_session",
                &from,
                SessionState::Idle.as_str(),
                &state.snapshot().active_user_id,
                "velyx-session.target",
                "ok",
                "systemctl --user stop velyx-session.target",
            )
            .map_err(zbus::fdo::Error::Failed)?;
        Ok(true)
    }

    async fn stop_session(&self) -> zbus::fdo::Result<bool> {
        self.stop_user_session().await
    }

    async fn run_health_checks(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let units = sorted_runtime_units();
        let mut checks = check_required_services(&units, 1000).await;
        checks.extend(check_optional_services(&units, 1000).await);
        Ok(checks.into_iter().map(|entry| entry.to_map()).collect())
    }
}
