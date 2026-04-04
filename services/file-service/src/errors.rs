#[derive(Debug)]
pub enum FileServiceError {
    InvalidPath(String),
    AccessDenied(String),
}

impl FileServiceError {
    pub fn message(self) -> String {
        match self {
            Self::InvalidPath(message) => message,
            Self::AccessDenied(message) => message,
        }
    }
}
