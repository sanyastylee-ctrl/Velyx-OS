use crate::model::{FirstBootMarker, FirstBootState, InstallHandoffState, InstallPlan};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub struct InstallerHandoffStore {
    handoff_path: PathBuf,
    first_boot_path: PathBuf,
    handoff: Option<InstallHandoffState>,
    first_boot: Option<FirstBootMarker>,
}

impl InstallerHandoffStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let handoff_path = base_dir.join("install_handoff.json");
        let first_boot_path = base_dir.join("first_boot_state.json");
        let handoff = fs::read_to_string(&handoff_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<InstallHandoffState>(&raw).ok());
        let first_boot = fs::read_to_string(&first_boot_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<FirstBootMarker>(&raw).ok());
        Self {
            handoff_path,
            first_boot_path,
            handoff,
            first_boot,
        }
    }

    pub fn handoff(&self) -> Option<InstallHandoffState> {
        self.handoff.clone()
    }

    pub fn first_boot(&self) -> Option<FirstBootMarker> {
        self.first_boot.clone()
    }

    pub fn write_install_handoff(
        &mut self,
        install_id: String,
        plan: &InstallPlan,
    ) -> Result<(), String> {
        let handoff = InstallHandoffState {
            install_id: install_id.clone(),
            target_id: plan.target_id.clone(),
            profile_id: plan.profile_id.clone(),
            encryption_enabled: plan.encryption_enabled,
            requested_username: plan.username.clone(),
            requested_locale: plan.locale.clone(),
            first_boot_pending: true,
            baseline_settings_pending: true,
            session_start_pending: true,
            created_at: Utc::now().to_rfc3339(),
        };
        let first_boot = FirstBootMarker {
            state: FirstBootState::Pending,
            install_id,
            target_id: plan.target_id.clone(),
            requested_username: plan.username.clone(),
            requested_locale: plan.locale.clone(),
            baseline_settings_applied: false,
            user_created: false,
            handoff_ready: false,
            completed_at: None,
            failed_reason: None,
        };
        persist_json(&self.handoff_path, &handoff)?;
        persist_json(&self.first_boot_path, &first_boot)?;
        self.handoff = Some(handoff);
        self.first_boot = Some(first_boot);
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
