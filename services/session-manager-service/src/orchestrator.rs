use crate::automation::evaluate_rules;
use crate::audit::SessionAuditLogger;
use crate::model::{
    AppRegistryEntry, AppRuntimeSnapshot, SessionState, SpaceRegistryEntry, SpaceRuntimeSnapshot,
    SpaceRuntimeState, SpaceSource, SpaceStatus, SpacesRegistryFile,
};
use crate::shell_launch::build_shell_runtime;
use crate::state::SessionStateStore;
use chrono::Utc;
use std::collections::{HashMap, HashSet};
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
const DEFAULT_SPACE_ID: &str = "general";

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

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn default_spaces_registry() -> SpacesRegistryFile {
    let created_at = now();
    SpacesRegistryFile {
        active_space_id: Some(DEFAULT_SPACE_ID.to_string()),
        spaces: vec![
            SpaceRegistryEntry {
                space_id: "general".to_string(),
                display_name: "General".to_string(),
                description: Some("Общий рабочий space".to_string()),
                apps: vec!["com.velyx.browser".to_string(), "com.velyx.files".to_string()],
                autostart_apps: vec!["com.velyx.browser".to_string()],
                required_apps: vec![],
                preferred_active_app: Some("com.velyx.browser".to_string()),
                security_mode: "normal".to_string(),
                permissions_profile: Some("default".to_string()),
                focus_policy: "prefer_active_app".to_string(),
                ui_layout: Some("standard".to_string()),
                status: SpaceStatus::Active,
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
                source: SpaceSource::System,
            },
            SpaceRegistryEntry {
                space_id: "development".to_string(),
                display_name: "Development".to_string(),
                description: Some("Контекст для разработки и проверки".to_string()),
                apps: vec![
                    "com.velyx.browser".to_string(),
                    "com.velyx.files".to_string(),
                    "com.velyx.testapp".to_string(),
                ],
                autostart_apps: vec!["com.velyx.files".to_string()],
                required_apps: vec![],
                preferred_active_app: Some("com.velyx.files".to_string()),
                security_mode: "relaxed".to_string(),
                permissions_profile: Some("developer".to_string()),
                focus_policy: "prefer_active_app".to_string(),
                ui_layout: Some("development".to_string()),
                status: SpaceStatus::Inactive,
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
                source: SpaceSource::System,
            },
            SpaceRegistryEntry {
                space_id: "safe-web".to_string(),
                display_name: "Safe Web".to_string(),
                description: Some("Ограниченный браузерный контекст".to_string()),
                apps: vec!["com.velyx.browser".to_string()],
                autostart_apps: vec!["com.velyx.browser".to_string()],
                required_apps: vec!["com.velyx.browser".to_string()],
                preferred_active_app: Some("com.velyx.browser".to_string()),
                security_mode: "strict".to_string(),
                permissions_profile: Some("safe_web".to_string()),
                focus_policy: "prefer_active_app".to_string(),
                ui_layout: Some("focused".to_string()),
                status: SpaceStatus::Inactive,
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
                source: SpaceSource::System,
            },
            SpaceRegistryEntry {
                space_id: "recovery".to_string(),
                display_name: "Recovery".to_string(),
                description: Some("Диагностический и recovery context".to_string()),
                apps: vec!["com.velyx.files".to_string()],
                autostart_apps: vec!["com.velyx.files".to_string()],
                required_apps: vec![],
                preferred_active_app: Some("com.velyx.files".to_string()),
                security_mode: "diagnostic".to_string(),
                permissions_profile: Some("recovery".to_string()),
                focus_policy: "prefer_active_app".to_string(),
                ui_layout: Some("diagnostic".to_string()),
                status: SpaceStatus::Inactive,
                created_at: created_at.clone(),
                updated_at: created_at,
                source: SpaceSource::System,
            },
        ],
    }
}

fn save_spaces_registry(base_dir: &Path, registry: &SpacesRegistryFile) -> Result<(), String> {
    let path = base_dir.join("spaces_registry.json");
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(registry)
        .map_err(|err| format!("spaces registry serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("spaces registry temp write failed: {err}"))?;
    fs::rename(&tmp, &path).map_err(|err| format!("spaces registry rename failed: {err}"))
}

pub fn load_spaces_registry(base_dir: &Path) -> SpacesRegistryFile {
    let path = base_dir.join("spaces_registry.json");
    let mut seeded = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<SpacesRegistryFile>(&raw).ok())
        .unwrap_or_else(default_spaces_registry);

    if seeded.spaces.is_empty() {
        seeded = default_spaces_registry();
    }
    if seeded.active_space_id.is_none() {
        seeded.active_space_id = Some(DEFAULT_SPACE_ID.to_string());
    }
    let active_id = seeded
        .active_space_id
        .clone()
        .unwrap_or_else(|| DEFAULT_SPACE_ID.to_string());
    let active_exists = seeded.spaces.iter().any(|space| space.space_id == active_id);
    if !active_exists {
        seeded.active_space_id = Some(DEFAULT_SPACE_ID.to_string());
    }
    for space in &mut seeded.spaces {
        space.status = if seeded.active_space_id.as_deref() == Some(space.space_id.as_str()) {
            SpaceStatus::Active
        } else {
            SpaceStatus::Inactive
        };
    }
    let _ = save_spaces_registry(base_dir, &seeded);
    seeded
}

pub fn activate_space(base_dir: &Path, space_id: &str) -> Result<SpaceRegistryEntry, String> {
    let mut registry = load_spaces_registry(base_dir);
    if !registry.spaces.iter().any(|space| space.space_id == space_id) {
        return Err(format!("unknown space_id {}", space_id));
    }
    registry.active_space_id = Some(space_id.to_string());
    for space in &mut registry.spaces {
        space.status = if space.space_id == space_id {
            space.updated_at = now();
            SpaceStatus::Active
        } else {
            SpaceStatus::Inactive
        };
    }
    save_spaces_registry(base_dir, &registry)?;
    registry
        .spaces
        .into_iter()
        .find(|space| space.space_id == space_id)
        .ok_or_else(|| format!("unknown space_id {}", space_id))
}

fn current_space(registry: &SpacesRegistryFile) -> Option<SpaceRegistryEntry> {
    let active_id = registry
        .active_space_id
        .clone()
        .unwrap_or_else(|| DEFAULT_SPACE_ID.to_string());
    registry
        .spaces
        .iter()
        .find(|space| space.space_id == active_id)
        .cloned()
        .or_else(|| registry.spaces.first().cloned())
}

pub fn list_spaces_payload(base_dir: &Path, snapshot: &crate::model::SessionSnapshot) -> Vec<HashMap<String, String>> {
    let registry = load_spaces_registry(base_dir);
    registry
        .spaces
        .iter()
        .map(|space| {
            let runtime = snapshot
                .spaces
                .iter()
                .find(|item| item.space_id == space.space_id)
                .cloned()
                .unwrap_or_else(|| SpaceRuntimeSnapshot {
                    space_id: space.space_id.clone(),
                    display_name: space.display_name.clone(),
                    source: space.source.as_str().to_string(),
                    status: space.status.as_str().to_string(),
                    runtime_state: "degraded".to_string(),
                    security_mode: space.security_mode.clone(),
                    permissions_profile: space.permissions_profile.clone(),
                    focus_policy: space.focus_policy.clone(),
                    preferred_active_app: space.preferred_active_app.clone(),
                    apps: space.apps.clone(),
                    autostart_apps: space.autostart_apps.clone(),
                    required_apps: space.required_apps.clone(),
                    active: snapshot.active_space_id.as_deref() == Some(space.space_id.as_str()),
                    reason: Some("runtime_not_yet_computed".to_string()),
                });
            space.to_map(
                &runtime.runtime_state,
                runtime.reason.as_deref().unwrap_or_default(),
                runtime.active,
            )
        })
        .collect()
}

pub fn current_space_payload(base_dir: &Path, snapshot: &crate::model::SessionSnapshot) -> HashMap<String, String> {
    let registry = load_spaces_registry(base_dir);
    let Some(space) = current_space(&registry) else {
        return HashMap::new();
    };
    let runtime = snapshot
        .spaces
        .iter()
        .find(|item| item.space_id == space.space_id)
        .cloned()
        .unwrap_or_else(|| SpaceRuntimeSnapshot {
            space_id: space.space_id.clone(),
            display_name: space.display_name.clone(),
            source: space.source.as_str().to_string(),
            status: space.status.as_str().to_string(),
            runtime_state: "degraded".to_string(),
            security_mode: space.security_mode.clone(),
            permissions_profile: space.permissions_profile.clone(),
            focus_policy: space.focus_policy.clone(),
            preferred_active_app: space.preferred_active_app.clone(),
            apps: space.apps.clone(),
            autostart_apps: space.autostart_apps.clone(),
            required_apps: space.required_apps.clone(),
            active: true,
            reason: Some("runtime_not_yet_computed".to_string()),
        });
    space.to_map(
        &runtime.runtime_state,
        runtime.reason.as_deref().unwrap_or_default(),
        true,
    )
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

fn monitored_registry_entries(
    app_registry: &[AppRegistryEntry],
    spaces_registry: &SpacesRegistryFile,
) -> Vec<AppRegistryEntry> {
    let mut merged = registry_map(app_registry);
    for space in &spaces_registry.spaces {
        for app_id in &space.apps {
            merged.entry(app_id.clone()).or_insert(AppRegistryEntry {
                app_id: app_id.clone(),
                required: false,
                autostart: false,
            });
        }
    }
    let mut values = merged.into_values().collect::<Vec<_>>();
    values.sort_by(|left, right| left.app_id.cmp(&right.app_id));
    values
}

fn build_app_snapshot(
    entry: &AppRegistryEntry,
    payload: Option<&HashMap<String, String>>,
    previous_retry_count: u32,
    in_active_space: bool,
    required: bool,
    autostart: bool,
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
        required,
        autostart,
        in_active_space,
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

fn compute_space_state(space: &SpaceRegistryEntry, apps: &[AppRuntimeSnapshot]) -> (SpaceRuntimeState, String) {
    for app_id in &space.required_apps {
        let Some(app) = apps.iter().find(|entry| &entry.app_id == app_id) else {
            return (SpaceRuntimeState::Failed, format!("missing_required_app:{}", app_id));
        };
        if app.state != "running" {
            return (
                SpaceRuntimeState::Failed,
                format!("required_app_not_running:{}", app.app_id),
            );
        }
    }
    for app_id in &space.autostart_apps {
        let Some(app) = apps.iter().find(|entry| &entry.app_id == app_id) else {
            return (SpaceRuntimeState::Degraded, format!("missing_autostart_app:{}", app_id));
        };
        if app.state != "running" {
            return (
                SpaceRuntimeState::Degraded,
                format!("autostart_app_not_running:{}", app.app_id),
            );
        }
    }
    (SpaceRuntimeState::Ready, "space_ready".to_string())
}

fn compute_orchestrated_state(
    shell_running: bool,
    required_services_ok: bool,
    active_space_state: &SpaceRuntimeState,
    reason: &str,
) -> (SessionState, String) {
    if !shell_running {
        return (SessionState::Failed, "shell_not_running".to_string());
    }
    if !required_services_ok {
        return (SessionState::Failed, "required_service_unavailable".to_string());
    }
    match active_space_state {
        SpaceRuntimeState::Ready => (SessionState::Ready, "all_required_apps_running".to_string()),
        SpaceRuntimeState::Degraded => (SessionState::Degraded, reason.to_string()),
        SpaceRuntimeState::Failed => (SessionState::Failed, reason.to_string()),
    }
}

async fn run_autostart_for_app(
    audit: &SessionAuditLogger,
    user_id: &str,
    entry: &AppRegistryEntry,
    space_id: &str,
) -> Result<HashMap<String, String>, String> {
    let _ = audit.log_transition(
        "space_app_autostart_begin",
        "",
        "",
        user_id,
        &entry.app_id,
        "pending",
        &format!("space_id={space_id} autostart requested by session-manager"),
    );
    let payload = launcher_call_map("Launch", &entry.app_id).await?;
    let status = payload
        .get("status")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let action = if status == "launched" || status == "already_running" {
        "space_app_autostart_ok"
    } else {
        "space_app_autostart_failed"
    };
    let _ = audit.log_transition(
        action,
        "",
        "",
        user_id,
        &entry.app_id,
        &status,
        &format!(
            "space_id={} reason={}",
            space_id,
            payload.get("reason").map(|s| s.as_str()).unwrap_or("")
        ),
    );
    Ok(payload)
}

async fn run_restart_for_app(
    audit: &SessionAuditLogger,
    user_id: &str,
    app_id: &str,
    space_id: &str,
) -> Result<HashMap<String, String>, String> {
    let _ = audit.log_transition(
        "app_restart_attempt",
        "",
        "",
        user_id,
        app_id,
        "pending",
        &format!("space_id={} required app down; restart requested", space_id),
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
        &format!(
            "space_id={} reason={}",
            space_id,
            payload.get("reason").map(|s| s.as_str()).unwrap_or("")
        ),
    );
    Ok(payload)
}

pub async fn orchestrate_once(
    state: &Arc<Mutex<SessionStateStore>>,
    audit: &SessionAuditLogger,
    base_dir: &Path,
    user_id: &str,
) {
    let app_registry = load_app_registry(base_dir);
    let spaces_registry = load_spaces_registry(base_dir);
    let Some(active_space) = current_space(&spaces_registry) else {
        return;
    };
    let active_space_app_set = active_space.apps.iter().cloned().collect::<HashSet<_>>();

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

    let registry_entries = monitored_registry_entries(&app_registry, &spaces_registry);
    let registry_lookup = registry_map(&registry_entries);
    let mut apps = Vec::new();

    for entry in &registry_entries {
        let payload = launcher_call_map("GetAppRuntime", &entry.app_id).await.ok();
        let previous_retry_count = previous_map
            .get(&entry.app_id)
            .map(|app| app.retry_count)
            .unwrap_or(0);
        let required = active_space.required_apps.iter().any(|item| item == &entry.app_id);
        let autostart = active_space.autostart_apps.iter().any(|item| item == &entry.app_id);
        let in_active_space = active_space_app_set.contains(&entry.app_id);
        apps.push(build_app_snapshot(
            entry,
            payload.as_ref(),
            previous_retry_count,
            in_active_space,
            required,
            autostart,
        ));
    }

    for app in &mut apps {
        if app.autostart && app.state == "idle" {
            if let Ok(payload) =
                run_autostart_for_app(audit, user_id, &registry_lookup[&app.app_id], &active_space.space_id).await
            {
                *app = build_app_snapshot(
                    &registry_lookup[&app.app_id],
                    Some(&payload),
                    app.retry_count,
                    app.in_active_space,
                    app.required,
                    app.autostart,
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
                &format!(
                    "space_id={} {}",
                    active_space.space_id,
                    app.failure_reason.as_deref().unwrap_or("")
                ),
            );
            if app.retry_count >= MAX_RESTART_ATTEMPTS {
                continue;
            }
            match run_restart_for_app(audit, user_id, &app.app_id, &active_space.space_id).await {
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
                            app.in_active_space,
                            app.required,
                            app.autostart,
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

    let active_space_state = compute_space_state(&active_space, &apps);
    let active_space_snapshot = SpaceRuntimeSnapshot {
        space_id: active_space.space_id.clone(),
        display_name: active_space.display_name.clone(),
        source: active_space.source.as_str().to_string(),
        status: active_space.status.as_str().to_string(),
        runtime_state: active_space_state.0.as_str().to_string(),
        security_mode: active_space.security_mode.clone(),
        permissions_profile: active_space.permissions_profile.clone(),
        focus_policy: active_space.focus_policy.clone(),
        preferred_active_app: active_space.preferred_active_app.clone(),
        apps: active_space.apps.clone(),
        autostart_apps: active_space.autostart_apps.clone(),
        required_apps: active_space.required_apps.clone(),
        active: true,
        reason: Some(active_space_state.1.clone()),
    };
    let mut spaces = Vec::new();
    for space in &spaces_registry.spaces {
        if space.space_id == active_space.space_id {
            spaces.push(active_space_snapshot.clone());
        } else {
            let (runtime_state, reason) = if space
                .apps
                .iter()
                .all(|app_id| apps.iter().any(|app| &app.app_id == app_id))
            {
                (SpaceRuntimeState::Ready, "space_available".to_string())
            } else {
                (SpaceRuntimeState::Degraded, "space_missing_app_reference".to_string())
            };
            spaces.push(SpaceRuntimeSnapshot {
                space_id: space.space_id.clone(),
                display_name: space.display_name.clone(),
                source: space.source.as_str().to_string(),
                status: space.status.as_str().to_string(),
                runtime_state: runtime_state.as_str().to_string(),
                security_mode: space.security_mode.clone(),
                permissions_profile: space.permissions_profile.clone(),
                focus_policy: space.focus_policy.clone(),
                preferred_active_app: space.preferred_active_app.clone(),
                apps: space.apps.clone(),
                autostart_apps: space.autostart_apps.clone(),
                required_apps: space.required_apps.clone(),
                active: false,
                reason: Some(reason),
            });
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
    let (new_state, reason) = compute_orchestrated_state(
        shell_running,
        required_services_ok,
        &active_space_state.0,
        &active_space_state.1,
    );

    let new_snapshot = {
        let mut store = state.lock().await;
        let snapshot_before = store.snapshot();
        let old_state = snapshot_before.current_state.as_str().to_string();
        let old_space_id = snapshot_before.active_space_id.clone().unwrap_or_default();
        let old_space_state = snapshot_before.active_space_state.clone().unwrap_or_default();
        let _ = store.set_app_snapshots(apps.clone());
        let _ = store.set_space_snapshots(
            spaces.clone(),
            Some(active_space.space_id.clone()),
            Some(active_space.display_name.clone()),
            Some(active_space_state.0.as_str().to_string()),
            Some(active_space.security_mode.clone()),
            active_space.preferred_active_app.clone(),
            active_space.apps.clone(),
        );
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
        if old_space_id != active_space.space_id {
            let _ = audit.log_transition(
                "space_activate_ok",
                &old_space_id,
                &active_space.space_id,
                user_id,
                "",
                active_space_state.0.as_str(),
                &format!("space_name={}", active_space.display_name),
            );
        }
        if old_space_state != active_space_state.0.as_str() {
            let _ = audit.log_transition(
                "space_state_changed",
                &old_space_state,
                active_space_state.0.as_str(),
                user_id,
                "",
                active_space_state.0.as_str(),
                &active_space_state.1,
            );
        }
        store.snapshot()
    };

    evaluate_rules(base_dir, audit, user_id, &previous_snapshot, &new_snapshot).await;
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
        let registry = load_spaces_registry(&base_dir);
        let active_space = current_space(&registry)
            .map(|space| space.space_id)
            .unwrap_or_else(|| DEFAULT_SPACE_ID.to_string());
        let _ = audit.log_transition(
            "space_registry_seeded",
            "",
            "",
            &user_id,
            "",
            "ok",
            &format!("active_space={}", active_space),
        );
        while running.load(Ordering::SeqCst) {
            orchestrate_once(&state, &audit, &base_dir, &user_id).await;
            sleep(Duration::from_secs(ORCHESTRATION_INTERVAL_SECS)).await;
        }
    })
}
