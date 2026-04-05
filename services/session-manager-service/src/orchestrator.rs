use crate::audit::SessionAuditLogger;
use crate::model::{AppRegistryEntry, AppRuntimeSnapshot, SessionState};
use crate::shell_launch::build_shell_runtime;
use crate::state::SessionStateStore;
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

const ORCHESTRATION_INTERVAL_SECS: u64 = 3;
const MAX_RESTART_ATTEMPTS: u32 = 3;

pub fn default_app_registry() -> Vec<AppRegistryEntry> {
    vec![
        AppRegistryEntry {
            app_id: "com.velyx.browser".to_string(),
            required: false,
            autostart: false,
        },
        AppRegistryEntry {
            app_id: "com.velyx.files".to_string(),
            required: false,
            autostart: false,
        },
        AppRegistryEntry {
            app_id: "com.velyx.testapp".to_string(),
            required: false,
            autostart: false,
        },
    ]
}

pub fn load_app_registry(base_dir: &Path) -> Vec<AppRegistryEntry> {
    let path = base_dir.join("app_registry.json");
    fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Vec<AppRegistryEntry>>(&raw).ok())
        .unwrap_or_else(default_app_registry)
}

async fn launcher_call_map(method: &str, app_id: &str) -> Result<HashMap<String, String>, String> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|err| format!("dbus session connection failed: {err}"))?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Launcher",
        "/com/velyx/Launcher",
        "com.velyx.Launcher1",
    )
    .await
    .map_err(|err| format!("launcher proxy init failed: {err}"))?;
    proxy
        .call(method, &(app_id))
        .await
        .map_err(|err| format!("launcher call {method} failed: {err}"))
}

fn registry_map(entries: &[AppRegistryEntry]) -> HashMap<String, AppRegistryEntry> {
    entries
        .iter()
        .cloned()
        .map(|entry| (entry.app_id.clone(), entry))
        .collect()
}

fn build_app_snapshot(
    entry: &AppRegistryEntry,
    payload: Option<&HashMap<String, String>>,
    previous_retry_count: u32,
) -> AppRuntimeSnapshot {
    let state = payload
        .and_then(|map| map.get("state").cloned().or_else(|| map.get("status").cloned()))
        .unwrap_or_else(|| "idle".to_string());
    let stop_requested = payload
        .and_then(|map| map.get("stop_requested"))
        .map(|reason| reason == "true")
        .unwrap_or(false);

    AppRuntimeSnapshot {
        app_id: entry.app_id.clone(),
        required: entry.required,
        autostart: entry.autostart,
        state: state.clone(),
        pid: payload
            .and_then(|map| map.get("pid"))
            .and_then(|value| value.parse::<u32>().ok()),
        launched_at: payload.and_then(|map| map.get("launched_at").cloned()),
        exited_at: payload
            .and_then(|map| map.get("exited_at").cloned())
            .filter(|value| !value.is_empty()),
        exit_code: payload
            .and_then(|map| map.get("exit_code"))
            .and_then(|value| {
                if value.is_empty() {
                    None
                } else {
                    value.parse::<i32>().ok()
                }
            }),
        launch_status: payload
            .and_then(|map| map.get("launch_status").cloned())
            .unwrap_or_else(|| state.clone()),
        sandbox_id: payload
            .and_then(|map| map.get("sandbox_id").cloned())
            .filter(|value| !value.is_empty()),
        failure_reason: payload
            .and_then(|map| map.get("failure_reason").cloned())
            .filter(|value| !value.is_empty()),
        retry_count: previous_retry_count,
        stop_requested,
    }
}

fn desired_running(app: &AppRuntimeSnapshot) -> bool {
    app.required || app.autostart
}

fn compute_orchestrated_state(
    shell_running: bool,
    required_services_ok: bool,
    apps: &[AppRuntimeSnapshot],
) -> (SessionState, String) {
    if !shell_running {
        return (SessionState::Failed, "shell_not_running".to_string());
    }
    if !required_services_ok {
        return (SessionState::Failed, "required_service_unavailable".to_string());
    }
    if let Some(app) = apps
        .iter()
        .find(|app| app.required && app.state != "running")
    {
        return (
            SessionState::Failed,
            format!("required_app_not_running:{}", app.app_id),
        );
    }
    if let Some(app) = apps
        .iter()
        .find(|app| !app.required && app.autostart && app.state != "running")
    {
        return (
            SessionState::Degraded,
            format!("optional_app_not_running:{}", app.app_id),
        );
    }
    (SessionState::Ready, "all_required_apps_running".to_string())
}

async fn run_autostart_for_app(
    audit: &SessionAuditLogger,
    user_id: &str,
    entry: &AppRegistryEntry,
) -> Result<HashMap<String, String>, String> {
    let _ = audit.log_transition(
        "app_autostart_begin",
        "",
        "",
        user_id,
        &entry.app_id,
        "pending",
        "autostart requested by session-manager",
    );
    let payload = launcher_call_map("Launch", &entry.app_id).await?;
    let status = payload
        .get("status")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let action = if status == "launched" || status == "already_running" {
        "app_autostart_ok"
    } else {
        "app_autostart_failed"
    };
    let _ = audit.log_transition(
        action,
        "",
        "",
        user_id,
        &entry.app_id,
        &status,
        payload.get("reason").map(|s| s.as_str()).unwrap_or(""),
    );
    Ok(payload)
}

async fn run_restart_for_app(
    audit: &SessionAuditLogger,
    user_id: &str,
    app_id: &str,
) -> Result<HashMap<String, String>, String> {
    let _ = audit.log_transition(
        "app_restart_attempt",
        "",
        "",
        user_id,
        app_id,
        "pending",
        "required app down; restart requested",
    );
    let payload = launcher_call_map("RestartApp", app_id).await?;
    let status = payload
        .get("status")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let action = if status == "launched" || status == "already_running" {
        "app_restart_ok"
    } else {
        "app_restart_failed"
    };
    let _ = audit.log_transition(
        action,
        "",
        "",
        user_id,
        app_id,
        &status,
        payload.get("reason").map(|s| s.as_str()).unwrap_or(""),
    );
    Ok(payload)
}

async fn orchestrate_once(
    state: &Arc<Mutex<SessionStateStore>>,
    audit: &SessionAuditLogger,
    base_dir: &Path,
    user_id: &str,
) {
    let registry = load_app_registry(base_dir);
    let previous_snapshot = {
        let store = state.lock().await;
        store.snapshot()
    };
    let previous_map = previous_snapshot
        .apps
        .iter()
        .cloned()
        .map(|app| (app.app_id.clone(), app))
        .collect::<HashMap<_, _>>();

    let registry_lookup = registry_map(&registry);
    let mut apps = Vec::new();

    for entry in &registry {
        let payload = launcher_call_map("GetAppRuntime", &entry.app_id).await.ok();
        let previous_retry_count = previous_map
            .get(&entry.app_id)
            .map(|app| app.retry_count)
            .unwrap_or(0);
        apps.push(build_app_snapshot(entry, payload.as_ref(), previous_retry_count));
    }

    for app in &mut apps {
        if app.autostart && app.state == "idle" {
            if let Ok(payload) = run_autostart_for_app(audit, user_id, &registry_lookup[&app.app_id]).await {
                *app = build_app_snapshot(
                    &registry_lookup[&app.app_id],
                    Some(&payload),
                    app.retry_count,
                );
            }
        }
    }

    for app in &mut apps {
        if !desired_running(app) || app.stop_requested || app.state == "running" {
            app.retry_count = 0;
            continue;
        }
        if matches!(app.state.as_str(), "idle" | "exited" | "failed" | "stopped") {
            let _ = audit.log_transition(
                "app_detected_down",
                "",
                "",
                user_id,
                &app.app_id,
                &app.state,
                app.failure_reason.as_deref().unwrap_or(""),
            );
            if app.retry_count >= MAX_RESTART_ATTEMPTS {
                continue;
            }
            match run_restart_for_app(audit, user_id, &app.app_id).await {
                Ok(payload) => {
                    let status = payload
                        .get("status")
                        .cloned()
                        .unwrap_or_else(|| "failed".to_string());
                    if status == "launched" || status == "already_running" {
                        *app = build_app_snapshot(
                            &registry_lookup[&app.app_id],
                            Some(&payload),
                            0,
                        );
                    } else {
                        app.retry_count += 1;
                        app.failure_reason = payload
                            .get("reason")
                            .cloned()
                            .filter(|value| !value.is_empty());
                    }
                }
                Err(err) => {
                    app.retry_count += 1;
                    app.failure_reason = Some(err);
                }
            }
        }
    }

    let shell_running = build_shell_runtime()
        .await
        .map(|shell| shell.shell_state == "active" || shell.shell_state == "running")
        .unwrap_or(false);
    let required_services_ok = previous_snapshot
        .required_services
        .iter()
        .all(|service| service.status == "available");
    let (new_state, reason) = compute_orchestrated_state(shell_running, required_services_ok, &apps);

    let mut store = state.lock().await;
    let old_state = store.snapshot().current_state.as_str().to_string();
    let _ = store.set_app_snapshots(apps.clone());
    let _ = if new_state == SessionState::Ready {
        store.mark_ready()
    } else if new_state == SessionState::Degraded {
        store.mark_degraded(reason.clone())
    } else {
        store.mark_failed(reason.clone())
    };
    let _ = store.update_state(new_state.clone());
    if old_state != new_state.as_str() {
        let _ = audit.log_transition(
            "session_state_changed",
            &old_state,
            new_state.as_str(),
            user_id,
            "",
            new_state.as_str(),
            &reason,
        );
    }
}

pub fn spawn_orchestrator_loop(
    state: Arc<Mutex<SessionStateStore>>,
    audit: SessionAuditLogger,
    base_dir: std::path::PathBuf,
    user_id: String,
    running: Arc<AtomicBool>,
) -> JoinHandle<()> {
    task::spawn(async move {
        let _ = audit.log_transition(
            "session_start",
            "idle",
            "ready",
            &user_id,
            "",
            "ok",
            &format!("orchestrator started at {}", Utc::now().to_rfc3339()),
        );
        while running.load(Ordering::SeqCst) {
            orchestrate_once(&state, &audit, &base_dir, &user_id).await;
            sleep(Duration::from_secs(ORCHESTRATION_INTERVAL_SECS)).await;
        }
    })
}
