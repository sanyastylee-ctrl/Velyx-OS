use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct SessionAuditLogger {
    path: PathBuf,
}

impl SessionAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(base_dir)
            .map_err(|err| format!("failed to create session audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("session_manager_audit.log"),
        })
    }

    pub fn log_transition(
        &self,
        action: &str,
        state_from: &str,
        state_to: &str,
        user_id: &str,
        service_name: &str,
        result: &str,
        reason: &str,
    ) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("session audit open failed: {err}"))?;
        writeln!(
            file,
            "{} action={} state_from={} state_to={} user_id={} service_name={} result={} reason={}",
            Utc::now().to_rfc3339(),
            action,
            state_from,
            state_to,
            user_id,
            service_name,
            result,
            reason
        )
        .map_err(|err| format!("session audit write failed: {err}"))
    }
}
