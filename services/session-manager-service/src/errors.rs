#[derive(Debug)]
pub enum SessionManagerError {
    StartupFailed(String),
    ServiceTimeout(String),
    ShellLaunchFailed(String),
    FirstBootFailed(String),
    SystemdUnavailable(String),
}

impl SessionManagerError {
    pub fn message(&self) -> String {
        match self {
            Self::StartupFailed(reason) => reason.clone(),
            Self::ServiceTimeout(reason) => reason.clone(),
            Self::ShellLaunchFailed(reason) => reason.clone(),
            Self::FirstBootFailed(reason) => reason.clone(),
            Self::SystemdUnavailable(reason) => reason.clone(),
        }
    }
}
