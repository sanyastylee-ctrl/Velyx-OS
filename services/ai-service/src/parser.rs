use crate::intent::{Intent, IntentKind};
use std::collections::HashMap;

pub fn parse_input(input: &str) -> Intent {
    let lower = input.trim().to_lowercase();

    if lower.contains("открой браузер") || lower.contains("запусти браузер") {
        let mut entities = HashMap::new();
        entities.insert("app_id".to_string(), "com.velyx.browser".to_string());
        entities.insert("app_name".to_string(), "Браузер".to_string());
        return Intent {
            kind: IntentKind::LaunchApp,
            entities,
            confidence: 0.94,
        };
    }

    if lower.contains("bluetooth") {
        let mut entities = HashMap::new();
        entities.insert("key".to_string(), "bluetooth.enabled".to_string());
        entities.insert("display_name".to_string(), "Bluetooth".to_string());
        return Intent {
            kind: IntentKind::ReadSetting,
            entities,
            confidence: 0.91,
        };
    }

    if lower.contains("какая тема") || lower.contains("тема системы") {
        let mut entities = HashMap::new();
        entities.insert("key".to_string(), "appearance.theme".to_string());
        entities.insert("display_name".to_string(), "Тема".to_string());
        return Intent {
            kind: IntentKind::ReadSetting,
            entities,
            confidence: 0.9,
        };
    }

    if (lower.contains("ai") || lower.contains("ии")) && (lower.contains("включен") || lower.contains("включён")) {
        let mut entities = HashMap::new();
        entities.insert("key".to_string(), "ai.enabled".to_string());
        entities.insert("display_name".to_string(), "AI Layer".to_string());
        return Intent {
            kind: IntentKind::ReadSetting,
            entities,
            confidence: 0.9,
        };
    }

    if lower.contains("режим приватности ai") || lower.contains("режим приватности ии") {
        let mut entities = HashMap::new();
        entities.insert("key".to_string(), "ai.privacy_mode".to_string());
        entities.insert("display_name".to_string(), "AI Privacy Mode".to_string());
        return Intent {
            kind: IntentKind::ReadSetting,
            entities,
            confidence: 0.9,
        };
    }

    if lower.contains("недавн") && (lower.contains("документ") || lower.contains("файл")) {
        let mut entities = HashMap::new();
        entities.insert("mode".to_string(), "recent".to_string());
        entities.insert("query".to_string(), "".to_string());
        return Intent {
            kind: IntentKind::SearchFiles,
            entities,
            confidence: 0.9,
        };
    }

    if lower.contains("найди") && (lower.contains("файл") || lower.contains("pdf") || lower.contains("документ")) {
        let mut entities = HashMap::new();
        let query = if lower.contains("pdf") {
            "*.pdf".to_string()
        } else if let Some(rest) = lower.split("найди").nth(1) {
            rest.trim().to_string()
        } else {
            String::new()
        };
        entities.insert("mode".to_string(), "search".to_string());
        entities.insert("query".to_string(), query);
        return Intent {
            kind: IntentKind::SearchFiles,
            entities,
            confidence: 0.86,
        };
    }

    if lower.contains("тормозит")
        || lower.contains("почему тормозит")
        || lower.contains("почему система тормозит")
        || lower.contains("что тормозит систему")
        || lower.contains("что с памятью")
        || lower.contains("как чувствует себя система")
    {
        let mut entities = HashMap::new();
        entities.insert("scope".to_string(), "system".to_string());
        return Intent {
            kind: IntentKind::DiagnosticsExplain,
            entities,
            confidence: 0.88,
        };
    }

    if lower.contains("почему не запуст") || lower.contains("почему доступ запрещ") || lower.contains("что произошло") || lower.contains("почему запросило") {
        let mut entities = HashMap::new();
        if lower.contains("браузер") {
            entities.insert("app_id".to_string(), "com.velyx.browser".to_string());
        }
        if lower.contains("обновлен") || lower.contains("rollback") || lower.contains("восстанов") {
            entities.insert("scope".to_string(), "update_recovery".to_string());
        }
        return Intent {
            kind: IntentKind::SecurityExplain,
            entities,
            confidence: 0.87,
        };
    }

    if lower.contains("запрети") && lower.contains("доступ") && lower.contains("файл") {
        let mut entities = HashMap::new();
        let app_id = if lower.contains("браузер") {
            "com.velyx.browser"
        } else {
            ""
        };
        entities.insert("app_id".to_string(), app_id.to_string());
        entities.insert("permission".to_string(), "filesystem".to_string());
        entities.insert("decision".to_string(), "deny".to_string());
        return Intent {
            kind: IntentKind::UpdatePermissions,
            entities,
            confidence: if app_id.is_empty() { 0.45 } else { 0.9 },
        };
    }

    Intent {
        kind: IntentKind::Unknown,
        entities: HashMap::new(),
        confidence: 0.24,
    }
}
