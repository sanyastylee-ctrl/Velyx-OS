use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallHandoffState {
    pub install_id: String,
    pub target_id: String,
    pub profile_id: String,
    pub encryption_enabled: bool,
    pub requested_username: String,
    pub requested_locale: String,
    pub first_boot_pending: bool,
    pub baseline_settings_pending: bool,
    pub session_start_pending: bool,
    pub created_at: String,
}

pub struct InstallHandoffStore {
    path: PathBuf,
    state: Option<InstallHandoffState>,
}

impl InstallHandoffStore {
    pub fn load(base_dir: &Path) -> Self {
        let path = base_dir.join("install_handoff.json");
        let state = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<InstallHandoffState>(&raw).ok());
        Self { path, state }
    }

    pub fn state(&self) -> Option<InstallHandoffState> {
        self.state.clone()
    }

    pub fn update(&mut self, state: InstallHandoffState) -> Result<(), String> {
        persist_json(&self.path, &state)?;
        self.state = Some(state);
        Ok(())
    }
}

fn persist_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(value)
        .map_err(|err| format!("handoff serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("handoff temp write failed: {err}"))?;
    fs::rename(&tmp, path).map_err(|err| format!("handoff rename failed: {err}"))
}
