#[derive(Debug)]
pub enum RecoveryError {
    Store(String),
    UnknownSnapshot(String),
    SnapshotLinkage(String),
}

impl RecoveryError {
    pub fn message(&self) -> String {
        match self {
            Self::Store(message) | Self::UnknownSnapshot(message) | Self::SnapshotLinkage(message) => {
                message.clone()
            }
        }
    }
}
