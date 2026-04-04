use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IntentKind {
    LaunchApp,
    ReadSetting,
    SearchFiles,
    DiagnosticsExplain,
    UpdatePermissions,
    SecurityExplain,
    Unknown,
}

impl IntentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LaunchApp => "LaunchApp",
            Self::ReadSetting => "ReadSetting",
            Self::SearchFiles => "SearchFiles",
            Self::DiagnosticsExplain => "DiagnosticsExplain",
            Self::UpdatePermissions => "UpdatePermissions",
            Self::SecurityExplain => "SecurityExplain",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Intent {
    pub kind: IntentKind,
    pub entities: HashMap<String, String>,
    pub confidence: f32,
}
