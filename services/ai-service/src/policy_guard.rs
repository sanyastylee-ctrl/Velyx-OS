use crate::context::SessionContext;
use crate::errors::AiServiceError;
use crate::model::{RiskLevel, ToolDefinition, ToolExecutionRequest};

pub enum PolicyDecision {
    Allow,
    RequiresConfirmation,
    Deny(String),
}

pub fn evaluate(
    tool: Option<&ToolDefinition>,
    request: &ToolExecutionRequest,
    session: &SessionContext,
) -> Result<PolicyDecision, AiServiceError> {
    let tool = tool.ok_or_else(|| AiServiceError::ToolNotFound(request.tool_id.clone()))?;

    if session.session_id != request.session_id || session.user_id != request.user_id {
        return Ok(PolicyDecision::Deny("session mismatch".to_string()));
    }

    if matches!(tool.risk_level, RiskLevel::Restricted) {
        return Ok(PolicyDecision::Deny(
            "restricted tool denied by default".to_string(),
        ));
    }

    if tool.id == "permissions.update" {
        let app_id = request.arguments.get("app_id").cloned().unwrap_or_default();
        if app_id.is_empty() {
            return Ok(PolicyDecision::Deny("unknown app_id".to_string()));
        }
        return Ok(PolicyDecision::RequiresConfirmation);
    }

    if tool.id == "security.explain" || tool.id == "files.search" {
        return Ok(PolicyDecision::Allow);
    }

    Ok(PolicyDecision::Allow)
}
