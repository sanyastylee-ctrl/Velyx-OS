use crate::errors::SessionManagerError;
use crate::model::ShellRuntime;
use crate::units::{is_active, main_pid, start_unit};
use chrono::Utc;

const SHELL_UNIT: &str = "velyx-shell.service";

pub async fn start_shell_process() -> Result<ShellRuntime, SessionManagerError> {
    start_unit(SHELL_UNIT).await?;
    build_shell_runtime().await
}

pub async fn verify_shell_started(shell: &ShellRuntime) -> Result<(), SessionManagerError> {
    let status = is_active(SHELL_UNIT).await?;
    if status.trim() == "active" && shell.shell_pid.is_some() {
        Ok(())
    } else {
        Err(SessionManagerError::ShellLaunchFailed(format!(
            "shell unit '{}' не стал active, status={}",
            SHELL_UNIT, status
        )))
    }
}

pub async fn build_shell_runtime() -> Result<ShellRuntime, SessionManagerError> {
    let pid = main_pid(SHELL_UNIT).await?;
    let shell_state = match is_active(SHELL_UNIT).await {
        Ok(status) => status,
        Err(_) => "unknown".to_string(),
    };
    Ok(ShellRuntime {
        shell_pid: pid,
        shell_started_at: Some(Utc::now().to_rfc3339()),
        shell_state,
    })
}
