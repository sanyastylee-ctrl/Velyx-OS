use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    SafeRead,
    SafeWrite,
    SensitiveRead,
    SensitiveWrite,
    Restricted,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SafeRead => "safe_read",
            Self::SafeWrite => "safe_write",
            Self::SensitiveRead => "sensitive_read",
            Self::SensitiveWrite => "sensitive_write",
            Self::Restricted => "restricted",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExecutionMode {
    ReadOnly,
    AskBeforeAct,
    SafeAuto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrivacyMode {
    LocalOnly,
    Hybrid,
    CloudDisabled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub capability_group: String,
    pub risk_level: RiskLevel,
    pub requires_confirmation: bool,
    pub required_permissions: Vec<String>,
    pub input_schema: String,
    pub output_schema: String,
    pub downstream_service: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolExecutionRequest {
    pub session_id: String,
    pub user_id: String,
    pub intent_id: String,
    pub tool_id: String,
    pub arguments: HashMap<String, String>,
    pub justification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    pub status: String,
    pub output: String,
    pub side_effects: Vec<String>,
    pub audit_ref: String,
    pub requires_user_confirmation: bool,
    pub error: Option<String>,
    pub downstream_service: String,
    pub service_result: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmationRequest {
    pub action_id: String,
    pub summary: String,
    pub detailed_reason: String,
    pub tool_id: String,
    pub risk_level: RiskLevel,
    pub affected_app: String,
    pub affected_permission: String,
    pub impacted_resources: Vec<String>,
    pub explicit_user_choice_required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub session_id: String,
    pub user_id: String,
    pub prompt_summary: String,
    pub resolved_intent: String,
    pub selected_tool: String,
    pub arguments_summary: String,
    pub risk_level: String,
    pub confirmation_required: bool,
    pub confirmation_result: String,
    pub execution_result: String,
    pub downstream_service: String,
    pub service_result: String,
    pub policy_decision: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplainResult {
    pub summary: String,
    pub reason: String,
    pub source: String,
    pub suggested_action: Option<String>,
}
