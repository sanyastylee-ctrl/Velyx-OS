use crate::audit::InstallerAuditLogger;
use crate::handoff::InstallerHandoffStore;
use crate::model::{DiskTarget, FirstBootState, InstallPlan, InstallProfile};
use crate::store::InstallerStore;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InstallerApi {
    store: Arc<Mutex<InstallerStore>>,
    handoff: Arc<Mutex<InstallerHandoffStore>>,
    audit: InstallerAuditLogger,
}

impl InstallerApi {
    pub fn new(store: InstallerStore, handoff: InstallerHandoffStore, audit: InstallerAuditLogger) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            handoff: Arc::new(Mutex::new(handoff)),
            audit,
        }
    }

    fn disk_targets() -> Vec<DiskTarget> {
        vec![DiskTarget {
            target_id: "disk0".to_string(),
            device_path: "/dev/nvme0n1".to_string(),
            capacity_gb: 512,
            scheme: "gpt-btrfs".to_string(),
            supports_encryption: true,
            supports_rollback_layout: true,
        }]
    }

    fn install_profiles() -> Vec<InstallProfile> {
        vec![
            InstallProfile {
                profile_id: "desktop-default".to_string(),
                display_name: "Desktop Default".to_string(),
                description: "Стандартный профиль Velyx OS для everyday desktop usage.".to_string(),
                gaming_ready: false,
                developer_ready: false,
                baseline_ai_mode: "local_only".to_string(),
            },
            InstallProfile {
                profile_id: "developer".to_string(),
                display_name: "Developer".to_string(),
                description: "Профиль с упором на разработку и debug tooling.".to_string(),
                gaming_ready: false,
                developer_ready: true,
                baseline_ai_mode: "local_only".to_string(),
            },
            InstallProfile {
                profile_id: "gaming-ready".to_string(),
                display_name: "Gaming Ready".to_string(),
                description: "Профиль с подготовкой под Steam target и compatibility stack.".to_string(),
                gaming_ready: true,
                developer_ready: false,
                baseline_ai_mode: "disabled".to_string(),
            },
        ]
    }
}

#[zbus::interface(name = "com.velyx.Installer1")]
impl InstallerApi {
    async fn get_disk_targets(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let _ = self.audit.log("get_disk_targets", "ok", "returned static targets");
        Ok(Self::disk_targets()
            .into_iter()
            .map(|target| target.to_map())
            .collect())
    }

    async fn list_install_profiles(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let _ = self.audit.log("list_install_profiles", "ok", "returned install profiles");
        Ok(Self::install_profiles()
            .into_iter()
            .map(|profile| profile.to_map())
            .collect())
    }

    async fn prepare_install(
        &self,
        target_id: &str,
        profile_id: &str,
        encryption_enabled: bool,
        username: &str,
        locale: &str,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let plan = InstallPlan {
            target_id: target_id.to_string(),
            profile_id: profile_id.to_string(),
            encryption_enabled,
            username: username.to_string(),
            locale: locale.to_string(),
            bootloader_target: "uefi-default".to_string(),
        };

        let mut store = self.store.lock().await;
        store
            .prepare_plan(plan.clone())
            .map_err(zbus::fdo::Error::Failed)?;
        let _ = self.audit.log(
            "prepare_install",
            "ok",
            &format!(
                "target_id={} profile_id={} encryption_enabled={} username={}",
                target_id, profile_id, encryption_enabled, username
            ),
        );

        let mut payload = HashMap::new();
        payload.insert("status".to_string(), "ready".to_string());
        payload.insert("target_id".to_string(), plan.target_id);
        payload.insert("profile_id".to_string(), plan.profile_id);
        payload.insert("username".to_string(), plan.username);
        payload.insert("locale".to_string(), plan.locale);
        payload.insert(
            "encryption_enabled".to_string(),
            plan.encryption_enabled.to_string(),
        );
        payload.insert("bootloader_target".to_string(), plan.bootloader_target);
        payload.insert(
            "first_boot_state".to_string(),
            FirstBootState::Pending.as_str().to_string(),
        );
        Ok(payload)
    }

    async fn commit_install(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let mut store = self.store.lock().await;
        let snapshot = store.snapshot();
        let plan = snapshot
            .prepared_plan
            .clone()
            .ok_or_else(|| zbus::fdo::Error::Failed("install plan is missing".to_string()))?;
        store
            .mark_post_install_config_written()
            .map_err(zbus::fdo::Error::Failed)?;
        store
            .advance_first_boot(FirstBootState::Pending)
            .map_err(zbus::fdo::Error::Failed)?;
        drop(store);

        let install_id = format!("install-{}", Utc::now().timestamp_millis());
        let mut handoff = self.handoff.lock().await;
        handoff
            .write_install_handoff(install_id.clone(), &plan)
            .map_err(zbus::fdo::Error::Failed)?;
        let _ = self.audit.log("commit_install", "ok", "post-install config staged");
        let _ = self.audit.log(
            "install_handoff_written",
            "ok",
            &format!("install_id={} username={}", install_id, plan.username),
        );
        let _ = self
            .audit
            .log("first_boot_marker_set", "ok", "first_boot_pending=true");

        let mut payload = HashMap::new();
        payload.insert("status".to_string(), "installed".to_string());
        payload.insert("install_id".to_string(), install_id);
        payload.insert(
            "first_boot_state".to_string(),
            FirstBootState::Pending.as_str().to_string(),
        );
        payload.insert("recovery_hook".to_string(), "registered".to_string());
        payload.insert("first_boot_pending".to_string(), "true".to_string());
        Ok(payload)
    }

    async fn get_first_boot_state(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let handoff = self.handoff.lock().await;
        let first_boot = handoff.first_boot();
        let mut payload = HashMap::new();
        if let Some(marker) = first_boot {
            payload.extend(marker.to_map());
        } else {
            payload.insert("state".to_string(), FirstBootState::None.as_str().to_string());
        }
        Ok(payload)
    }

    async fn get_install_handoff(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let handoff = self.handoff.lock().await;
        let record = handoff
            .handoff()
            .ok_or_else(|| zbus::fdo::Error::Failed("install handoff not found".to_string()))?;
        let mut payload = HashMap::new();
        payload.insert("install_id".to_string(), record.install_id);
        payload.insert("target_id".to_string(), record.target_id);
        payload.insert("profile_id".to_string(), record.profile_id);
        payload.insert(
            "encryption_enabled".to_string(),
            record.encryption_enabled.to_string(),
        );
        payload.insert("requested_username".to_string(), record.requested_username);
        payload.insert("requested_locale".to_string(), record.requested_locale);
        payload.insert(
            "first_boot_pending".to_string(),
            record.first_boot_pending.to_string(),
        );
        payload.insert(
            "baseline_settings_pending".to_string(),
            record.baseline_settings_pending.to_string(),
        );
        payload.insert(
            "session_start_pending".to_string(),
            record.session_start_pending.to_string(),
        );
        payload.insert("created_at".to_string(), record.created_at);
        Ok(payload)
    }

    async fn get_install_status(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        let store = self.store.lock().await;
        let snapshot = store.snapshot();
        let handoff = self.handoff.lock().await;
        let handoff_state = handoff.handoff();
        let first_boot = handoff.first_boot();
        let mut payload = HashMap::new();
        payload.insert(
            "installer_state".to_string(),
            snapshot.current_state.as_str().to_string(),
        );
        payload.insert(
            "post_install_config_written".to_string(),
            snapshot.post_install_config_written.to_string(),
        );
        payload.insert(
            "recovery_hook_registered".to_string(),
            snapshot.recovery_hook_registered.to_string(),
        );
        payload.insert(
            "install_handoff_present".to_string(),
            handoff_state.is_some().to_string(),
        );
        payload.insert(
            "first_boot_state".to_string(),
            first_boot
                .map(|state| state.state.as_str().to_string())
                .unwrap_or_else(|| FirstBootState::None.as_str().to_string()),
        );
        Ok(payload)
    }
}
