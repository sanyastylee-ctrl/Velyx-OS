use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct AuditLogger {
    path: PathBuf,
}

impl AuditLogger {
    pub fn new(base_dir: &Path) -> Result<Self, String> {
        create_dir_all(base_dir).map_err(|err| format!("failed to create audit dir: {err}"))?;
        Ok(Self {
            path: base_dir.join("audit.log"),
        })
    }

    pub fn log(
        &self,
        app_id: &str,
        permission: &str,
        action: &str,
        result: &str,
        sender: &str,
        user_id: &str,
        trust_level: &str,
        policy_decision_source: &str,
    ) -> Result<(), String> {
        let timestamp = Utc::now().to_rfc3339();
        let line = format!(
            "{} user={} app={} perm={} action={} result={} sender={} trust_level={} source={}\n",
            timestamp, user_id, app_id, permission, action, result, sender, trust_level, policy_decision_source
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| format!("failed to open audit log: {err}"))?;
        file.write_all(line.as_bytes())
            .map_err(|err| format!("failed to write audit log: {err}"))?;
        Ok(())
    }
}
