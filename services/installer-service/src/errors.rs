#[derive(Debug)]
pub enum InstallerError {
    MissingPlan,
    PersistFailure(String),
}

impl InstallerError {
    pub fn message(&self) -> String {
        match self {
            Self::MissingPlan => "install plan is missing".to_string(),
            Self::PersistFailure(reason) => reason.clone(),
        }
    }
}
