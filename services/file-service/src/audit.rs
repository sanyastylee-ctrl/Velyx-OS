use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileAuditLogger {
    path: PathBuf,
}

impl FileAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(base_dir)
            .map_err(|err| format!("failed to create file audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("file_audit.log"),
        })
    }

    pub fn log(
        &self,
        requester: &str,
        action: &str,
        target: &str,
        result_count: usize,
        access_level: &str,
    ) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("file audit open failed: {err}"))?;
        writeln!(
            file,
            "{} requester={} action={} target={} result_count={} access_level={}",
            Utc::now().to_rfc3339(),
            requester,
            action,
            target,
            result_count,
            access_level
        )
        .map_err(|err| format!("file audit write failed: {err}"))
    }
}
