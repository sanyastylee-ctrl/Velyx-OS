use crate::model::{FirstBootState, InstallPlan, InstallerState};
use std::fs;
use std::path::{Path, PathBuf};

pub struct InstallerStore {
    path: PathBuf,
    state: InstallerState,
}

impl InstallerStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let path = base_dir.join("installer_state.json");
        let state = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<InstallerState>(&raw).ok())
            .unwrap_or_default();
        Self { path, state }
    }

    pub fn snapshot(&self) -> InstallerState {
        self.state.clone()
    }

    pub fn prepare_plan(&mut self, plan: InstallPlan) -> Result<(), String> {
        self.state.prepared_plan = Some(plan);
        self.state.current_state = FirstBootState::Pending;
        self.state.recovery_hook_registered = true;
        self.persist()
    }

    pub fn advance_first_boot(&mut self, state: FirstBootState) -> Result<(), String> {
        self.state.current_state = state;
        self.persist()
    }

    pub fn mark_post_install_config_written(&mut self) -> Result<(), String> {
        self.state.post_install_config_written = true;
        self.persist()
    }

    fn persist(&self) -> Result<(), String> {
        let tmp = self.path.with_extension("json.tmp");
        let raw = serde_json::to_string_pretty(&self.state)
            .map_err(|err| format!("installer state serialization failed: {err}"))?;
        fs::write(&tmp, raw)
            .map_err(|err| format!("installer state temp write failed: {err}"))?;
        fs::rename(&tmp, &self.path)
            .map_err(|err| format!("installer state rename failed: {err}"))
    }
}
