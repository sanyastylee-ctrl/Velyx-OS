use crate::errors::SessionManagerError;
use crate::model::ShellRuntime;
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

fn pid_file_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".velyx").join("primary-shell.pid")
}

fn primary_shell_pid() -> Option<u32> {
    let raw = fs::read_to_string(pid_file_path()).ok()?;
    let pid = raw.trim().parse::<u32>().ok()?;
    if PathBuf::from(format!("/proc/{pid}")).exists() {
        Some(pid)
    } else {
        None
    }
}

pub async fn start_shell_process() -> Result<ShellRuntime, SessionManagerError> {
    for _ in 0..50 {
        let shell = build_shell_runtime().await?;
        if shell.shell_pid.is_some() && shell.shell_state == "active" {
            return Ok(shell);
        }
        sleep(Duration::from_millis(100)).await;
    }
    Err(SessionManagerError::ShellLaunchFailed(
        "primary shell launcher on tty1 did not expose a running shell pid in time".to_string(),
    ))
}

pub async fn verify_shell_started(shell: &ShellRuntime) -> Result<(), SessionManagerError> {
    if shell.shell_state == "active" && shell.shell_pid.is_some() {
        Ok(())
    } else {
        Err(SessionManagerError::ShellLaunchFailed(format!(
            "primary shell did not become active, state={}",
            shell.shell_state
        )))
    }
}

pub async fn build_shell_runtime() -> Result<ShellRuntime, SessionManagerError> {
    let pid = primary_shell_pid();
    let shell_state = if pid.is_some() {
        "active".to_string()
    } else {
        "inactive".to_string()
    };
    Ok(ShellRuntime {
        shell_pid: pid,
        shell_started_at: Some(Utc::now().to_rfc3339()),
        shell_state,
    })
}
