use crate::model::{RecoveryState, RecoveryStatus, RestorePoint};
use std::fs;
use std::path::{Path, PathBuf};

pub struct RecoveryStore {
    status_path: PathBuf,
    restore_points_path: PathBuf,
    status: RecoveryStatus,
    restore_points: Vec<RestorePoint>,
}

impl RecoveryStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let status_path = base_dir.join("recovery_status.json");
        let restore_points_path = base_dir.join("restore_points.json");
        let status = fs::read_to_string(&status_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<RecoveryStatus>(&raw).ok())
            .unwrap_or_default();
        let restore_points = fs::read_to_string(&restore_points_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<Vec<RestorePoint>>(&raw).ok())
            .unwrap_or_default();
        Self {
            status_path,
            restore_points_path,
            status,
            restore_points,
        }
    }

    pub fn status(&self) -> RecoveryStatus {
        self.status.clone()
    }

    pub fn restore_points(&self) -> Vec<RestorePoint> {
        self.restore_points.clone()
    }

    pub fn set_state(&mut self, state: RecoveryState) -> Result<(), String> {
        self.status.current_state = state;
        self.persist_status()
    }

    pub fn set_last_result(
        &mut self,
        snapshot_id: Option<String>,
        update_id: Option<String>,
        attempt_id: Option<String>,
        result: String,
    ) -> Result<(), String> {
        self.status.last_snapshot_id = snapshot_id;
        self.status.last_update_id = update_id;
        self.status.last_attempt_id = attempt_id;
        self.status.last_result = result;
        self.persist_status()
    }

    pub fn add_restore_point(&mut self, point: RestorePoint) -> Result<(), String> {
        self.restore_points.push(point);
        self.persist_restore_points()
    }

    pub fn get_restore_point(&self, snapshot_id: &str) -> Option<RestorePoint> {
        self.restore_points
            .iter()
            .find(|point| point.snapshot_id == snapshot_id)
            .cloned()
    }

    pub fn update_restore_point(&mut self, updated: RestorePoint) -> Result<(), String> {
        if let Some(existing) = self
            .restore_points
            .iter_mut()
            .find(|point| point.snapshot_id == updated.snapshot_id)
        {
            *existing = updated;
            self.persist_restore_points()
        } else {
            Err(format!("restore point {} not found", updated.snapshot_id))
        }
    }

    fn persist_status(&self) -> Result<(), String> {
        persist_json(&self.status_path, &self.status)
    }

    fn persist_restore_points(&self) -> Result<(), String> {
        persist_json(&self.restore_points_path, &self.restore_points)
    }
}

fn persist_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(value)
        .map_err(|err| format!("recovery store serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("recovery store temp write failed: {err}"))?;
    fs::rename(&tmp, path).map_err(|err| format!("recovery store rename failed: {err}"))
}
