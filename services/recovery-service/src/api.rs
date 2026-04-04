use crate::audit::RecoveryAuditLogger;
use crate::errors::RecoveryError;
use crate::orchestration::{
    list_restore_points, notify_update_engine_rollback_completed, register_restore_point,
    rollback_snapshot, status_payload,
};
use crate::store::RecoveryStore;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RecoveryApi {
    store: Arc<Mutex<RecoveryStore>>,
    audit: RecoveryAuditLogger,
    base_dir: PathBuf,
}

impl RecoveryApi {
    pub fn new(store: RecoveryStore, audit: RecoveryAuditLogger, base_dir: PathBuf) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            audit,
            base_dir,
        }
    }
}

#[zbus::interface(name = "com.velyx.Recovery1")]
impl RecoveryApi {
    async fn list_restore_points(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let mut store = self.store.lock().await;
        let payload = list_restore_points(&mut store)
            .map_err(|err: RecoveryError| zbus::fdo::Error::Failed(err.message()))?;
        let _ = self.audit.log(
            "list_restore_points",
            "ok",
            &format!("count={}", payload.len()),
        );
        Ok(payload)
    }

    async fn rollback(&self, snapshot_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut store = self.store.lock().await;
        match rollback_snapshot(&mut store, &self.audit, &self.base_dir, snapshot_id) {
            Ok(payload) => {
                drop(store);
                let callback_result = notify_update_engine_rollback_completed(
                    &self.audit,
                    snapshot_id,
                    "success",
                )
                .await;
                if let Err(err) = callback_result {
                    let _ = self.audit.log(
                        "rollback_callback_send_failed",
                        "failed",
                        &format!("snapshot_id={} reason={}", snapshot_id, err.message()),
                    );
                }
                Ok(payload)
            }
            Err(err) => {
                let _ = store.set_state(crate::model::RecoveryState::Failed);
                let _ = store.set_last_result(
                    Some(snapshot_id.to_string()),
                    None,
                    None,
                    "rollback_failed".to_string(),
                );
                drop(store);
                let _ = notify_update_engine_rollback_completed(&self.audit, snapshot_id, "failed").await;
                let _ = self.audit.log(
                    "rollback_failed",
                    "failed",
                    &format!("snapshot_id={} reason={}", snapshot_id, err.message()),
                );
                Err(zbus::fdo::Error::Failed(err.message()))
            }
        }
    }

    async fn get_recovery_status(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let store = self.store.lock().await;
        let payload = status_payload(&store);
        let _ = self.audit.log(
            "recovery_status_requested",
            "ok",
            &format!("state={}", payload.get("state").cloned().unwrap_or_default()),
        );
        Ok(payload)
    }

    async fn register_restore_point(
        &self,
        snapshot_id: &str,
        update_id: &str,
        reason: &str,
    ) -> zbus::fdo::Result<bool> {
        let mut store = self.store.lock().await;
        let result = register_restore_point(&mut store, snapshot_id, update_id, reason)
            .map_err(|err: RecoveryError| zbus::fdo::Error::Failed(err.message()))?;
        let attempt_id = reason
            .split("attempt_id=")
            .nth(1)
            .unwrap_or_default()
            .to_string();
        let _ = store.set_last_result(
            Some(snapshot_id.to_string()),
            Some(update_id.to_string()),
            Some(attempt_id),
            "restore_point_registered".to_string(),
        );
        let _ = self.audit.log(
            "restore_point_registered",
            "ok",
            &format!("snapshot_id={} update_id={} reason={}", snapshot_id, update_id, reason),
        );
        Ok(result)
    }
}
