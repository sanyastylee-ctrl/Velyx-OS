use crate::audit::UpdateAuditLogger;
use crate::errors::UpdateEngineError;
use crate::orchestration::{apply_update, available_updates, status_payload};
use crate::reconciliation::{reconcile_state, validate_consistency};
use crate::store::UpdateStore;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct UpdateApi {
    store: Arc<Mutex<UpdateStore>>,
    audit: UpdateAuditLogger,
    base_dir: PathBuf,
}

impl UpdateApi {
    pub fn new(store: UpdateStore, audit: UpdateAuditLogger, base_dir: PathBuf) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            audit,
            base_dir,
        }
    }
}

#[zbus::interface(name = "com.velyx.UpdateEngine1")]
impl UpdateApi {
    async fn check_for_updates(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let mut store = self.store.lock().await;
        let payload = available_updates(&mut store).map_err(|err: UpdateEngineError| {
            zbus::fdo::Error::Failed(err.message())
        })?;
        let _ = self.audit.log(
            "check_updates",
            "ok",
            &format!("count={}", payload.len()),
        );
        Ok(payload)
    }

    async fn apply_update(&self, update_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut store = self.store.lock().await;
        match apply_update(&mut store, &self.audit, &self.base_dir, update_id).await {
            Ok(payload) => Ok(payload),
            Err(err) => {
                let _ = self.audit.log(
                    "update_failed",
                    "failed",
                    &format!("update_id={} reason={}", update_id, err.message()),
                );
                Err(zbus::fdo::Error::Failed(err.message()))
            }
        }
    }

    async fn get_update_status(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let store = self.store.lock().await;
        Ok(status_payload(&store))
    }

    async fn list_update_attempts(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let store = self.store.lock().await;
        Ok(store
            .attempts()
            .into_iter()
            .map(|attempt| attempt.to_map())
            .collect())
    }

    async fn get_attempt_details(&self, attempt_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let store = self.store.lock().await;
        let attempt = store
            .get_attempt(attempt_id)
            .ok_or_else(|| zbus::fdo::Error::Failed(format!("unknown attempt_id {}", attempt_id)))?;
        Ok(attempt.to_map())
    }

    async fn on_rollback_completed(&self, snapshot_id: &str, result: &str) -> zbus::fdo::Result<bool> {
        let mut store = self.store.lock().await;
        match crate::orchestration::on_rollback_completed(&mut store, &self.audit, snapshot_id, result) {
            Ok(applied) => Ok(applied),
            Err(err) => {
                let _ = self.audit.log(
                    "rollback_callback_failed",
                    "failed",
                    &format!("snapshot_id={} reason={}", snapshot_id, err.message()),
                );
                Err(zbus::fdo::Error::Failed(err.message()))
            }
        }
    }

    async fn validate_consistency(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let store = self.store.lock().await;
        let payload = validate_consistency(&self.base_dir, &store);
        if payload.get("status").map(String::as_str) == Some("ok") {
            let _ = self.audit.log("consistency_validation_ok", "ok", "no issues detected");
        } else {
            let summary = payload.get("issues_summary").cloned().unwrap_or_default();
            let _ = self.audit.log("consistency_validation_failed", "failed", &summary);
            let _ = self.audit.log("consistency_issue_detected", "warning", &summary);
        }
        Ok(payload)
    }

    async fn reconcile_state(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut store = self.store.lock().await;
        reconcile_state(&self.base_dir, &mut store, &self.audit)
            .map_err(|err: UpdateEngineError| zbus::fdo::Error::Failed(err.message()))
    }
}
