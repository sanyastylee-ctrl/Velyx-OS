use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct DiagnosticsAuditLogger {
    path: PathBuf,
}

impl DiagnosticsAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        create_dir_all(base_dir).map_err(|err| format!("failed to create diagnostics audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("diagnostics_audit.log"),
        })
    }

    pub fn log(&self, user_id: &str, action: &str, result: &str) -> Result<(), String> {
        let line = format!(
            "{} user={} action={} result={}\n",
            Utc::now().to_rfc3339(),
            user_id,
            action,
            result
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("failed to open diagnostics audit log: {err}"))?;
        file.write_all(line.as_bytes())
            .map_err(|err| format!("failed to write diagnostics audit log: {err}"))?;
        Ok(())
    }
}
