#[derive(Debug)]
pub enum DiagnosticsError {
    Io(String),
    Parse(String),
}

impl DiagnosticsError {
    pub fn message(&self) -> String {
        match self {
            Self::Io(reason) => format!("io error: {reason}"),
            Self::Parse(reason) => format!("parse error: {reason}"),
        }
    }
}
