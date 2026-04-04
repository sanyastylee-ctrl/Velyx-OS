use crate::audit::LauncherAuditLogger;
use crate::manifest::{AppManifest, ManifestRegistry};
use crate::sandbox::{SandboxLaunchRequest, SandboxRunner};
use crate::tracking::ProcessTracker;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::message::Header;
use zbus::names::BusName;

pub struct LauncherApi {
    manifests: ManifestRegistry,
    tracker: Arc<Mutex<ProcessTracker>>,
    audit: LauncherAuditLogger,
}

impl LauncherApi {
    pub fn new(manifests: ManifestRegistry, audit: LauncherAuditLogger) -> Self {
        Self {
            manifests,
            tracker: Arc::new(Mutex::new(ProcessTracker::default())),
            audit,
        }
    }

    async fn check_permission(app_id: &str, permission: &str) -> zbus::fdo::Result<String> {
        let connection = zbus::Connection::session()
            .await
            .map_err(|err| zbus::fdo::Error::Failed(format!("dbus session connection failed: {err}")))?;
        let proxy = zbus::Proxy::new(
            &connection,
            "com.velyx.Permissions",
            "/com/velyx/Permissions",
            "com.velyx.Permissions1",
        )
        .await
        .map_err(|err| zbus::fdo::Error::Failed(format!("permissions proxy init failed: {err}")))?;

        proxy
            .call("CheckPermission", &(app_id, permission))
            .await
            .map_err(|err| zbus::fdo::Error::Failed(format!("check permission failed: {err}")))
    }

    async fn request_permission(
        app_id: &str,
        permission: &str,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let connection = zbus::Connection::session()
            .await
            .map_err(|err| zbus::fdo::Error::Failed(format!("dbus session connection failed: {err}")))?;
        let proxy = zbus::Proxy::new(
            &connection,
            "com.velyx.Permissions",
            "/com/velyx/Permissions",
            "com.velyx.Permissions1",
        )
        .await
        .map_err(|err| zbus::fdo::Error::Failed(format!("permissions proxy init failed: {err}")))?;

        proxy
            .call("RequestPermission", &(app_id, permission))
            .await
            .map_err(|err| zbus::fdo::Error::Failed(format!("request permission failed: {err}")))
    }

    async fn resolve_launched_by(sender: &str) -> String {
        let connection = match zbus::Connection::session().await {
            Ok(connection) => connection,
            Err(_) => return "shell".to_string(),
        };
        let dbus_proxy = match zbus::fdo::DBusProxy::new(&connection).await {
            Ok(proxy) => proxy,
            Err(_) => return "shell".to_string(),
        };
        let ai_bus_name = match BusName::try_from("com.velyx.AI") {
            Ok(name) => name,
            Err(_) => return "shell".to_string(),
        };
        match dbus_proxy.get_name_owner(ai_bus_name).await {
            Ok(owner) if owner.as_str() == sender => "ai".to_string(),
            _ => "shell".to_string(),
        }
    }

    fn sender_from_header(header: &Header<'_>) -> String {
        header
            .sender()
            .map(|sender| sender.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    fn build_prompt_payload(
        mut payload: HashMap<String, String>,
        manifest: &AppManifest,
    ) -> HashMap<String, String> {
        payload.insert("status".to_string(), "prompt".to_string());
        payload.insert("display_name".to_string(), manifest.display_name.clone());
        payload.insert("trust_level".to_string(), manifest.trust_level.as_str().to_string());
        payload.insert("sandbox_profile".to_string(), manifest.sandbox_profile.clone());
        payload
    }

    fn build_terminal_payload(status: &str, message: &str) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        payload.insert("status".to_string(), status.to_string());
        payload.insert("message".to_string(), message.to_string());
        payload
    }
}

#[zbus::interface(name = "com.velyx.Launcher1")]
impl LauncherApi {
    async fn launch(
        &self,
        app_id: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let Some(manifest) = self.manifests.get(app_id).cloned() else {
            let _ = self
                .audit
                .log_launch_history(app_id, "unknown", "deny", "missing", "unknown_manifest");
            return Ok(Self::build_terminal_payload(
                "deny",
                &format!(
                    "Запуск запрещен: app_id '{}' не зарегистрирован в manifests",
                    app_id
                ),
            ));
        };
        if manifest.sandbox_profile.trim().is_empty() {
            let _ = self.audit.log_launch_history(
                app_id,
                manifest.trust_level.as_str(),
                "deny",
                "missing",
                "sandbox_profile_missing",
            );
            return Ok(Self::build_terminal_payload(
                "deny",
                "launch denied: sandbox profile не задан",
            ));
        }

        let sender = Self::sender_from_header(&header);
        let launched_by = Self::resolve_launched_by(&sender).await;
        let mut pending_prompt: Option<HashMap<String, String>> = None;
        let mut permission_context = HashMap::new();

        for permission in &manifest.requested_permissions {
            let status = Self::check_permission(app_id, permission).await?;
            permission_context.insert(permission.clone(), status.clone());

            if status == "deny" {
                let _ = self.audit.log_launch_history(
                    app_id,
                    manifest.trust_level.as_str(),
                    "deny",
                    &manifest.sandbox_profile,
                    "permissions_deny",
                );
                return Ok(Self::build_terminal_payload(
                    "deny",
                    &format!("Запуск {} запрещен политикой разрешений", app_id),
                ));
            }

            if status == "prompt" {
                let payload = Self::request_permission(app_id, permission).await?;
                pending_prompt = Some(Self::build_prompt_payload(payload, &manifest));
                let _ = self.audit.log_launch_history(
                    app_id,
                    manifest.trust_level.as_str(),
                    "prompt",
                    &manifest.sandbox_profile,
                    "permission_prompt_required",
                );
                break;
            }
        }

        if let Some(payload) = pending_prompt {
            return Ok(payload);
        }

        let request = SandboxLaunchRequest {
            app_id: app_id.to_string(),
            display_name: manifest.display_name.clone(),
            executable_path: manifest.executable_path.clone(),
            sandbox_profile: manifest.sandbox_profile.clone(),
            trust_level: manifest.trust_level.clone(),
            permission_context: permission_context.clone(),
            launched_by,
        };

        let launched = match SandboxRunner::launch(&request) {
            Ok(launched) => launched,
            Err(err) => {
                let _ = self.audit.log_launch_history(
                    app_id,
                    manifest.trust_level.as_str(),
                    "deny",
                    &manifest.sandbox_profile,
                    &format!("sandbox_error:{err}"),
                );
                return Err(zbus::fdo::Error::Failed(format!("sandbox launch failed: {err}")));
            }
        };
        let permission_summary = permission_context
            .iter()
            .map(|(permission, decision)| format!("{permission}={decision}"))
            .collect::<Vec<_>>()
            .join(",");
        let _ = self.audit.log_sandbox(&launched, &permission_summary, "started");
        let _ = self.audit.log_launch_history(
            app_id,
            manifest.trust_level.as_str(),
            "allow",
            &manifest.sandbox_profile,
            "started",
        );
        {
            let mut tracker = self.tracker.lock().await;
            tracker.insert(launched.identity.clone());
        }

        let mut payload = HashMap::new();
        payload.insert("status".to_string(), "started".to_string());
        payload.insert(
            "message".to_string(),
            format!(
                "{} запущено через secure launcher v2, pid={}, profile={}, source={}",
                app_id,
                launched.identity.pid,
                manifest.sandbox_profile,
                launched.identity.launched_by
            ),
        );
        payload.insert("app_id".to_string(), app_id.to_string());
        payload.insert("sandbox".to_string(), "bwrap".to_string());
        payload.insert("sandbox_profile".to_string(), manifest.sandbox_profile.clone());
        payload.insert("trust_level".to_string(), manifest.trust_level.as_str().to_string());
        payload.insert("launched_by".to_string(), launched.identity.launched_by.clone());
        payload.insert("pid".to_string(), launched.identity.pid.to_string());
        payload.insert("sandbox_id".to_string(), launched.identity.sandbox_id.clone());
        payload.insert("runtime_path".to_string(), launched.runtime_path);
        payload.insert("mounts".to_string(), launched.applied_mounts.join("|"));
        payload.insert("env_allowlist".to_string(), launched.filtered_env.join(","));
        Ok(payload)
    }

    async fn get_app_info(&self, app_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let manifest = self.manifests.get(app_id).ok_or_else(|| {
            zbus::fdo::Error::Failed(format!("app manifest not found for {app_id}"))
        })?;
        let mut payload = manifest.to_map();
        let tracker = self.tracker.lock().await;
        payload.insert(
            "running_instances".to_string(),
            tracker.running_for_app(app_id).to_string(),
        );
        Ok(payload)
    }

    async fn list_apps(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let mut apps = Vec::new();
        let tracker = self.tracker.lock().await;
        for manifest in self.manifests.list() {
            let mut payload = manifest.to_map();
            payload.insert(
                "running_instances".to_string(),
                tracker.running_for_app(&manifest.app_id).to_string(),
            );
            apps.push(payload);
        }
        Ok(apps)
    }
}
