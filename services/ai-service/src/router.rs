use crate::intent::{IntentKind, IntentResolution};
use std::collections::HashMap;

pub trait AIBackend: Send + Sync {
    fn generate_intent(&self, input: &str) -> IntentResolution;
    fn propose_tool_plan(&self, intent: &IntentResolution) -> Vec<String>;
    fn summarize_result(&self, tool_output: &str) -> String;
}

pub struct MockBackend;

impl AIBackend for MockBackend {
    fn generate_intent(&self, input: &str) -> IntentResolution {
        let lower = input.to_lowercase();
        let (intent_kind, tools, clarification) = if lower.contains("открой") || lower.contains("запусти") {
            (IntentKind::LaunchApp, vec!["app.launch".to_string()], None)
        } else if lower.contains("запрети") && lower.contains("доступ") {
            (IntentKind::UpdatePermissions, vec!["permissions.update".to_string()], None)
        } else if lower.contains("тормозит") || lower.contains("памяти") {
            (IntentKind::DiagnosticsExplain, vec!["diagnostics.summary".to_string()], None)
        } else if lower.contains("файл") || lower.contains("pdf") {
            (IntentKind::SearchFiles, vec!["files.search".to_string()], None)
        } else {
            (
                IntentKind::Unknown,
                Vec::new(),
                Some("Уточните, какое действие вы хотите выполнить.".to_string()),
            )
        };

        IntentResolution {
            intent_id: format!("intent-{}", input.len()),
            intent_kind,
            confidence: if clarification.is_some() { 0.32 } else { 0.84 },
            extracted_entities: HashMap::new(),
            requires_clarification: clarification.is_some(),
            clarification_question: clarification,
            proposed_tools: tools,
        }
    }

    fn propose_tool_plan(&self, intent: &IntentResolution) -> Vec<String> {
        intent.proposed_tools.clone()
    }

    fn summarize_result(&self, tool_output: &str) -> String {
        format!("AI завершил обработку запроса: {tool_output}")
    }
}
