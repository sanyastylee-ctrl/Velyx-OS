use crate::errors::AiServiceError;
use crate::explain::{build_explain_result, ExplainStore};
use crate::model::{ExplainResult, ToolDefinition, ToolExecutionRequest, ToolExecutionResult};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn launch_via_launcher(app_id: &str) -> Result<HashMap<String, String>, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Launcher",
        "/com/velyx/Launcher",
        "com.velyx.Launcher1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("launcher proxy init failed: {err}")))?;

    proxy
        .call("Launch", &(app_id))
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("launcher call failed: {err}")))
}

async fn update_permission(
    app_id: &str,
    permission: &str,
    decision: &str,
) -> Result<(), AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Permissions",
        "/com/velyx/Permissions",
        "com.velyx.Permissions1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("permissions proxy init failed: {err}")))?;

    let success: bool = proxy
        .call("StoreDecision", &(app_id, permission, decision))
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("permissions call failed: {err}")))?;

    if success {
        Ok(())
    } else {
        Err(AiServiceError::InvalidRequest(
            "permissions-service did not accept decision".to_string(),
        ))
    }
}

async fn get_setting_value(key: &str) -> Result<String, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Settings",
        "/com/velyx/Settings",
        "com.velyx.Settings1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("settings proxy init failed: {err}")))?;

    proxy
        .call("GetValue", &(key))
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("settings call failed: {err}")))
}

async fn check_permission_status(app_id: &str, permission: &str) -> Result<String, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Permissions",
        "/com/velyx/Permissions",
        "com.velyx.Permissions1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("permissions proxy init failed: {err}")))?;

    proxy
        .call("CheckPermission", &(app_id, permission))
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("permissions check failed: {err}")))
}

async fn get_diagnostics_summary() -> Result<HashMap<String, String>, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Diagnostics",
        "/com/velyx/Diagnostics",
        "com.velyx.Diagnostics1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("diagnostics proxy init failed: {err}")))?;

    proxy
        .call("GetSystemSummary", &())
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("diagnostics call failed: {err}")))
}

async fn search_files_via_service(mode: &str, query: &str) -> Result<Vec<HashMap<String, String>>, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.FileService",
        "/com/velyx/FileService",
        "com.velyx.FileService1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("file-service proxy init failed: {err}")))?;

    match mode {
        "recent" => proxy
            .call("ListRecentFiles", &())
            .await
            .map_err(|err| AiServiceError::InvalidRequest(format!("file-service recent call failed: {err}"))),
        _ => proxy
            .call("SearchFiles", &(query))
            .await
            .map_err(|err| AiServiceError::InvalidRequest(format!("file-service search call failed: {err}"))),
    }
}

async fn get_update_status() -> Result<HashMap<String, String>, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.UpdateEngine",
        "/com/velyx/UpdateEngine",
        "com.velyx.UpdateEngine1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("update-engine proxy init failed: {err}")))?;

    proxy
        .call("GetUpdateStatus", &())
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("update-engine call failed: {err}")))
}

async fn get_recovery_status() -> Result<HashMap<String, String>, AiServiceError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| AiServiceError::BackendUnavailable)?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Recovery",
        "/com/velyx/Recovery",
        "com.velyx.Recovery1",
    )
    .await
    .map_err(|err| AiServiceError::InvalidRequest(format!("recovery-service proxy init failed: {err}")))?;

    proxy
        .call("GetRecoveryStatus", &())
        .await
        .map_err(|err| AiServiceError::InvalidRequest(format!("recovery-service call failed: {err}")))
}

fn velyx_home() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".velyx")
}

fn last_log_line(file_name: &str) -> Option<String> {
    let path = velyx_home().join(file_name);
    let content = fs::read_to_string(path).ok()?;
    content
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
}

pub async fn execute(
    tool: &ToolDefinition,
    request: &ToolExecutionRequest,
    explain_store: Option<Arc<Mutex<ExplainStore>>>,
) -> Result<ToolExecutionResult, AiServiceError> {
    match tool.id.as_str() {
        "app.launch" => {
            let app_id = request
                .arguments
                .get("app_id")
                .cloned()
                .ok_or_else(|| AiServiceError::InvalidRequest("missing app_id".to_string()))?;
            let payload = launch_via_launcher(&app_id).await?;
            let status = payload
                .get("status")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            let message = payload
                .get("message")
                .cloned()
                .unwrap_or_else(|| "launcher did not return message".to_string());

            Ok(ToolExecutionResult {
                status,
                output: message,
                side_effects: vec!["application_launch".to_string()],
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: false,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: format!(
                    "launcher_called;profile={};trust_level={};launched_by={}",
                    payload.get("sandbox_profile").cloned().unwrap_or_default(),
                    payload.get("trust_level").cloned().unwrap_or_default(),
                    payload.get("launched_by").cloned().unwrap_or_default()
                ),
            })
        }
        "settings.get" => {
            let key = request
                .arguments
                .get("key")
                .cloned()
                .ok_or_else(|| AiServiceError::InvalidRequest("missing key".to_string()))?;
            let display_name = request
                .arguments
                .get("display_name")
                .cloned()
                .unwrap_or_else(|| key.clone());
            let value = get_setting_value(&key).await?;

            Ok(ToolExecutionResult {
                status: "ok".to_string(),
                output: format!("{display_name}: {value}"),
                side_effects: Vec::new(),
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: false,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: format!("key={};value={};source=settings-service", key, value),
            })
        }
        "files.search" => {
            let mode = request
                .arguments
                .get("mode")
                .cloned()
                .unwrap_or_else(|| "search".to_string());
            let query = request
                .arguments
                .get("query")
                .cloned()
                .unwrap_or_default();
            let results = search_files_via_service(&mode, &query).await?;
            let count = results.len();
            let first = results
                .first()
                .map(|entry| {
                    let name = entry.get("name").cloned().unwrap_or_default();
                    let path = entry.get("path").cloned().unwrap_or_default();
                    format!("{name} ({path})")
                })
                .unwrap_or_else(|| "ничего не найдено".to_string());

            Ok(ToolExecutionResult {
                status: "ok".to_string(),
                output: if mode == "recent" {
                    format!("Найдено недавних файлов: {}. Первый результат: {}", count, first)
                } else {
                    format!("Найдено файлов: {}. Первый результат: {}", count, first)
                },
                side_effects: Vec::new(),
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: false,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: format!(
                    "count={};mode={};source=file-service;first={}",
                    count, mode, first
                ),
            })
        }
        "diagnostics.summary" => {
            let summary = get_diagnostics_summary().await?;
            let human_summary = summary
                .get("human_summary")
                .cloned()
                .unwrap_or_else(|| "Диагностическая сводка недоступна".to_string());
            let suggested_action = if summary
                .get("memory_state")
                .map(|state| state == "high")
                .unwrap_or(false)
            {
                "Закройте тяжелые приложения или проверьте недавние запуски."
            } else if summary
                .get("cpu_state")
                .map(|state| state == "high")
                .unwrap_or(false)
            {
                "Проверьте, какие задачи были запущены недавно, и откройте диагностику подробнее."
            } else {
                "Система выглядит стабильной. Дополнительных действий не требуется."
            };
            let cpu_state = summary.get("cpu_state").cloned().unwrap_or_default();
            let memory_state = summary.get("memory_state").cloned().unwrap_or_default();
            let hottest_component = summary
                .get("hottest_component")
                .cloned()
                .unwrap_or_default();
            let service_health = summary
                .get("service_health")
                .cloned()
                .unwrap_or_default();

            Ok(ToolExecutionResult {
                status: "ok".to_string(),
                output: human_summary,
                side_effects: Vec::new(),
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: false,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: format!(
                    "cpu_state={};memory_state={};hottest_component={};service_health={};source=diagnostics-service;suggested_action={}",
                    cpu_state, memory_state, hottest_component, service_health, suggested_action
                ),
            })
        }
        "permissions.update" => {
            let app_id = request
                .arguments
                .get("app_id")
                .cloned()
                .ok_or_else(|| AiServiceError::InvalidRequest("missing app_id".to_string()))?;
            let permission = request
                .arguments
                .get("permission")
                .cloned()
                .ok_or_else(|| AiServiceError::InvalidRequest("missing permission".to_string()))?;
            let decision = request
                .arguments
                .get("decision")
                .cloned()
                .ok_or_else(|| AiServiceError::InvalidRequest("missing decision".to_string()))?;

            update_permission(&app_id, &permission, &decision).await?;

            Ok(ToolExecutionResult {
                status: "ok".to_string(),
                output: format!(
                    "Разрешение обновлено: app_id={}, permission={}, decision={}",
                    app_id, permission, decision
                ),
                side_effects: vec!["permissions_update".to_string()],
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: true,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: "permissions_store_decision_called".to_string(),
            })
        }
        "security.explain" => {
            let app_id = request
                .arguments
                .get("app_id")
                .cloned()
                .unwrap_or_else(|| "com.velyx.browser".to_string());
            let permission_status = check_permission_status(&app_id, "filesystem").await?;
            let diagnostics = get_diagnostics_summary().await.ok();
            let update_status = get_update_status().await.ok();
            let recovery_status = get_recovery_status().await.ok();
            let launcher_history = last_log_line("launcher_history.log");
            let ai_audit = last_log_line("ai_audit.log");

            let state = if let Some(store) = explain_store {
                let store = store.lock().await;
                store.last_for_app(&app_id)
            } else {
                None
            };
            let explain: ExplainResult = if permission_status == "deny" {
                ExplainResult {
                    summary: "Действие было ограничено, потому что доступ к файловой системе для приложения запрещен.".to_string(),
                    reason: "permissions-service: filesystem=deny".to_string(),
                    source: "permissions-service".to_string(),
                    suggested_action: Some("разреши браузеру доступ к файлам".to_string()),
                }
            } else if let Some(line) = launcher_history.clone() {
                if line.contains("decision=deny") || line.contains("permissions_deny") || line.contains("unknown_manifest") {
                    ExplainResult {
                        summary: "Запуск был остановлен в launcher-service до старта процесса.".to_string(),
                        reason: line,
                        source: "launcher-service".to_string(),
                        suggested_action: Some("проверь manifest приложения или его разрешения".to_string()),
                    }
                } else if let Some(update) = update_status.clone() {
                    if update
                        .get("current_state")
                        .map(|value| value == "rollback_required")
                        .unwrap_or(false)
                    {
                        ExplainResult {
                            summary: "Система перевела обновление в rollback-required после неудачной проверки состояния.".to_string(),
                            reason: format!(
                                "update-engine: rollback_reason={}",
                                update.get("rollback_reason").cloned().unwrap_or_default()
                            ),
                            source: "update-engine".to_string(),
                            suggested_action: Some("открой recovery flow и проверь доступные restore points".to_string()),
                        }
                    } else if let Some(recovery) = recovery_status.clone() {
                        if recovery
                            .get("current_state")
                            .map(|value| value == "failed")
                            .unwrap_or(false)
                        {
                            ExplainResult {
                                summary: "Сценарий восстановления завершился с ошибкой и требует явного recovery вмешательства.".to_string(),
                                reason: format!(
                                    "recovery-service: last_result={}",
                                    recovery.get("last_result").cloned().unwrap_or_default()
                                ),
                                source: "recovery-service".to_string(),
                                suggested_action: Some("проверь restore points и повтори rollback из recovery mode".to_string()),
                            }
                        } else if let Some(summary) = diagnostics {
                            ExplainResult {
                                summary: state
                                    .clone()
                                    .map(|entry| entry.last_summary)
                                    .unwrap_or_else(|| "AI собрал последнее объяснение на основе системных сервисов.".to_string()),
                                reason: format!(
                                    "launcher_history={}; diagnostics={}; ai_audit={}",
                                    line,
                                    summary.get("human_summary").cloned().unwrap_or_default(),
                                    ai_audit.unwrap_or_default()
                                ),
                                source: "composed-system-sources".to_string(),
                                suggested_action: state.and_then(|entry| entry.suggested_action),
                            }
                        } else {
                            build_explain_result(state, &permission_status)
                        }
                    } else {
                        build_explain_result(state, &permission_status)
                    }
                } else {
                    build_explain_result(state, &permission_status)
                }
            } else {
                build_explain_result(state, &permission_status)
            };

            Ok(ToolExecutionResult {
                status: "ok".to_string(),
                output: explain.summary.clone(),
                side_effects: Vec::new(),
                audit_ref: request.intent_id.clone(),
                requires_user_confirmation: false,
                error: None,
                downstream_service: tool.downstream_service.clone(),
                service_result: format!(
                    "reason={};source={};suggested_action={};permission_status={}",
                    explain.reason,
                    explain.source,
                    explain.suggested_action.clone().unwrap_or_default(),
                    permission_status
                ),
            })
        }
        _ => Err(AiServiceError::ToolNotFound(tool.id.clone())),
    }
}
