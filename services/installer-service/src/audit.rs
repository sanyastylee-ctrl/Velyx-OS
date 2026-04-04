use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct InstallerAuditLogger {
    path: PathBuf,
}

impl InstallerAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(base_dir)
            .map_err(|err| format!("failed to create installer audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("installer_audit.log"),
        })
    }

    pub fn log(&self, action: &str, result: &str, details: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("installer audit open failed: {err}"))?;
        writeln!(
            file,
            "{} action={} result={} details={}",
            Utc::now().to_rfc3339(),
            action,
            result,
            details
        )
        .map_err(|err| format!("installer audit write failed: {err}"))
    }
}
