use crate::model::{RiskLevel, SettingMetadata};

pub const SETTINGS_SCHEMA: &[SettingMetadata] = &[
    SettingMetadata {
        key: "appearance.theme",
        display_name: "Тема",
        description: "Системная тема интерфейса.",
        value_type: "enum",
        allowed_values: &["light", "dark"],
        default_value: "dark",
        risk_level: RiskLevel::SafeWrite,
        requires_confirmation: false,
    },
    SettingMetadata {
        key: "system.locale",
        display_name: "Язык системы",
        description: "Текущая локаль пользователя.",
        value_type: "enum",
        allowed_values: &["ru_RU", "en_US"],
        default_value: "ru_RU",
        risk_level: RiskLevel::SafeWrite,
        requires_confirmation: false,
    },
    SettingMetadata {
        key: "bluetooth.enabled",
        display_name: "Bluetooth",
        description: "Включен ли Bluetooth адаптер.",
        value_type: "bool",
        allowed_values: &["true", "false"],
        default_value: "true",
        risk_level: RiskLevel::SafeWrite,
        requires_confirmation: false,
    },
    SettingMetadata {
        key: "ai.enabled",
        display_name: "AI Layer",
        description: "Включен ли системный AI-слой.",
        value_type: "bool",
        allowed_values: &["true", "false"],
        default_value: "true",
        risk_level: RiskLevel::SensitiveWrite,
        requires_confirmation: true,
    },
    SettingMetadata {
        key: "ai.privacy_mode",
        display_name: "AI Privacy Mode",
        description: "Режим приватности AI.",
        value_type: "enum",
        allowed_values: &["local_only", "hybrid", "disabled"],
        default_value: "local_only",
        risk_level: RiskLevel::SensitiveWrite,
        requires_confirmation: true,
    },
];

pub fn metadata_for_key(key: &str) -> Option<&'static SettingMetadata> {
    SETTINGS_SCHEMA.iter().find(|item| item.key == key)
}
