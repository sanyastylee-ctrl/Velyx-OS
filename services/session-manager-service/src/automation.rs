use crate::audit::SessionAuditLogger;
use crate::model::SessionSnapshot;
use crate::orchestrator::activate_space;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RuleCondition {
    pub field: String,
    pub equals: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleRegistryEntry {
    pub rule_id: String,
    pub display_name: String,
    pub description: String,
    pub enabled: bool,
    pub trigger_type: String,
    #[serde(default)]
    pub trigger_payload: HashMap<String, String>,
    #[serde(default)]
    pub conditions: Vec<RuleCondition>,
    pub action_type: String,
    #[serde(default)]
    pub action_payload: HashMap<String, String>,
    pub cooldown_seconds: u64,
    pub last_triggered_at: Option<String>,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RulesRegistryFile {
    #[serde(default)]
    pub rules: Vec<RuleRegistryEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RulesStateFile {
    pub last_rule_id: String,
    pub last_result: String,
    pub last_trigger_type: String,
    pub last_action_type: String,
    pub last_run_at: String,
    pub last_message: String,
    pub last_seen_update_key: String,
    pub last_seen_recovery_needed: bool,
    pub last_seen_intent_run_at: String,
    pub last_seen_permission_event: String,
}

#[derive(Clone, Debug, Default)]
struct RuleEvent {
    trigger_type: String,
    payload: HashMap<String, String>,
    summary: String,
}

#[derive(Clone, Debug, Default)]
struct UpdateStateSnapshot {
    update_state: String,
    last_update_result: String,
    recovery_needed: bool,
}

#[derive(Clone, Debug, Default)]
struct IntentStateSnapshot {
    last_intent_id: String,
    last_result: String,
    last_space_id: String,
    last_run_at: String,
}

#[derive(Clone, Debug, Default)]
struct PermissionEventSnapshot {
    trigger_type: String,
    event_key: String,
    app_id: String,
    message: String,
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn rules_registry_path(base_dir: &Path) -> PathBuf {
    base_dir.join("rules_registry.json")
}

fn rules_state_path(base_dir: &Path) -> PathBuf {
    base_dir.join("rules_state.json")
}

fn rules_log_path(base_dir: &Path) -> PathBuf {
    base_dir.join("rules.log")
}

fn helper_binary(name: &str) -> PathBuf {
    if let Ok(prefix) = std::env::var("VELYX_INSTALL_PREFIX") {
        let candidate = PathBuf::from(prefix).join("bin").join(name);
        if candidate.exists() {
            return candidate;
        }
    }
    PathBuf::from(name)
}

fn log_rules(base_dir: &Path, event: &str, status: &str, detail: &str) {
    let path = rules_log_path(base_dir);
    let _ = fs::create_dir_all(base_dir);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(
            file,
            "{} event={} status={} {}",
            now(),
            event,
            status,
            detail
        );
    }
}

fn default_registry() -> RulesRegistryFile {
    let created_at = now();
    RulesRegistryFile {
        rules: vec![
            RuleRegistryEntry {
                rule_id: "auto_run_general_after_session_ready".to_string(),
                display_name: "Run General after session ready".to_string(),
                description: "Автоматически включает общий intent после готовности session."
                    .to_string(),
                enabled: true,
                trigger_type: "session_ready".to_string(),
                trigger_payload: HashMap::new(),
                conditions: Vec::new(),
                action_type: "run_intent".to_string(),
                action_payload: HashMap::from([(
                    "intent_id".to_string(),
                    "general_use".to_string(),
                )]),
                cooldown_seconds: 10,
                last_triggered_at: None,
                source: "system".to_string(),
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
            },
            RuleRegistryEntry {
                rule_id: "go_recovery_on_update_failed".to_string(),
                display_name: "Recovery on update failure".to_string(),
                description: "Переводит систему в recovery intent после failed update."
                    .to_string(),
                enabled: true,
                trigger_type: "update_failed".to_string(),
                trigger_payload: HashMap::new(),
                conditions: Vec::new(),
                action_type: "run_intent".to_string(),
                action_payload: HashMap::from([(
                    "intent_id".to_string(),
                    "recovery_mode".to_string(),
                )]),
                cooldown_seconds: 30,
                last_triggered_at: None,
                source: "system".to_string(),
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
            },
            RuleRegistryEntry {
                rule_id: "go_recovery_on_recovery_needed".to_string(),
                display_name: "Activate recovery space when recovery is needed".to_string(),
                description:
                    "Переключает систему в recovery space, если update pipeline требует восстановления."
                        .to_string(),
                enabled: true,
                trigger_type: "recovery_needed".to_string(),
                trigger_payload: HashMap::new(),
                conditions: Vec::new(),
                action_type: "activate_space".to_string(),
                action_payload: HashMap::from([(
                    "space_id".to_string(),
                    "recovery".to_string(),
                )]),
                cooldown_seconds: 30,
                last_triggered_at: None,
                source: "system".to_string(),
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
            },
            RuleRegistryEntry {
                rule_id: "restart_required_app_if_crashed".to_string(),
                display_name: "Restart required app after crash".to_string(),
                description:
                    "Пытается перезапустить required app в active space после runtime failure."
                        .to_string(),
                enabled: true,
                trigger_type: "app_runtime_failed".to_string(),
                trigger_payload: HashMap::new(),
                conditions: vec![RuleCondition {
                    field: "app_required".to_string(),
                    equals: "true".to_string(),
                }],
                action_type: "restart_app".to_string(),
                action_payload: HashMap::new(),
                cooldown_seconds: 15,
                last_triggered_at: None,
                source: "system".to_string(),
                created_at: created_at.clone(),
                updated_at: created_at.clone(),
            },
            RuleRegistryEntry {
                rule_id: "log_permission_denied".to_string(),
                display_name: "Audit permission denied".to_string(),
                description: "Отмечает permission denied как automation audit event.".to_string(),
                enabled: true,
                trigger_type: "permission_denied".to_string(),
                trigger_payload: HashMap::new(),
                conditions: Vec::new(),
                action_type: "write_audit".to_string(),
                action_payload: HashMap::from([(
                    "message".to_string(),
                    "permission denied observed by rules engine".to_string(),
                )]),
                cooldown_seconds: 5,
                last_triggered_at: None,
                source: "system".to_string(),
                created_at: created_at.clone(),
                updated_at: created_at,
            },
        ],
    }
}

fn save_registry(base_dir: &Path, registry: &RulesRegistryFile) -> Result<(), String> {
    let path = rules_registry_path(base_dir);
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(registry)
        .map_err(|err| format!("rules registry serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("rules registry temp write failed: {err}"))?;
    fs::rename(&tmp, &path).map_err(|err| format!("rules registry rename failed: {err}"))
}

fn save_state(base_dir: &Path, state: &RulesStateFile) -> Result<(), String> {
    let path = rules_state_path(base_dir);
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(state)
        .map_err(|err| format!("rules state serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("rules state temp write failed: {err}"))?;
    fs::rename(&tmp, &path).map_err(|err| format!("rules state rename failed: {err}"))
}

pub fn load_rules_registry(base_dir: &Path) -> RulesRegistryFile {
    let path = rules_registry_path(base_dir);
    let mut registry = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<RulesRegistryFile>(&raw).ok())
        .unwrap_or_else(default_registry);
    if registry.rules.is_empty() {
        registry = default_registry();
    }
    if !path.exists() {
        log_rules(
            base_dir,
            "rule_registry_seeded",
            "ok",
            &format!("count={}", registry.rules.len()),
        );
    }
    let _ = save_registry(base_dir, &registry);
    registry
}

fn load_rules_state(base_dir: &Path) -> RulesStateFile {
    let path = rules_state_path(base_dir);
    let state = fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<RulesStateFile>(&raw).ok())
        .unwrap_or_default();
    let _ = save_state(base_dir, &state);
    state
}

fn read_update_state(base_dir: &Path) -> UpdateStateSnapshot {
    let path = base_dir.join("update_state.json");
    let value = fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .unwrap_or(Value::Null);
    UpdateStateSnapshot {
        update_state: value
            .get("update_state")
            .and_then(|item| item.as_str())
            .unwrap_or("unknown")
            .to_string(),
        last_update_result: value
            .get("last_update_result")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string(),
        recovery_needed: value
            .get("recovery_needed")
            .and_then(|item| item.as_bool())
            .unwrap_or(false),
    }
}

fn read_intent_state(base_dir: &Path) -> IntentStateSnapshot {
    let path = base_dir.join("intent_state.json");
    let value = fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .unwrap_or(Value::Null);
    IntentStateSnapshot {
        last_intent_id: value
            .get("last_intent_id")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string(),
        last_result: value
            .get("last_result")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string(),
        last_space_id: value
            .get("last_space_id")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string(),
        last_run_at: value
            .get("last_run_at")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string(),
    }
}

fn parse_permission_snapshot(base_dir: &Path) -> PermissionEventSnapshot {
    let path = base_dir.join("launcher_history.log");
    let Ok(raw) = fs::read_to_string(path) else {
        return PermissionEventSnapshot::default();
    };
    for line in raw.lines().rev() {
        if line.contains("action=permission_gate_deny")
            || (line.contains("action=launch_denied") && line.contains("permission"))
        {
            return PermissionEventSnapshot {
                trigger_type: "permission_denied".to_string(),
                event_key: line.to_string(),
                app_id: extract_log_token(line, "app_id"),
                message: line.to_string(),
            };
        }
        if line.contains("action=permission_gate_prompt") || line.contains("action=launch_prompted") {
            return PermissionEventSnapshot {
                trigger_type: "permission_prompted".to_string(),
                event_key: line.to_string(),
                app_id: extract_log_token(line, "app_id"),
                message: line.to_string(),
            };
        }
    }
    PermissionEventSnapshot::default()
}

fn extract_log_token(line: &str, key: &str) -> String {
    let needle = format!("{key}=");
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&needle))
        .unwrap_or_default()
        .to_string()
}

fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn cooldown_active(rule: &RuleRegistryEntry) -> bool {
    if rule.cooldown_seconds == 0 {
        return false;
    }
    let Some(last_triggered_at) = rule.last_triggered_at.as_deref() else {
        return false;
    };
    let Some(last_ts) = parse_timestamp(last_triggered_at) else {
        return false;
    };
    let elapsed = Utc::now().signed_duration_since(last_ts).num_seconds();
    elapsed >= 0 && elapsed < rule.cooldown_seconds as i64
}

fn make_event(trigger_type: &str, payload: HashMap<String, String>, summary: String) -> RuleEvent {
    RuleEvent {
        trigger_type: trigger_type.to_string(),
        payload,
        summary,
    }
}

fn matches_conditions(rule: &RuleRegistryEntry, context: &HashMap<String, String>) -> Option<String> {
    for condition in &rule.conditions {
        let current = context.get(&condition.field).cloned().unwrap_or_default();
        if current != condition.equals {
            return Some(format!(
                "condition {} expected {} got {}",
                condition.field, condition.equals, current
            ));
        }
    }
    None
}

fn build_context(
    snapshot: &SessionSnapshot,
    update_state: &UpdateStateSnapshot,
    intent_state: &IntentStateSnapshot,
    event: &RuleEvent,
) -> HashMap<String, String> {
    let mut context = event.payload.clone();
    context.insert(
        "session_state".to_string(),
        snapshot.current_state.as_str().to_string(),
    );
    context.insert(
        "active_space".to_string(),
        snapshot.active_space_id.clone().unwrap_or_default(),
    );
    context.insert(
        "active_space_state".to_string(),
        snapshot.active_space_state.clone().unwrap_or_default(),
    );
    context.insert(
        "recovery_needed".to_string(),
        update_state.recovery_needed.to_string(),
    );
    context.insert(
        "update_status".to_string(),
        update_state.update_state.clone(),
    );
    context.insert(
        "update_result".to_string(),
        update_state.last_update_result.clone(),
    );
    context.insert("intent_id".to_string(), intent_state.last_intent_id.clone());
    context.insert(
        "intent_result".to_string(),
        intent_state.last_result.clone(),
    );
    context.insert(
        "intent_space_id".to_string(),
        intent_state.last_space_id.clone(),
    );
    context
}

fn run_helper_command(program: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("{} failed to start: {err}", program.display()))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
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

async fn execute_action(
    base_dir: &Path,
    audit: &SessionAuditLogger,
    user_id: &str,
    snapshot: &SessionSnapshot,
    rule: &RuleRegistryEntry,
    event: &RuleEvent,
) -> (String, String) {
    match rule.action_type.as_str() {
        "run_intent" => {
            let intent_id = rule
                .action_payload
                .get("intent_id")
                .cloned()
                .or_else(|| event.payload.get("intent_id").cloned())
                .unwrap_or_default();
            if intent_id.is_empty() {
                return ("failed".to_string(), "missing_intent_id".to_string());
            }
            let helper = helper_binary("velyx-intent");
            match run_helper_command(&helper, &["run", &intent_id]) {
                Ok(output) => {
                    let message = if output.is_empty() {
                        format!("intent_id={intent_id}")
                    } else {
                        output
                    };
                    if message.contains("success_with_warnings") || message.contains("degraded") {
                        ("degraded".to_string(), message)
                    } else {
                        ("ok".to_string(), message)
                    }
                }
                Err(err) => ("failed".to_string(), err),
            }
        }
        "activate_space" => {
            let space_id = rule
                .action_payload
                .get("space_id")
                .cloned()
                .or_else(|| event.payload.get("space_id").cloned())
                .unwrap_or_default();
            if space_id.is_empty() {
                return ("failed".to_string(), "missing_space_id".to_string());
            }
            match activate_space(base_dir, &space_id) {
                Ok(_) => ("ok".to_string(), format!("space_id={space_id}")),
                Err(err) => ("failed".to_string(), err),
            }
        }
        "ensure_space_active" => {
            let space_id = rule
                .action_payload
                .get("space_id")
                .cloned()
                .or_else(|| event.payload.get("space_id").cloned())
                .unwrap_or_default();
            if space_id.is_empty() {
                return ("failed".to_string(), "missing_space_id".to_string());
            }
            if snapshot.active_space_id.as_deref() == Some(space_id.as_str()) {
                ("skipped".to_string(), format!("space_id={space_id} already_active"))
            } else {
                match activate_space(base_dir, &space_id) {
                    Ok(_) => ("ok".to_string(), format!("space_id={space_id}")),
                    Err(err) => ("failed".to_string(), err),
                }
            }
        }
        "restart_app" => {
            let app_id = rule
                .action_payload
                .get("app_id")
                .cloned()
                .or_else(|| event.payload.get("app_id").cloned())
                .unwrap_or_default();
            if app_id.is_empty() {
                return ("failed".to_string(), "missing_app_id".to_string());
            }
            match launcher_call_map("RestartApp", &app_id).await {
                Ok(payload) => {
                    let status = payload.get("status").cloned().unwrap_or_default();
                    if status == "launched" || status == "already_running" {
                        ("ok".to_string(), format!("app_id={app_id} status={status}"))
                    } else {
                        (
                            "failed".to_string(),
                            format!(
                                "app_id={} status={} reason={}",
                                app_id,
                                status,
                                payload.get("reason").cloned().unwrap_or_default()
                            ),
                        )
                    }
                }
                Err(err) => ("failed".to_string(), err),
            }
        }
        "launch_app" => {
            let app_id = rule
                .action_payload
                .get("app_id")
                .cloned()
                .or_else(|| event.payload.get("app_id").cloned())
                .unwrap_or_default();
            if app_id.is_empty() {
                return ("failed".to_string(), "missing_app_id".to_string());
            }
            match launcher_call_map("Launch", &app_id).await {
                Ok(payload) => {
                    let status = payload.get("status").cloned().unwrap_or_default();
                    if status == "launched" || status == "already_running" || status == "prompt" {
                        let result = if status == "prompt" { "degraded" } else { "ok" };
                        (result.to_string(), format!("app_id={app_id} status={status}"))
                    } else {
                        (
                            "failed".to_string(),
                            format!(
                                "app_id={} status={} reason={}",
                                app_id,
                                status,
                                payload.get("reason").cloned().unwrap_or_default()
                            ),
                        )
                    }
                }
                Err(err) => ("failed".to_string(), err),
            }
        }
        "stop_app" => {
            let app_id = rule
                .action_payload
                .get("app_id")
                .cloned()
                .or_else(|| event.payload.get("app_id").cloned())
                .unwrap_or_default();
            if app_id.is_empty() {
                return ("failed".to_string(), "missing_app_id".to_string());
            }
            match launcher_call_map("StopApp", &app_id).await {
                Ok(payload) => {
                    let status = payload.get("status").cloned().unwrap_or_default();
                    if status == "stopping" || status == "not_running" {
                        ("ok".to_string(), format!("app_id={app_id} status={status}"))
                    } else {
                        (
                            "failed".to_string(),
                            format!(
                                "app_id={} status={} reason={}",
                                app_id,
                                status,
                                payload.get("reason").cloned().unwrap_or_default()
                            ),
                        )
                    }
                }
                Err(err) => ("failed".to_string(), err),
            }
        }
        "request_recovery_space" => match activate_space(base_dir, "recovery") {
            Ok(_) => ("ok".to_string(), "space_id=recovery".to_string()),
            Err(err) => ("failed".to_string(), err),
        },
        "write_audit" => {
            let message = rule
                .action_payload
                .get("message")
                .cloned()
                .unwrap_or_else(|| event.summary.clone());
            let _ = audit.log_transition(
                "rule_action_audit",
                "",
                "",
                user_id,
                &rule.rule_id,
                "ok",
                &message,
            );
            ("ok".to_string(), message)
        }
        "mark_alert" => {
            let message = rule
                .action_payload
                .get("message")
                .cloned()
                .unwrap_or_else(|| event.summary.clone());
            log_rules(base_dir, "rule_alert_marked", "ok", &message);
            ("ok".to_string(), message)
        }
        _ => ("failed".to_string(), "unsupported_action".to_string()),
    }
}

pub async fn evaluate_rules(
    base_dir: &Path,
    audit: &SessionAuditLogger,
    user_id: &str,
    previous_snapshot: &SessionSnapshot,
    current_snapshot: &SessionSnapshot,
) {
    let mut registry = load_rules_registry(base_dir);
    let mut state = load_rules_state(base_dir);
    let update_state = read_update_state(base_dir);
    let intent_state = read_intent_state(base_dir);
    let permission_snapshot = parse_permission_snapshot(base_dir);
    let mut events = Vec::new();

    if previous_snapshot.current_state.as_str() != current_snapshot.current_state.as_str() {
        let trigger = match current_snapshot.current_state.as_str() {
            "ready" => Some("session_ready"),
            "degraded" => Some("session_degraded"),
            "failed" => Some("session_failed"),
            _ => None,
        };
        if let Some(trigger_type) = trigger {
            events.push(make_event(
                trigger_type,
                HashMap::from([(
                    "session_state".to_string(),
                    current_snapshot.current_state.as_str().to_string(),
                )]),
                format!(
                    "session state {} -> {}",
                    previous_snapshot.current_state.as_str(),
                    current_snapshot.current_state.as_str()
                ),
            ));
        }
    }

    if previous_snapshot.active_space_id != current_snapshot.active_space_id
        && current_snapshot.active_space_id.is_some()
    {
        let space_id = current_snapshot.active_space_id.clone().unwrap_or_default();
        events.push(make_event(
            "space_activated",
            HashMap::from([("space_id".to_string(), space_id.clone())]),
            format!("space activated {}", space_id),
        ));
    }

    let previous_apps = previous_snapshot
        .apps
        .iter()
        .map(|app| (app.app_id.clone(), app))
        .collect::<HashMap<_, _>>();
    for app in &current_snapshot.apps {
        let previous_state = previous_apps
            .get(&app.app_id)
            .map(|previous| previous.state.as_str())
            .unwrap_or("idle");
        if previous_state != app.state {
            if app.state == "failed" {
                events.push(make_event(
                    "app_runtime_failed",
                    HashMap::from([
                        ("app_id".to_string(), app.app_id.clone()),
                        ("app_required".to_string(), app.required.to_string()),
                        ("app_autostart".to_string(), app.autostart.to_string()),
                        ("in_active_space".to_string(), app.in_active_space.to_string()),
                    ]),
                    format!("app {} entered failed state", app.app_id),
                ));
            } else if app.state == "exited" {
                events.push(make_event(
                    "app_runtime_exited",
                    HashMap::from([
                        ("app_id".to_string(), app.app_id.clone()),
                        ("app_required".to_string(), app.required.to_string()),
                        ("app_autostart".to_string(), app.autostart.to_string()),
                        ("in_active_space".to_string(), app.in_active_space.to_string()),
                    ]),
                    format!("app {} exited", app.app_id),
                ));
            }
        }
    }

    let update_key = format!("{}:{}", update_state.update_state, update_state.last_update_result);
    if !update_state.last_update_result.is_empty() && update_key != state.last_seen_update_key {
        let success_results = ["update_committed", "install_ok", "rollback_ok"];
        let failed_results = [
            "payload_invalid",
            "update_apply_failed",
            "update_healthcheck_failed",
            "rollback_failed",
        ];
        if success_results.contains(&update_state.last_update_result.as_str()) {
            events.push(make_event(
                "update_succeeded",
                HashMap::from([(
                    "update_result".to_string(),
                    update_state.last_update_result.clone(),
                )]),
                format!("update result {}", update_state.last_update_result),
            ));
        } else if update_state.update_state == "failed"
            || failed_results.contains(&update_state.last_update_result.as_str())
        {
            events.push(make_event(
                "update_failed",
                HashMap::from([(
                    "update_result".to_string(),
                    update_state.last_update_result.clone(),
                )]),
                format!("update failed {}", update_state.last_update_result),
            ));
        }
    }
    if update_state.recovery_needed && !state.last_seen_recovery_needed {
        events.push(make_event(
            "recovery_needed",
            HashMap::from([("recovery_needed".to_string(), "true".to_string())]),
            "recovery needed marker raised".to_string(),
        ));
    }

    if !intent_state.last_run_at.is_empty() && intent_state.last_run_at != state.last_seen_intent_run_at {
        events.push(make_event(
            "intent_ran",
            HashMap::from([
                ("intent_id".to_string(), intent_state.last_intent_id.clone()),
                ("intent_result".to_string(), intent_state.last_result.clone()),
                ("space_id".to_string(), intent_state.last_space_id.clone()),
            ]),
            format!("intent ran {}", intent_state.last_intent_id),
        ));
    }

    if !permission_snapshot.event_key.is_empty()
        && permission_snapshot.event_key != state.last_seen_permission_event
    {
        let mut payload = HashMap::new();
        payload.insert("app_id".to_string(), permission_snapshot.app_id.clone());
        payload.insert("message".to_string(), permission_snapshot.message.clone());
        events.push(make_event(
            &permission_snapshot.trigger_type,
            payload,
            permission_snapshot.message.clone(),
        ));
    }

    for event in &events {
        for index in 0..registry.rules.len() {
            let rule = registry.rules[index].clone();
            if !rule.enabled || rule.trigger_type != event.trigger_type {
                continue;
            }

            log_rules(
                base_dir,
                "rule_evaluate_begin",
                "started",
                &format!("rule_id={} trigger_type={}", rule.rule_id, event.trigger_type),
            );

            let context = build_context(current_snapshot, &update_state, &intent_state, event);
            if let Some(reason) = matches_conditions(&rule, &context) {
                log_rules(
                    base_dir,
                    "rule_condition_skip",
                    "skipped",
                    &format!("rule_id={} {}", rule.rule_id, reason),
                );
                state.last_rule_id = rule.rule_id.clone();
                state.last_result = "skipped".to_string();
                state.last_trigger_type = event.trigger_type.clone();
                state.last_action_type = rule.action_type.clone();
                state.last_run_at = now();
                state.last_message = reason;
                continue;
            }

            if cooldown_active(&rule) {
                log_rules(
                    base_dir,
                    "rule_cooldown_skip",
                    "cooldown",
                    &format!("rule_id={} cooldown_seconds={}", rule.rule_id, rule.cooldown_seconds),
                );
                state.last_rule_id = rule.rule_id.clone();
                state.last_result = "cooldown".to_string();
                state.last_trigger_type = event.trigger_type.clone();
                state.last_action_type = rule.action_type.clone();
                state.last_run_at = now();
                state.last_message = format!("cooldown {}s", rule.cooldown_seconds);
                continue;
            }

            log_rules(
                base_dir,
                "rule_action_begin",
                "started",
                &format!(
                    "rule_id={} action_type={} trigger_type={}",
                    rule.rule_id, rule.action_type, event.trigger_type
                ),
            );
            let (result, message) =
                execute_action(base_dir, audit, user_id, current_snapshot, &rule, event).await;
            let log_event = if result == "ok" || result == "degraded" {
                "rule_action_ok"
            } else {
                "rule_action_failed"
            };
            log_rules(
                base_dir,
                log_event,
                &result,
                &format!("rule_id={} {}", rule.rule_id, message),
            );
            registry.rules[index].last_triggered_at = Some(now());
            registry.rules[index].updated_at = now();
            state.last_rule_id = rule.rule_id.clone();
            state.last_result = result;
            state.last_trigger_type = event.trigger_type.clone();
            state.last_action_type = rule.action_type.clone();
            state.last_run_at = now();
            state.last_message = message;
        }
    }

    state.last_seen_update_key = update_key;
    state.last_seen_recovery_needed = update_state.recovery_needed;
    state.last_seen_intent_run_at = intent_state.last_run_at;
    state.last_seen_permission_event = permission_snapshot.event_key;

    let _ = save_registry(base_dir, &registry);
    let _ = save_state(base_dir, &state);
}
