use crate::errors::SessionManagerError;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct UnitDefinition {
    pub unit_name: String,
    pub dbus_name: String,
    pub required: bool,
    pub startup_order: u32,
    pub restart_policy: String,
    pub contents: String,
}

fn env_exec(binary_env: &str, fallback: &str) -> String {
    std::env::var(binary_env).unwrap_or_else(|_| fallback.to_string())
}

fn service_unit(
    unit_name: &str,
    description: &str,
    dbus_name: &str,
    required: bool,
    startup_order: u32,
    restart_policy: &str,
    exec_start: &str,
    wanted_by: &str,
) -> UnitDefinition {
    UnitDefinition {
        unit_name: unit_name.to_string(),
        dbus_name: dbus_name.to_string(),
        required,
        startup_order,
        restart_policy: restart_policy.to_string(),
        contents: format!(
            "[Unit]\nDescription={description}\nPartOf=velyx-session.target\nAfter=graphical-session-pre.target dbus.service\n\n[Service]\nType=simple\nEnvironment=VELYX_USER_ID=%u\nEnvironment=HOME=%h\nExecStart={exec_start}\nRestart={restart_policy}\nRestartSec=1\n\n[Install]\nWantedBy={wanted_by}\n"
        ),
    }
}

pub fn session_target() -> UnitDefinition {
    UnitDefinition {
        unit_name: "velyx-session.target".to_string(),
        dbus_name: String::new(),
        required: true,
        startup_order: 0,
        restart_policy: "none".to_string(),
        contents: "[Unit]\nDescription=Velyx OS User Session Target\nWants=velyx-settings.service velyx-permissions.service velyx-launcher.service velyx-diagnostics.service velyx-ai.service velyx-file.service velyx-update-engine.service velyx-recovery.service velyx-shell.service\nAfter=graphical-session-pre.target dbus.service\nAllowIsolate=true\n"
            .to_string(),
    }
}

pub fn core_units() -> Vec<UnitDefinition> {
    let mut units = vec![
        service_unit(
            "velyx-settings.service",
            "Velyx OS Settings Service",
            "com.velyx.Settings",
            false,
            10,
            "on-failure",
            &env_exec("VELYX_SETTINGS_BINARY", "/usr/bin/velyx-settings-service"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-permissions.service",
            "Velyx OS Permissions Service",
            "com.velyx.Permissions",
            true,
            20,
            "on-failure",
            &env_exec(
                "VELYX_PERMISSIONS_BINARY",
                "/usr/bin/velyx-permissions-service",
            ),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-launcher.service",
            "Velyx OS Launcher Service",
            "com.velyx.Launcher",
            true,
            30,
            "on-failure",
            &env_exec("VELYX_LAUNCHER_BINARY", "/usr/bin/velyx-launcher-service"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-diagnostics.service",
            "Velyx OS Diagnostics Service",
            "com.velyx.Diagnostics",
            false,
            40,
            "on-failure",
            &env_exec(
                "VELYX_DIAGNOSTICS_BINARY",
                "/usr/bin/velyx-diagnostics-service",
            ),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-file.service",
            "Velyx OS File Service",
            "com.velyx.FileService",
            false,
            45,
            "on-failure",
            &env_exec("VELYX_FILE_BINARY", "/usr/bin/velyx-file-service"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-ai.service",
            "Velyx OS AI Service",
            "com.velyx.AI",
            false,
            50,
            "on-failure",
            &env_exec("VELYX_AI_BINARY", "/usr/bin/velyx-ai-service"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-update-engine.service",
            "Velyx OS Update Engine",
            "com.velyx.UpdateEngine",
            false,
            55,
            "on-failure",
            &env_exec("VELYX_UPDATE_ENGINE_BINARY", "/usr/bin/velyx-update-engine"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-recovery.service",
            "Velyx OS Recovery Service",
            "com.velyx.Recovery",
            false,
            56,
            "on-failure",
            &env_exec("VELYX_RECOVERY_BINARY", "/usr/bin/velyx-recovery-service"),
            "velyx-session.target",
        ),
        service_unit(
            "velyx-shell.service",
            "Velyx OS Shell",
            "",
            true,
            60,
            "always",
            &env_exec("VELYX_SHELL_BINARY", "/usr/bin/velyx-shell"),
            "velyx-session.target",
        ),
    ];
    if let Some(shell) = units.iter_mut().find(|unit| unit.unit_name == "velyx-shell.service") {
        shell.contents = format!(
            "[Unit]\nDescription=Velyx OS Shell\nPartOf=velyx-session.target\nAfter=velyx-permissions.service velyx-launcher.service graphical-session-pre.target dbus.service\nRequires=velyx-permissions.service velyx-launcher.service\nWants=velyx-settings.service\nAfter=velyx-settings.service\n\n[Service]\nType=simple\nEnvironment=VELYX_USER_ID=%u\nEnvironment=HOME=%h\nExecStart={}\nRestart=always\nRestartSec=1\n\n[Install]\nWantedBy=velyx-session.target\n",
            env_exec("VELYX_SHELL_BINARY", "/usr/bin/velyx-shell")
        );
    }
    units
}

pub fn sorted_runtime_units() -> Vec<UnitDefinition> {
    let mut units = core_units();
    units.sort_by_key(|unit| unit.startup_order);
    units
}

pub fn unit_install_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("systemd").join("user")
}

pub fn install_unit_files() -> Result<Vec<PathBuf>, SessionManagerError> {
    let install_dir = unit_install_dir();
    fs::create_dir_all(&install_dir).map_err(|err| {
        SessionManagerError::SystemdUnavailable(format!(
            "не удалось создать каталог user units '{}': {}",
            install_dir.display(),
            err
        ))
    })?;

    let mut written = Vec::new();
    for unit in std::iter::once(session_target()).chain(sorted_runtime_units().into_iter()) {
        let path = install_dir.join(&unit.unit_name);
        fs::write(&path, unit.contents.as_bytes()).map_err(|err| {
            SessionManagerError::SystemdUnavailable(format!(
                "не удалось записать unit '{}': {}",
                path.display(),
                err
            ))
        })?;
        written.push(path);
    }
    Ok(written)
}

pub async fn daemon_reload() -> Result<(), SessionManagerError> {
    run_systemctl_user(&["daemon-reload"]).await.map(|_| ())
}

pub async fn start_target(target: &str) -> Result<(), SessionManagerError> {
    run_systemctl_user(&["start", target]).await.map(|_| ())
}

pub async fn start_unit(unit_name: &str) -> Result<(), SessionManagerError> {
    run_systemctl_user(&["start", unit_name]).await.map(|_| ())
}

pub async fn stop_target(target: &str) -> Result<(), SessionManagerError> {
    run_systemctl_user(&["stop", target]).await.map(|_| ())
}

pub async fn restart_unit(unit_name: &str) -> Result<(), SessionManagerError> {
    run_systemctl_user(&["restart", unit_name]).await.map(|_| ())
}

pub async fn is_active(unit_name: &str) -> Result<String, SessionManagerError> {
    run_systemctl_user(&["is-active", unit_name]).await
}

pub async fn main_pid(unit_name: &str) -> Result<Option<u32>, SessionManagerError> {
    let raw = run_systemctl_user(&["show", unit_name, "--property", "MainPID", "--value"]).await?;
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "0" {
        Ok(None)
    } else {
        trimmed
            .parse::<u32>()
            .map(Some)
            .map_err(|err| SessionManagerError::SystemdUnavailable(format!(
                "не удалось распарсить MainPID для '{}': {}",
                unit_name, err
            )))
    }
}

pub async fn ensure_systemd_user_available() -> Result<(), SessionManagerError> {
    if run_systemctl_user(&["show-environment"]).await.is_ok() {
        return Ok(());
    }
    run_systemctl_user(&["status", "default.target"])
        .await
        .map(|_| ())
}

pub async fn ensure_unit_start_order() -> Result<Vec<UnitDefinition>, SessionManagerError> {
    install_unit_files()?;
    daemon_reload().await?;
    Ok(sorted_runtime_units())
}

async fn run_systemctl_user(args: &[&str]) -> Result<String, SessionManagerError> {
    let mut command = Command::new("systemctl");
    if !args.first().map(|arg| *arg == "--user").unwrap_or(false) {
        command.arg("--user");
    }
    command.args(args);
    let output = command.output().await.map_err(|err| {
        SessionManagerError::SystemdUnavailable(format!(
            "systemctl --user недоступен: {}",
            err
        ))
    })?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(SessionManagerError::SystemdUnavailable(format!(
            "systemctl --user {:?} завершился ошибкой: {}",
            args,
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

pub fn repo_unit_templates(root: &Path) -> Vec<(PathBuf, String)> {
    let dir = root.join("systemd").join("user");
    let mut files = Vec::new();
    for unit in std::iter::once(session_target()).chain(sorted_runtime_units().into_iter()) {
        files.push((dir.join(unit.unit_name), unit.contents));
    }
    files
}
