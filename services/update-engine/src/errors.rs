#[derive(Debug)]
pub enum UpdateEngineError {
    Store(String),
    UnknownUpdate(String),
    SignatureDenied(String),
    RecoveryRegistration(String),
    ApplyFailed(String),
    VerificationFailed(String),
}

impl UpdateEngineError {
    pub fn message(&self) -> String {
        match self {
            Self::Store(message)
            | Self::UnknownUpdate(message)
            | Self::SignatureDenied(message)
            | Self::RecoveryRegistration(message)
            | Self::ApplyFailed(message)
            | Self::VerificationFailed(message) => message.clone(),
        }
    }
}
