use crate::model::{ExecutionMode, PrivacyMode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub user_id: String,
    pub active_app: Option<String>,
    pub current_window: Option<String>,
    pub current_workspace: Option<String>,
    pub recent_actions: Vec<String>,
    pub granted_ai_scopes: Vec<String>,
    pub privacy_mode: PrivacyMode,
    pub execution_mode: ExecutionMode,
}
