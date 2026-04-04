use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    SafeWrite,
    SensitiveWrite,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SafeWrite => "safe_write",
            Self::SensitiveWrite => "sensitive_write",
        }
    }
}

#[derive(Clone, Debug)]
pub struct SettingMetadata {
    pub key: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub value_type: &'static str,
    pub allowed_values: &'static [&'static str],
    pub default_value: &'static str,
    pub risk_level: RiskLevel,
    pub requires_confirmation: bool,
}
