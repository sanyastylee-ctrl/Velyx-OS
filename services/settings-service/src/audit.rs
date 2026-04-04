use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct SettingsAuditLogger {
    path: PathBuf,
}

impl SettingsAuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        create_dir_all(base_dir).map_err(|err| format!("failed to create settings audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("settings_audit.log"),
        })
    }

    pub fn log(
        &self,
        user_id: &str,
        key: &str,
        old_value: &str,
        new_value: &str,
        source: &str,
        result: &str,
    ) -> Result<(), String> {
        let line = format!(
            "{} user={} key={} old_value={} new_value={} source={} result={}\n",
            Utc::now().to_rfc3339(),
            user_id,
            key,
            old_value,
            new_value,
            source,
            result
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("failed to open settings audit log: {err}"))?;
        file.write_all(line.as_bytes())
            .map_err(|err| format!("failed to write settings audit log: {err}"))?;
        Ok(())
    }
}
