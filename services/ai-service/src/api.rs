use crate::audit::AiAuditLogger;
use crate::context::SessionContext;
use crate::errors::AiServiceError;
use crate::explain::{ExplainState, ExplainStore};
use crate::intent::{Intent, IntentKind};
use crate::model::{AuditEntry, ConfirmationRequest, ExecutionMode, PrivacyMode, ToolDefinition, ToolExecutionRequest};
use crate::parser::parse_input;
use crate::pending::{PendingAction, PendingActionStore};
use crate::policy_guard::{evaluate, PolicyDecision};
use crate::session::SessionStore;
use crate::tool_executor;
use crate::tool_registry::default_tools;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

fn service_result_value(payload: &HashMap<String, String>) -> String {
    payload
        .get("service_result")
        .cloned()
        .unwrap_or_default()
}

pub struct AiServiceApi {
    tools: Vec<ToolDefinition>,
    sessions: Arc<Mutex<SessionStore>>,
    pending_actions: Arc<Mutex<PendingActionStore>>,
    explain_store: Arc<Mutex<ExplainStore>>,
    audit: AiAuditLogger,
}

impl AiServiceApi {
    pub fn new(audit: AiAuditLogger) -> Self {
        Self {
            tools: default_tools(),
            sessions: Arc::new(Mutex::new(SessionStore::default())),
            pending_actions: Arc::new(Mutex::new(PendingActionStore::default())),
            explain_store: Arc::new(Mutex::new(ExplainStore::default())),
            audit,
        }
    }

    fn tool_for_intent(&self, intent: &Intent) -> Option<ToolDefinition> {
        let tool_id = match intent.kind {
            IntentKind::LaunchApp => "app.launch",
            IntentKind::ReadSetting => "settings.get",
            IntentKind::SearchFiles => "files.search",
            IntentKind::DiagnosticsExplain => "diagnostics.summary",
            IntentKind::UpdatePermissions => "permissions.update",
            IntentKind::SecurityExplain => "security.explain",
            IntentKind::Unknown => return None,
        };

        self.tools.iter().find(|tool| tool.id == tool_id).cloned()
    }

    async fn ensure_session(&self, session_id: &str, user_id: &str) -> SessionContext {
        let mut sessions = self.sessions.lock().await;
        if let Some(existing) = sessions.get(session_id) {
            return existing;
        }

        let session = SessionContext {
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            active_app: Some("shell".to_string()),
            current_window: Some("AI overlay".to_string()),
            current_workspace: Some("desktop".to_string()),
            recent_actions: Vec::new(),
            granted_ai_scopes: vec!["shell.commands".to_string()],
            privacy_mode: PrivacyMode::LocalOnly,
            execution_mode: ExecutionMode::AskBeforeAct,
        };
        sessions.insert(session.clone());
        session
    }

    fn audit_entry(
        &self,
        session_id: &str,
        user_id: &str,
        prompt_summary: &str,
        resolved_intent: &str,
        selected_tool: &str,
        arguments_summary: &str,
        confirmation_required: bool,
        confirmation_result: &str,
        execution_result: &str,
        downstream_service: &str,
        service_result: &str,
        policy_decision: &str,
    ) -> AuditEntry {
        AuditEntry {
            timestamp: AiAuditLogger::now(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            prompt_summary: prompt_summary.to_string(),
            resolved_intent: resolved_intent.to_string(),
            selected_tool: selected_tool.to_string(),
            arguments_summary: arguments_summary.to_string(),
            risk_level: "vertical_slice_sensitive".to_string(),
            confirmation_required,
            confirmation_result: confirmation_result.to_string(),
            execution_result: execution_result.to_string(),
            downstream_service: downstream_service.to_string(),
            service_result: service_result.to_string(),
            policy_decision: policy_decision.to_string(),
        }
    }

    fn build_confirmation(request: &ToolExecutionRequest) -> ConfirmationRequest {
        ConfirmationRequest {
            action_id: format!("confirm-{}", request.intent_id),
            summary: "AI хочет изменить разрешения приложения".to_string(),
            detailed_reason: request.justification.clone(),
            tool_id: request.tool_id.clone(),
            risk_level: crate::model::RiskLevel::SensitiveWrite,
            affected_app: request.arguments.get("app_id").cloned().unwrap_or_default(),
            affected_permission: request
                .arguments
                .get("permission")
                .cloned()
                .unwrap_or_default(),
            impacted_resources: vec!["permissions-service".to_string()],
            explicit_user_choice_required: true,
        }
    }

    async fn remember_explain_state(
        &self,
        app_id: &str,
        summary: &str,
        reason: &str,
        source: &str,
        suggested_action: Option<String>,
    ) {
        let mut store = self.explain_store.lock().await;
        store.update_for_app(
            app_id,
            ExplainState {
                app_id: app_id.to_string(),
                last_summary: summary.to_string(),
                last_reason: reason.to_string(),
                last_source: source.to_string(),
                suggested_action,
            },
        );
    }
}

#[zbus::interface(name = "com.velyx.AI1")]
impl AiServiceApi {
    async fn process_command(
        &self,
        session_id: &str,
        user_id: &str,
        input: &str,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let session = self.ensure_session(session_id, user_id).await;
        let intent = parse_input(input);

        if matches!(intent.kind, IntentKind::Unknown) || intent.confidence < 0.6 {
            self.audit
                .log(self.audit_entry(
                    session_id,
                    user_id,
                    input,
                    intent.kind.as_str(),
                    "",
                    "",
                    false,
                    "not_needed",
                    "clarify",
                    "",
                    "not_called",
                    "unknown_intent",
                ))
                .map_err(zbus::fdo::Error::Failed)?;

            let mut payload: HashMap<String, String> = HashMap::new();
            payload.insert("status".to_string(), "clarify".to_string());
            payload.insert("intent".to_string(), intent.kind.as_str().to_string());
            payload.insert("tool".to_string(), "".to_string());
            payload.insert(
                "message".to_string(),
                "Я пока умею: открыть браузер, прочитать настройки, искать файлы по metadata, объяснить состояние системы и менять разрешения с подтверждением.".to_string(),
            );
            return Ok(payload);
        }

        let tool = self
            .tool_for_intent(&intent)
            .ok_or_else(|| zbus::fdo::Error::Failed("tool not found for intent".to_string()))?;

        let request = ToolExecutionRequest {
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            intent_id: format!("intent-{}", input.len()),
            tool_id: tool.id.clone(),
            arguments: intent.entities.clone(),
            justification: format!("AI распознал {}", intent.kind.as_str()),
        };

        if matches!(intent.kind, IntentKind::SecurityExplain) {
            self.audit
                .log(self.audit_entry(
                    session_id,
                    user_id,
                    input,
                    intent.kind.as_str(),
                    "security.explain",
                    &serde_json::to_string(&request.arguments).unwrap_or_default(),
                    false,
                    "not_needed",
                    "explain_requested",
                    "explainability-layer",
                    "pending",
                    "allowed",
                ))
                .map_err(zbus::fdo::Error::Failed)?;
        }

        match evaluate(Some(&tool), &request, &session)
            .map_err(|err: AiServiceError| zbus::fdo::Error::Failed(err.message()))?
        {
            PolicyDecision::Deny(reason) => {
                self.audit
                    .log(self.audit_entry(
                        session_id,
                        user_id,
                        input,
                        intent.kind.as_str(),
                        &tool.id,
                        &serde_json::to_string(&request.arguments).unwrap_or_default(),
                        false,
                        "not_requested",
                        "blocked",
                        &tool.downstream_service,
                        "not_called",
                        &reason,
                    ))
                    .map_err(zbus::fdo::Error::Failed)?;

                let mut payload: HashMap<String, String> = HashMap::new();
                payload.insert("status".to_string(), "blocked".to_string());
                payload.insert("intent".to_string(), intent.kind.as_str().to_string());
                payload.insert("tool".to_string(), tool.id);
                payload.insert("message".to_string(), reason);
                return Ok(payload);
            }
            PolicyDecision::RequiresConfirmation => {
                let confirmation = Self::build_confirmation(&request);
                let pending = PendingAction {
                    request: request.clone(),
                    confirmation: confirmation.clone(),
                };
                let mut pending_actions = self.pending_actions.lock().await;
                pending_actions.insert(pending);
                drop(pending_actions);

                self.audit
                    .log(self.audit_entry(
                        session_id,
                        user_id,
                        input,
                        intent.kind.as_str(),
                        &tool.id,
                        &serde_json::to_string(&request.arguments).unwrap_or_default(),
                        true,
                        "pending",
                        "confirmation_requested",
                        &tool.downstream_service,
                        "not_called",
                        "requires_confirmation",
                    ))
                    .map_err(zbus::fdo::Error::Failed)?;

                let mut payload: HashMap<String, String> = HashMap::new();
                payload.insert("status".to_string(), "confirmation_required".to_string());
                payload.insert("intent".to_string(), intent.kind.as_str().to_string());
                payload.insert("tool".to_string(), tool.id);
                payload.insert("action_id".to_string(), confirmation.action_id);
                payload.insert("summary".to_string(), confirmation.summary);
                payload.insert("details".to_string(), confirmation.detailed_reason);
                payload.insert("risk_level".to_string(), confirmation.risk_level.as_str().to_string());
                payload.insert("affected_app".to_string(), confirmation.affected_app);
                payload.insert("affected_permission".to_string(), confirmation.affected_permission);
                return Ok(payload);
            }
            PolicyDecision::Allow => {}
        }

        let result = tool_executor::execute(&tool, &request, Some(self.explain_store.clone()))
            .await
            .map_err(|err| zbus::fdo::Error::Failed(err.message()))?;

        if tool.id == "app.launch" {
            let app_id = request.arguments.get("app_id").cloned().unwrap_or_default();
            let source = if result.status == "deny" { "permissions" } else { "launcher" };
            let suggested_action = if result.status == "deny" {
                Some("разреши браузеру доступ к файлам".to_string())
            } else {
                None
            };
            self.remember_explain_state(&app_id, &result.output, &result.output, source, suggested_action)
                .await;
        } else if tool.id == "permissions.update" {
            let app_id = request.arguments.get("app_id").cloned().unwrap_or_default();
            self.remember_explain_state(
                &app_id,
                "Разрешения приложения были изменены.",
                &result.output,
                "user",
                None,
            )
            .await;
        } else if tool.id == "security.explain" {
            let app_id = request
                .arguments
                .get("app_id")
                .cloned()
                .unwrap_or_else(|| "com.velyx.browser".to_string());
            let service_result = result.service_result.clone();
            let mut source = "system".to_string();
            let mut reason = service_result.clone();
            let mut suggested_action = None;
            for part in service_result.split(';') {
                if let Some(value) = part.strip_prefix("source=") {
                    source = value.to_string();
                }
                if let Some(value) = part.strip_prefix("reason=") {
                    reason = value.to_string();
                }
                if let Some(value) = part.strip_prefix("suggested_action=") {
                    if !value.is_empty() {
                        suggested_action = Some(value.to_string());
                    }
                }
            }
            self.remember_explain_state(&app_id, &result.output, &reason, &source, suggested_action.clone())
                .await;
        }

        self.audit
            .log(self.audit_entry(
                session_id,
                user_id,
                input,
                intent.kind.as_str(),
                &tool.id,
                &serde_json::to_string(&request.arguments).unwrap_or_default(),
                false,
                "not_needed",
                if matches!(intent.kind, IntentKind::SecurityExplain) {
                    "explain_result"
                } else {
                    &result.status
                },
                &result.downstream_service,
                &result.service_result,
                "allowed",
            ))
            .map_err(zbus::fdo::Error::Failed)?;

        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("status".to_string(), result.status);
        payload.insert("intent".to_string(), intent.kind.as_str().to_string());
        payload.insert("tool".to_string(), tool.id);
        payload.insert("message".to_string(), result.output);
        payload.insert("downstream_service".to_string(), result.downstream_service);
        payload.insert("service_result".to_string(), result.service_result);
        if matches!(intent.kind, IntentKind::ReadSetting) {
            let service_result = service_result_value(&payload);
            for part in service_result.split(';') {
                if let Some(value) = part.strip_prefix("key=") {
                    payload.insert("key".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("value=") {
                    payload.insert("value".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("source=") {
                    payload.insert("source".to_string(), value.to_string());
                }
            }
        }
        if matches!(intent.kind, IntentKind::SearchFiles) {
            let service_result = service_result_value(&payload);
            for part in service_result.split(';') {
                if let Some(value) = part.strip_prefix("count=") {
                    payload.insert("count".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("mode=") {
                    payload.insert("mode".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("source=") {
                    payload.insert("source".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("first=") {
                    payload.insert("first_result".to_string(), value.to_string());
                }
            }
        }
        if matches!(intent.kind, IntentKind::DiagnosticsExplain) {
            let service_result = service_result_value(&payload);
            for part in service_result.split(';') {
                if let Some(value) = part.strip_prefix("cpu_state=") {
                    payload.insert("cpu_state".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("memory_state=") {
                    payload.insert("memory_state".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("hottest_component=") {
                    payload.insert("hottest_component".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("service_health=") {
                    payload.insert("service_health".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("source=") {
                    payload.insert("source".to_string(), value.to_string());
                }
                if let Some(value) = part.strip_prefix("suggested_action=") {
                    payload.insert("suggested_action".to_string(), value.to_string());
                }
            }
        }
        if matches!(intent.kind, IntentKind::SecurityExplain) {
            let service_result = service_result_value(&payload);
            let mut source = "system".to_string();
            let mut suggested_action = String::new();
            for part in service_result.split(';') {
                if let Some(value) = part.strip_prefix("source=") {
                    source = value.to_string();
                }
                if let Some(value) = part.strip_prefix("suggested_action=") {
                    suggested_action = value.to_string();
                }
            }
            payload.insert("source".to_string(), source);
            payload.insert("suggested_action".to_string(), suggested_action);
        }
        Ok(payload)
    }

    async fn confirm_action(
        &self,
        session_id: &str,
        user_id: &str,
        action_id: &str,
        decision: &str,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let _session = self.ensure_session(session_id, user_id).await;
        let mut pending_actions = self.pending_actions.lock().await;
        let Some(pending) = pending_actions.take(action_id) else {
            let mut payload: HashMap<String, String> = HashMap::new();
            payload.insert("status".to_string(), "ignored".to_string());
            payload.insert("message".to_string(), "confirmation action уже использован или не найден".to_string());
            return Ok(payload);
        };
        drop(pending_actions);

        if decision != "confirm" {
            self.audit
                .log(self.audit_entry(
                    session_id,
                    user_id,
                    &pending.request.justification,
                    &pending.request.tool_id,
                    &pending.request.tool_id,
                    &serde_json::to_string(&pending.request.arguments).unwrap_or_default(),
                    true,
                    "rejected",
                    "confirmation_rejected",
                    "permissions-service",
                    "not_called",
                    "user_canceled",
                ))
                .map_err(zbus::fdo::Error::Failed)?;

            let mut payload: HashMap<String, String> = HashMap::new();
            payload.insert("status".to_string(), "canceled".to_string());
            payload.insert("message".to_string(), "Пользователь отменил действие".to_string());
            return Ok(payload);
        }

        self.audit
            .log(self.audit_entry(
                session_id,
                user_id,
                &pending.request.justification,
                &pending.request.tool_id,
                &pending.request.tool_id,
                &serde_json::to_string(&pending.request.arguments).unwrap_or_default(),
                true,
                "accepted",
                "confirmation_accepted",
                "permissions-service",
                "pending_execution",
                "confirmed_by_user",
            ))
            .map_err(zbus::fdo::Error::Failed)?;

        let tool = self
            .tools
            .iter()
            .find(|tool| tool.id == pending.request.tool_id)
            .cloned()
            .ok_or_else(|| zbus::fdo::Error::Failed("tool not found for confirmation".to_string()))?;

        let result = tool_executor::execute(&tool, &pending.request, Some(self.explain_store.clone()))
            .await
            .map_err(|err| zbus::fdo::Error::Failed(err.message()))?;

        if pending.request.tool_id == "permissions.update" {
            let app_id = pending.request.arguments.get("app_id").cloned().unwrap_or_default();
            self.remember_explain_state(
                &app_id,
                "Доступ был изменён после подтверждения пользователя.",
                &result.output,
                "user",
                None,
            )
            .await;
        }

        self.audit
            .log(self.audit_entry(
                session_id,
                user_id,
                &pending.request.justification,
                &pending.request.tool_id,
                &pending.request.tool_id,
                &serde_json::to_string(&pending.request.arguments).unwrap_or_default(),
                true,
                "accepted",
                &result.status,
                &result.downstream_service,
                &result.service_result,
                "execution_after_confirmation",
            ))
            .map_err(zbus::fdo::Error::Failed)?;

        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("status".to_string(), result.status);
        payload.insert("intent".to_string(), "UpdatePermissions".to_string());
        payload.insert("tool".to_string(), pending.request.tool_id);
        payload.insert("message".to_string(), result.output);
        payload.insert("downstream_service".to_string(), result.downstream_service);
        payload.insert("service_result".to_string(), result.service_result);
        Ok(payload)
    }
}
