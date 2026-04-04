use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionKind {
    Camera,
    Microphone,
    Filesystem,
    ScreenCapture,
}

impl PermissionKind {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "camera" => Some(Self::Camera),
            "microphone" => Some(Self::Microphone),
            "filesystem" => Some(Self::Filesystem),
            "screen_capture" => Some(Self::ScreenCapture),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Camera => "Доступ к камере",
            Self::Microphone => "Доступ к микрофону",
            Self::Filesystem => "Доступ к файлам",
            Self::ScreenCapture => "Захват экрана",
        }
    }

    pub fn explanation(&self) -> &'static str {
        match self {
            Self::Camera => "Приложению требуется доступ к камере для работы со съемкой и видеосвязью.",
            Self::Microphone => "Приложению требуется доступ к микрофону для записи звука и звонков.",
            Self::Filesystem => "Приложению требуется доступ к файлам пользователя для открытия и сохранения данных.",
            Self::ScreenCapture => "Приложению требуется доступ к захвату экрана. Для MVP это пока заглушка сценария.",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::Microphone => "microphone",
            Self::Filesystem => "filesystem",
            Self::ScreenCapture => "screen_capture",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decision {
    Allow,
    Deny,
}

impl Decision {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "allow" => Some(Self::Allow),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }

    pub fn as_status(&self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckResult {
    Allow,
    Deny,
    Prompt,
}

impl Display for CheckResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Deny => write!(f, "deny"),
            Self::Prompt => write!(f, "prompt"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustLevel {
    System,
    Trusted,
    Unknown,
}

impl TrustLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Trusted => "trusted",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyDecisionSource {
    Store,
    Policy,
    Default,
}

impl PolicyDecisionSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Store => "store",
            Self::Policy => "policy",
            Self::Default => "default",
        }
    }
}
