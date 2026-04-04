#[derive(Debug)]
pub enum SettingsError {
    UnknownKey(String),
    InvalidValue(String),
    StoreUnavailable(String),
}

impl SettingsError {
    pub fn message(&self) -> String {
        match self {
            Self::UnknownKey(key) => format!("unknown key: {key}"),
            Self::InvalidValue(reason) => format!("invalid value: {reason}"),
            Self::StoreUnavailable(reason) => format!("store unavailable: {reason}"),
        }
    }
}
