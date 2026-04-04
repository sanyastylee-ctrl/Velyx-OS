use crate::model::ExplainResult;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ExplainState {
    pub app_id: String,
    pub last_summary: String,
    pub last_reason: String,
    pub last_source: String,
    pub suggested_action: Option<String>,
}

#[derive(Default)]
pub struct ExplainStore {
    entries: HashMap<String, ExplainState>,
    last_global: Option<ExplainState>,
}

impl ExplainStore {
    pub fn update_for_app(&mut self, app_id: &str, state: ExplainState) {
        self.entries.insert(app_id.to_string(), state.clone());
        self.last_global = Some(state);
    }

    pub fn last_for_app(&self, app_id: &str) -> Option<ExplainState> {
        self.entries.get(app_id).cloned().or_else(|| self.last_global.clone())
    }
}

pub fn build_explain_result(
    state: Option<ExplainState>,
    permission_status: &str,
) -> ExplainResult {
    if let Some(state) = state {
        if permission_status == "deny" && state.app_id == "com.velyx.browser" {
            return ExplainResult {
                summary: "Запуск был заблокирован, потому что приложению запрещён доступ к файловой системе.".to_string(),
                reason: "permissions-service: filesystem=deny".to_string(),
                source: "permissions".to_string(),
                suggested_action: Some("разреши браузеру доступ к файлам".to_string()),
            };
        }

        return ExplainResult {
            summary: state.last_summary,
            reason: state.last_reason,
            source: state.last_source,
            suggested_action: state.suggested_action,
        };
    }

    ExplainResult {
        summary: "Система не нашла недавнего события, которое нужно объяснить.".to_string(),
        reason: "Недостаточно контекста explainability".to_string(),
        source: "system".to_string(),
        suggested_action: None,
    }
}
