use crate::sandbox::SandboxLaunchResult;
use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone)]
pub struct LauncherAuditLogger {
    history_path: PathBuf,
    sandbox_path: PathBuf,
}

impl LauncherAuditLogger {
    pub fn new(base_dir: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&base_dir)
            .map_err(|err| format!("не удалось подготовить каталог audit {}: {err}", base_dir.display()))?;
        Ok(Self {
            history_path: base_dir.join("launcher_history.log"),
            sandbox_path: base_dir.join("sandbox_audit.log"),
        })
    }

    pub fn log_launch_history(
        &self,
        app_id: &str,
        action: &str,
        trust_level: &str,
        decision: &str,
        sandbox_profile: &str,
        result: &str,
    ) -> Result<(), String> {
        self.write_line(
            &self.history_path,
            &format!(
                "{} app_id={} action={} trust_level={} decision={} sandbox_profile={} result={}",
                Self::now(),
                app_id,
                action,
                trust_level,
                decision,
                sandbox_profile,
                result
            ),
        )
    }

    pub fn log_sandbox(
        &self,
        result: &SandboxLaunchResult,
        permission_context: &str,
        launch_result: &str,
    ) -> Result<(), String> {
        self.write_line(
            &self.sandbox_path,
            &format!(
                "{} app_id={} profile={} mounts={} permission_context={} launched_by={} pid={} sandbox_id={} result={}",
                Self::now(),
                result.identity.app_id,
                result.identity.sandbox_profile,
                result.applied_mounts.join("|"),
                permission_context,
                result.identity.launched_by,
                result.identity.pid,
                result.identity.sandbox_id,
                launch_result
            ),
        )
    }

    fn write_line(&self, path: &PathBuf, line: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|err| format!("не удалось открыть audit log {}: {err}", path.display()))?;
        writeln!(file, "{line}")
            .map_err(|err| format!("не удалось записать audit log {}: {err}", path.display()))
    }

    fn now() -> String {
        Utc::now().to_rfc3339()
    }
}
