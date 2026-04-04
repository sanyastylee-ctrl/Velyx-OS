use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct RecoveryAuditLogger {
    path: PathBuf,
}

impl RecoveryAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(base_dir)
            .map_err(|err| format!("failed to create recovery audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("recovery_audit.log"),
        })
    }

    pub fn log(&self, action: &str, result: &str, details: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("recovery audit open failed: {err}"))?;
        writeln!(
            file,
            "{} action={} result={} details={}",
            Utc::now().to_rfc3339(),
            action,
            result,
            details
        )
        .map_err(|err| format!("recovery audit write failed: {err}"))
    }
}
