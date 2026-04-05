use crate::audit::SessionAuditLogger;
use crate::errors::SessionManagerError;
use crate::health::{check_optional_services, check_required_services, compute_session_health};
use crate::model::{SessionHealthStatus, SessionState, StartupOutcome};
use crate::shell_launch::{start_shell_process, verify_shell_started};
use crate::state::SessionStateStore;
use crate::units::{
    ensure_systemd_user_available, ensure_unit_start_order, start_target, UnitDefinition,
};
use chrono::Utc;

const STARTUP_TIMEOUT_MS: u64 = 5_000;

fn deadline_epoch_ms() -> u64 {
    (Utc::now().timestamp_millis() as u64) + STARTUP_TIMEOUT_MS
}

fn transition(
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id: &str,
    action: &str,
    to: SessionState,
    service_name: &str,
    result: &str,
    reason: &str,
) -> Result<(), String> {
    let from = store.snapshot().current_state.as_str().to_string();
    store.update_state(to.clone())?;
    audit.log_transition(
        action,
        &from,
        to.as_str(),
        user_id,
        service_name,
        result,
        reason,
    )
}

pub async fn bootstrap_session(
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id: &str,
    source: &str,
) -> Result<StartupOutcome, SessionManagerError> {
    store
        .set_active_user(user_id)
        .map_err(SessionManagerError::StartupFailed)?;
    store
        .set_startup_deadline_epoch_ms(Some(deadline_epoch_ms()))
        .map_err(SessionManagerError::StartupFailed)?;
    transition(
        store,
        audit,
        user_id,
        "startup_begin",
        SessionState::Bootstrapping,
        "",
        "ok",
        &format!("session bootstrap started source={}", source),
    )
    .map_err(SessionManagerError::StartupFailed)?;

    let units = ensure_core_services(store, audit, user_id).await?;
    let (required_services, optional_services) =
        wait_for_core_services(store, audit, user_id, &units).await?;

    transition(
        store,
        audit,
        user_id,
        "health_check_begin",
        SessionState::HealthChecking,
        "",
        "ok",
        "running core service health checks",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    let (health, degraded_reason, failed_reason) =
        compute_session_health(&required_services, &optional_services);
    store
        .set_service_snapshots(required_services.clone(), optional_services.clone())
        .map_err(SessionManagerError::StartupFailed)?;

    if let Some(reason) = failed_reason.clone() {
        store
            .mark_failed(reason.clone())
            .map_err(SessionManagerError::StartupFailed)?;
        transition(
            store,
            audit,
            user_id,
            "session_failed",
            SessionState::Failed,
            "",
            "failed",
            &reason,
        )
        .map_err(SessionManagerError::StartupFailed)?;
        return Ok(StartupOutcome {
            state: SessionState::Failed,
            health,
            required_services,
            optional_services,
            shell: store.snapshot().shell,
            degraded_reason: None,
            failed_reason: Some(reason),
        });
    }

    transition(
        store,
        audit,
        user_id,
        "shell_start_begin",
        SessionState::StartingShell,
        "velyx-shell.service",
        "ok",
        "starting shell after core services ready",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    let shell = match start_shell_process().await {
        Ok(shell) => shell,
        Err(err) => {
            let reason = err.message();
            store
                .mark_failed(reason.clone())
                .map_err(SessionManagerError::StartupFailed)?;
            transition(
                store,
                audit,
                user_id,
                "shell_start_failed",
                SessionState::Failed,
                "velyx-shell.service",
                "failed",
                &reason,
            )
            .map_err(SessionManagerError::StartupFailed)?;
            return Ok(StartupOutcome {
                state: SessionState::Failed,
                health: SessionHealthStatus::Failed,
                required_services,
                optional_services,
                shell: store.snapshot().shell,
                degraded_reason: None,
                failed_reason: Some(reason),
            });
        }
    };
    if let Err(err) = verify_shell_started(&shell).await {
        let reason = err.message();
        store
            .mark_failed(reason.clone())
            .map_err(SessionManagerError::StartupFailed)?;
        transition(
            store,
            audit,
            user_id,
            "shell_start_failed",
            SessionState::Failed,
            "velyx-shell.service",
            "failed",
            &reason,
        )
        .map_err(SessionManagerError::StartupFailed)?;
        return Ok(StartupOutcome {
            state: SessionState::Failed,
            health: SessionHealthStatus::Failed,
            required_services,
            optional_services,
            shell: store.snapshot().shell,
            degraded_reason: None,
            failed_reason: Some(reason),
        });
    }
    store
        .set_shell_runtime(shell.clone())
        .map_err(SessionManagerError::StartupFailed)?;
    audit
        .log_transition(
            "shell_start_ok",
            SessionState::StartingShell.as_str(),
            SessionState::StartingShell.as_str(),
            user_id,
            "velyx-shell.service",
            "ok",
            &format!("shell_pid={}", shell.shell_pid.unwrap_or_default()),
        )
        .map_err(SessionManagerError::StartupFailed)?;

    finalize_startup(
        store,
        audit,
        user_id,
        health,
        degraded_reason,
        required_services,
        optional_services,
        shell,
    )
    .await
}

pub async fn ensure_core_services(
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id: &str,
) -> Result<Vec<UnitDefinition>, SessionManagerError> {
    ensure_systemd_user_available().await?;
    transition(
        store,
        audit,
        user_id,
        "launch_core_services_begin",
        SessionState::LaunchingCoreServices,
        "",
        "ok",
        "checking unit definitions and intended startup order",
    )
    .map_err(SessionManagerError::StartupFailed)?;
    let units = ensure_unit_start_order().await?;
    start_target("velyx-session.target").await?;
    audit
        .log_transition(
            "session_target_started",
            SessionState::LaunchingCoreServices.as_str(),
            SessionState::WaitingForCoreServices.as_str(),
            user_id,
            "velyx-session.target",
            "ok",
            "systemctl --user start velyx-session.target",
        )
        .map_err(SessionManagerError::StartupFailed)?;
    for unit in &units {
        audit.log_transition(
            "service_wait_begin",
            SessionState::LaunchingCoreServices.as_str(),
            SessionState::WaitingForCoreServices.as_str(),
            user_id,
            &unit.unit_name,
            "planned",
            &format!(
                "dbus_name={} required={} order={}",
                unit.dbus_name, unit.required, unit.startup_order
            ),
        )
        .map_err(SessionManagerError::StartupFailed)?;
    }
    Ok(units)
}

pub async fn wait_for_core_services(
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id: &str,
    units: &[UnitDefinition],
) -> Result<(Vec<crate::model::ServiceHealth>, Vec<crate::model::ServiceHealth>), SessionManagerError> {
    transition(
        store,
        audit,
        user_id,
        "waiting_for_services",
        SessionState::WaitingForCoreServices,
        "",
        "ok",
        "waiting for D-Bus readiness of core services",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    let required = check_required_services(units, STARTUP_TIMEOUT_MS).await;
    let optional = check_optional_services(units, STARTUP_TIMEOUT_MS).await;

    for service in required.iter().chain(optional.iter()) {
        let action = if service.status == "available" {
            "service_ready"
        } else {
            "service_timeout"
        };
        audit
            .log_transition(
                action,
                SessionState::WaitingForCoreServices.as_str(),
                SessionState::WaitingForCoreServices.as_str(),
                user_id,
                &service.service_name,
                &service.status,
                "",
            )
            .map_err(SessionManagerError::StartupFailed)?;
    }

    Ok((required, optional))
}

pub async fn finalize_startup(
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id: &str,
    health: SessionHealthStatus,
    degraded_reason: Option<String>,
    required_services: Vec<crate::model::ServiceHealth>,
    optional_services: Vec<crate::model::ServiceHealth>,
    shell: crate::model::ShellRuntime,
) -> Result<StartupOutcome, SessionManagerError> {
    store
        .set_startup_deadline_epoch_ms(None)
        .map_err(SessionManagerError::StartupFailed)?;
    match health {
        SessionHealthStatus::Healthy => {
            store.mark_ready().map_err(SessionManagerError::StartupFailed)?;
            transition(
                store,
                audit,
                user_id,
                "session_ready",
                SessionState::Ready,
                "velyx-shell.service",
                "ok",
                "session startup completed",
            )
            .map_err(SessionManagerError::StartupFailed)?;
            audit
                .log_transition(
                    "session_ready_with_shell",
                    SessionState::Ready.as_str(),
                    SessionState::Ready.as_str(),
                    user_id,
                    "velyx-shell.service",
                    "ok",
                    &format!(
                        "shell_status={} shell_pid={}",
                        shell.shell_state,
                        shell.shell_pid.unwrap_or_default()
                    ),
                )
                .map_err(SessionManagerError::StartupFailed)?;
            Ok(StartupOutcome {
                state: SessionState::Ready,
                health,
                required_services,
                optional_services,
                shell,
                degraded_reason: None,
                failed_reason: None,
            })
        }
        SessionHealthStatus::Degraded => {
            let reason = degraded_reason.unwrap_or_else(|| "optional services unavailable".to_string());
            store
                .mark_degraded(reason.clone())
                .map_err(SessionManagerError::StartupFailed)?;
            transition(
                store,
                audit,
                user_id,
                "session_degraded",
                SessionState::Degraded,
                "velyx-shell.service",
                "degraded",
                &reason,
            )
            .map_err(SessionManagerError::StartupFailed)?;
            audit
                .log_transition(
                    "session_ready_with_shell",
                    SessionState::Degraded.as_str(),
                    SessionState::Degraded.as_str(),
                    user_id,
                    "velyx-shell.service",
                    "degraded",
                    &format!(
                        "shell_status={} shell_pid={} reason={}",
                        shell.shell_state,
                        shell.shell_pid.unwrap_or_default(),
                        reason
                    ),
                )
                .map_err(SessionManagerError::StartupFailed)?;
            Ok(StartupOutcome {
                state: SessionState::Degraded,
                health,
                required_services,
                optional_services,
                shell,
                degraded_reason: Some(reason),
                failed_reason: None,
            })
        }
        SessionHealthStatus::Failed => Err(SessionManagerError::StartupFailed(
            "session cannot finalize in failed state".to_string(),
        )),
    }
}
