use crate::model::AuditEntry;
use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct AiAuditLogger {
    path: PathBuf,
}

impl AiAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        create_dir_all(base_dir).map_err(|err| format!("failed to create ai audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("ai_audit.log"),
        })
    }

    pub fn log(&self, entry: AuditEntry) -> Result<(), String> {
        let line = serde_json::to_string(&entry)
            .map_err(|err| format!("failed to encode ai audit entry: {err}"))?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("failed to open ai audit log: {err}"))?;
        file.write_all(line.as_bytes())
            .map_err(|err| format!("failed to write ai audit log: {err}"))?;
        file.write_all(b"\n")
            .map_err(|err| format!("failed to terminate ai audit log line: {err}"))?;
        Ok(())
    }

    pub fn now() -> String {
        Utc::now().to_rfc3339()
    }
}
