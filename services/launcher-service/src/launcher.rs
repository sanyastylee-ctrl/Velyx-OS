use crate::audit::LauncherAuditLogger;
use crate::manifest::{AppManifest, AppStatus, ManifestRegistry};
use crate::sandbox::{validate_profile_name, SandboxLaunchRequest, SandboxRunner};
use crate::tracking::ProcessTracker;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};
use zbus::message::Header;
use zbus::names::BusName;

pub struct LauncherApi {
    registry: ManifestRegistry,
    tracker: Arc<Mutex<ProcessTracker>>,
    audit: LauncherAuditLogger,
}

impl LauncherApi {
    fn enrich_manifest_security(
        manifest: &AppManifest,
        payload: &mut HashMap<String, String>,
    ) {
        let manifest_validation = manifest.validate_for_launch();
        let executable_validation = manifest.validate_executable();
        let profile_validation = validate_profile_name(&manifest.sandbox_profile)
            .and_then(|profile| {
                let probe = SandboxLaunchRequest {
                    app_id: manifest.app_id.clone(),
                    display_name: manifest.display_name.clone(),
                    executable_path: manifest.executable_path.clone(),
                    sandbox_profile: profile,
                    trust_level: manifest.trust_level.clone(),
                    permission_context: HashMap::new(),
                    launched_by: "inspection".to_string(),
                };
                SandboxRunner::build_policy(&probe).map(|_| "ok".to_string())
            });

        payload.insert(
            "manifest_valid".to_string(),
            manifest_validation.is_ok().to_string(),
        );
        payload.insert(
            "executable_valid".to_string(),
            executable_validation.is_ok().to_string(),
        );
        payload.insert(
            "profile_valid".to_string(),
            profile_validation.is_ok().to_string(),
        );
        payload.insert(
            "manifest_validation_reason".to_string(),
            manifest_validation.err().unwrap_or_default(),
        );
        payload.insert(
            "executable_validation_reason".to_string(),
            executable_validation.err().unwrap_or_default(),
        );
        payload.insert(
            "profile_validation_reason".to_string(),
            profile_validation.err().unwrap_or_default(),
        );
    }

    fn security_payload(
        status: &str,
        message: &str,
        app_id: &str,
        reason: &str,
        next_action: &str,
        sandbox_profile: &str,
    ) -> HashMap<String, String> {
        let mut payload = Self::build_terminal_payload(status, message, app_id, "", reason, next_action);
        payload.insert("security_outcome".to_string(), reason.to_string());
        payload.insert("sandbox_profile".to_string(), sandbox_profile.to_string());
        payload
    }

    pub fn new(registry: ManifestRegistry, audit: LauncherAuditLogger) -> Self {
        Self {
            registry,
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
        required_permission: &str,
    ) -> HashMap<String, String> {
        payload.insert("status".to_string(), "prompt".to_string());
        payload.insert("app_id".to_string(), manifest.app_id.clone());
        payload.insert("display_name".to_string(), manifest.display_name.clone());
        payload.insert("required_permission".to_string(), required_permission.to_string());
        payload.insert("reason".to_string(), "permission_required".to_string());
        payload.insert("next_action".to_string(), "store_decision".to_string());
        payload.insert("trust_level".to_string(), manifest.trust_level.as_str().to_string());
        payload.insert("sandbox_profile".to_string(), manifest.sandbox_profile.clone());
        payload
    }

    fn build_terminal_payload(
        status: &str,
        message: &str,
        app_id: &str,
        required_permission: &str,
        reason: &str,
        next_action: &str,
    ) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        payload.insert("status".to_string(), status.to_string());
        payload.insert("message".to_string(), message.to_string());
        payload.insert("app_id".to_string(), app_id.to_string());
        payload.insert("required_permission".to_string(), required_permission.to_string());
        payload.insert("reason".to_string(), reason.to_string());
        payload.insert("next_action".to_string(), next_action.to_string());
        payload
    }

    fn build_runtime_payload(runtime: &crate::sandbox::ProcessIdentity) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        payload.insert("status".to_string(), runtime.state.clone());
        payload.insert("app_id".to_string(), runtime.app_id.clone());
        payload.insert("pid".to_string(), runtime.pid.to_string());
        payload.insert("state".to_string(), runtime.state.clone());
        payload.insert("launch_status".to_string(), runtime.launch_status.clone());
        payload.insert("launched_at".to_string(), runtime.launch_time.clone());
        payload.insert(
            "exited_at".to_string(),
            runtime.exited_at.clone().unwrap_or_default(),
        );
        payload.insert(
            "exit_code".to_string(),
            runtime.exit_code.map(|value| value.to_string()).unwrap_or_default(),
        );
        payload.insert("sandbox_profile".to_string(), runtime.sandbox_profile.clone());
        payload.insert("sandbox_id".to_string(), runtime.sandbox_id.clone());
        payload.insert("launched_by".to_string(), runtime.launched_by.clone());
        payload.insert(
            "stop_requested".to_string(),
            runtime.stop_requested.to_string(),
        );
        payload.insert(
            "failure_reason".to_string(),
            runtime.failure_reason.clone().unwrap_or_default(),
        );
        payload
    }

    fn stop_process_by_pid(pid: u32) -> Result<(), String> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = pid;
            Err("StopApp currently supported only on Linux".to_string())
        }

        #[cfg(target_os = "linux")]
        {
            let output = std::process::Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .output()
                .map_err(|err| format!("failed to send SIGTERM: {err}"))?;
            if output.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "kill -TERM {} failed: {}",
                    pid,
                    String::from_utf8_lossy(&output.stderr).trim()
                ))
            }
        }
    }

    async fn wait_until_not_running(&self, app_id: &str) -> bool {
        for _ in 0..15 {
            {
                let tracker = self.tracker.lock().await;
                if !tracker.is_running(app_id) {
                    return true;
                }
            }
            sleep(Duration::from_millis(200)).await;
        }
        false
    }

    fn spawn_supervision_task(
        tracker: Arc<Mutex<ProcessTracker>>,
        audit: LauncherAuditLogger,
        app_id: String,
        mut child: tokio::process::Child,
    ) {
        task::spawn(async move {
            match child.wait().await {
                Ok(status) => {
                    let exit_code = status.code();
                    let mut tracker = tracker.lock().await;
                    let prior = tracker.latest_for_app_cloned(&app_id);
                    let stop_requested = prior
                        .as_ref()
                        .map(|identity| identity.stop_requested)
                        .unwrap_or(false);
                    let failure_reason = if stop_requested || status.success() {
                        None
                    } else {
                        Some(format!(
                            "process exited with status {}",
                            exit_code
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "signal".to_string())
                        ))
                    };
                    if let Some(updated) =
                        tracker.mark_exited(&app_id, exit_code, stop_requested, failure_reason.clone())
                    {
                        drop(tracker);
                        if stop_requested {
                            let _ = audit.log_process_spawn(
                                &app_id,
                                "process_exited",
                                "stopped",
                                &format!(
                                    "pid={} exit_code={}",
                                    updated.pid,
                                    updated
                                        .exit_code
                                        .map(|value| value.to_string())
                                        .unwrap_or_else(|| "signal".to_string())
                                ),
                            );
                        } else if updated.state == "failed" {
                            let _ = audit.log_process_spawn(
                                &app_id,
                                "process_crashed",
                                "failed",
                                &format!(
                                    "pid={} exit_code={} reason={}",
                                    updated.pid,
                                    updated
                                        .exit_code
                                        .map(|value| value.to_string())
                                        .unwrap_or_else(|| "signal".to_string()),
                                    updated.failure_reason.unwrap_or_default()
                                ),
                            );
                        } else {
                            let _ = audit.log_process_spawn(
                                &app_id,
                                "process_exited",
                                "exited",
                                &format!(
                                    "pid={} exit_code={}",
                                    updated.pid,
                                    updated
                                        .exit_code
                                        .map(|value| value.to_string())
                                        .unwrap_or_default()
                                ),
                            );
                        }
                    }
                }
                Err(err) => {
                    let mut tracker = tracker.lock().await;
                    if let Some(updated) = tracker.mark_launch_failed(&app_id, err.to_string()) {
                        drop(tracker);
                        let _ = audit.log_process_spawn(
                            &app_id,
                            "process_crashed",
                            "failed",
                            &format!(
                                "pid={} reason={}",
                                updated.pid,
                                updated.failure_reason.unwrap_or_default()
                            ),
                        );
                    }
                }
            }
        });
    }

    async fn launch_internal(
        &self,
        app_id: &str,
        sender: &str,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        {
            let tracker = self.tracker.lock().await;
            if let Some(existing) = tracker.latest_for_app_cloned(app_id) {
                if existing.state == "running" || existing.state == "starting" {
                    let mut payload = Self::build_runtime_payload(&existing);
                    payload.insert("status".to_string(), "already_running".to_string());
                    payload.insert(
                        "message".to_string(),
                        format!("{} уже запущено, pid={}", app_id, existing.pid),
                    );
                    payload.insert("reason".to_string(), "already_running".to_string());
                    payload.insert("next_action".to_string(), "stop_or_focus".to_string());
                    return Ok(payload);
                }
            }
        }

        let Some(entry) = self
            .registry
            .get(app_id)
            .map_err(zbus::fdo::Error::Failed)?
        else {
            let _ = self
                .audit
                .log_launch_history(
                    app_id,
                    "launch_requested",
                    "unknown",
                    "deny",
                    "missing",
                    "unknown_manifest",
                );
            return Ok(Self::build_terminal_payload(
                "deny",
                &format!(
                    "Запуск запрещен: app_id '{}' не зарегистрирован в manifests",
                    app_id
                ),
                app_id,
                "",
                "unknown_app_id",
                "fix_manifest",
            ));
        };
        if entry.status == AppStatus::Broken {
            let _ = self.audit.log_launch_history(
                app_id,
                "controlled_launch_denied",
                entry.trust_level.as_str(),
                "deny",
                &entry.sandbox_profile,
                "app_status_broken",
            );
            return Ok(Self::build_terminal_payload(
                "manifest_invalid",
                &format!("Запуск {} заблокирован: app status=broken", app_id),
                app_id,
                "",
                "app_broken",
                "repair_or_reinstall",
            ));
        }
        let manifest = entry.to_manifest();
        let _ = self.audit.log_launch_history(
            app_id,
            "launch_requested",
            manifest.trust_level.as_str(),
            "requested",
            &manifest.sandbox_profile,
            "received",
        );
        let _ = self.audit.log_process_spawn(
            app_id,
            "manifest_validation_begin",
            "pending",
            &format!("profile={} executable={}", manifest.sandbox_profile, manifest.executable_path),
        );
        if let Err(reason) = manifest.validate_for_launch() {
            let _ = self.audit.log_process_spawn(
                app_id,
                "manifest_validation_failed",
                "manifest_invalid",
                &reason,
            );
            let _ = self.audit.log_launch_history(
                app_id,
                "controlled_launch_denied",
                manifest.trust_level.as_str(),
                "deny",
                &manifest.sandbox_profile,
                &format!("manifest_invalid:{reason}"),
            );
            return Ok(Self::security_payload(
                "manifest_invalid",
                &format!("Запуск {} заблокирован: manifest невалиден ({})", app_id, reason),
                app_id,
                "manifest_invalid",
                "fix_manifest",
                &manifest.sandbox_profile,
            ));
        }
        let _ = self.audit.log_process_spawn(app_id, "manifest_validation_ok", "ok", "");

        let launched_by = Self::resolve_launched_by(sender).await;
        let mut pending_prompt: Option<HashMap<String, String>> = None;
        let mut permission_context = HashMap::new();

        let _ = self.audit.log_process_spawn(app_id, "permission_gate_begin", "pending", "");
        for permission in &manifest.requested_permissions {
            let status = Self::check_permission(app_id, permission).await?;
            permission_context.insert(permission.clone(), status.clone());

            if status == "deny" {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "permission_gate_deny",
                    "deny",
                    permission,
                );
                let _ = self.audit.log_launch_history(
                    app_id,
                    "launch_denied",
                    manifest.trust_level.as_str(),
                    "deny",
                    &manifest.sandbox_profile,
                    "permissions_deny",
                );
                return Ok(Self::build_terminal_payload(
                    "deny",
                    &format!("Запуск {} запрещен политикой разрешений", app_id),
                    app_id,
                    permission,
                    "permission_denied",
                    "review_permission_state",
                ));
            }

            if status == "prompt" {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "permission_gate_prompt",
                    "prompt",
                    permission,
                );
                let payload = Self::request_permission(app_id, permission).await?;
                pending_prompt = Some(Self::build_prompt_payload(payload, &manifest, permission));
                let _ = self.audit.log_launch_history(
                    app_id,
                    "launch_prompted",
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
        let _ = self.audit.log_process_spawn(app_id, "permission_gate_allow", "allow", "");

        let _ = self.audit.log_process_spawn(
            app_id,
            "sandbox_profile_validation_begin",
            "pending",
            &manifest.sandbox_profile,
        );
        let validated_profile = match validate_profile_name(&manifest.sandbox_profile) {
            Ok(profile) => profile,
            Err(reason) => {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "sandbox_profile_validation_failed",
                    "profile_invalid",
                    &reason,
                );
                let _ = self.audit.log_launch_history(
                    app_id,
                    "controlled_launch_denied",
                    manifest.trust_level.as_str(),
                    "deny",
                    &manifest.sandbox_profile,
                    &format!("profile_invalid:{reason}"),
                );
                return Ok(Self::security_payload(
                    "profile_invalid",
                    &format!("Запуск {} заблокирован: sandbox profile невалиден ({})", app_id, reason),
                    app_id,
                    "profile_invalid",
                    "fix_manifest",
                    &manifest.sandbox_profile,
                ));
            }
        };
        let profile_request = SandboxLaunchRequest {
            app_id: app_id.to_string(),
            display_name: manifest.display_name.clone(),
            executable_path: manifest.executable_path.clone(),
            sandbox_profile: validated_profile.clone(),
            trust_level: manifest.trust_level.clone(),
            permission_context: permission_context.clone(),
            launched_by: launched_by.clone(),
        };
        if let Err(reason) = SandboxRunner::build_policy(&profile_request) {
            let _ = self.audit.log_process_spawn(
                app_id,
                "sandbox_profile_validation_failed",
                "profile_invalid",
                &reason,
            );
            let _ = self.audit.log_launch_history(
                app_id,
                "controlled_launch_denied",
                manifest.trust_level.as_str(),
                "deny",
                &validated_profile,
                &format!("profile_invalid:{reason}"),
            );
            return Ok(Self::security_payload(
                "profile_invalid",
                &format!("Запуск {} заблокирован: sandbox policy невалидна ({})", app_id, reason),
                app_id,
                "profile_invalid",
                "review_profile_policy",
                &validated_profile,
            ));
        }
        let _ = self.audit.log_process_spawn(app_id, "sandbox_profile_validation_ok", "ok", &validated_profile);

        let _ = self.audit.log_process_spawn(
            app_id,
            "executable_validation_begin",
            "pending",
            &manifest.executable_path,
        );
        if let Err(reason) = manifest.validate_executable() {
            let _ = self.audit.log_process_spawn(
                app_id,
                "executable_validation_failed",
                "executable_invalid",
                &reason,
            );
            let _ = self.audit.log_launch_history(
                app_id,
                "controlled_launch_denied",
                manifest.trust_level.as_str(),
                "deny",
                &validated_profile,
                &format!("executable_invalid:{reason}"),
            );
            return Ok(Self::security_payload(
                "executable_invalid",
                &format!("Запуск {} заблокирован: executable невалиден ({})", app_id, reason),
                app_id,
                "executable_invalid",
                "fix_executable_path",
                &validated_profile,
            ));
        }
        let _ = self.audit.log_process_spawn(app_id, "executable_validation_ok", "ok", "");

        let request = SandboxLaunchRequest {
            app_id: app_id.to_string(),
            display_name: manifest.display_name.clone(),
            executable_path: manifest.executable_path.clone(),
            sandbox_profile: validated_profile.clone(),
            trust_level: manifest.trust_level.clone(),
            permission_context: permission_context.clone(),
            launched_by,
        };
        let _ = self.audit.log_process_spawn(
            app_id,
            "controlled_launch_begin",
            "pending",
            &format!(
                "profile={} executable={}",
                validated_profile, manifest.executable_path
            ),
        );

        let launched = match SandboxRunner::launch(&request) {
            Ok(launched) => launched,
            Err(err) => {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "controlled_launch_failed",
                    "sandbox_failed",
                    &err,
                );
                let _ = self.audit.log_launch_history(
                    app_id,
                    "controlled_launch_failed",
                    manifest.trust_level.as_str(),
                    "failed",
                    &validated_profile,
                    &format!("sandbox_error:{err}"),
                );
                return Ok(Self::security_payload(
                    "sandbox_failed",
                    &format!("Запуск {} завершился ошибкой: {}", app_id, err),
                    app_id,
                    "sandbox_failed",
                    "inspect_failure_reason",
                    &validated_profile,
                ));
            }
        };
        let _ = self.audit.log_process_spawn(
            app_id,
            "controlled_launch_ok",
            "launched",
            &format!(
                "pid={} sandbox_id={}",
                launched.identity.pid, launched.identity.sandbox_id
            ),
        );
        let permission_summary = permission_context
            .iter()
            .map(|(permission, decision)| format!("{permission}={decision}"))
            .collect::<Vec<_>>()
            .join(",");
        let _ = self.audit.log_sandbox(&launched, &permission_summary, "started");
        let _ = self.audit.log_launch_history(
            app_id,
            "launch_allowed",
            manifest.trust_level.as_str(),
            "allow",
            &validated_profile,
            "started",
        );
        let crate::sandbox::SandboxLaunchResult {
            identity,
            runtime_path,
            applied_mounts,
            filtered_env,
            child,
        } = launched;
        {
            let mut tracker = self.tracker.lock().await;
            tracker.insert(identity.clone());
        }
        Self::spawn_supervision_task(
            Arc::clone(&self.tracker),
            self.audit.clone(),
            app_id.to_string(),
            child,
        );

        let mut payload = Self::build_runtime_payload(&identity);
        payload.insert("status".to_string(), "launched".to_string());
        payload.insert(
            "message".to_string(),
            format!(
                "{} запущено через secure launcher v2, pid={}, profile={}, source={}",
                app_id, identity.pid, validated_profile, identity.launched_by
            ),
        );
        payload.insert("required_permission".to_string(), String::new());
        payload.insert("reason".to_string(), "all_permissions_satisfied".to_string());
        payload.insert("security_outcome".to_string(), "launch_allowed".to_string());
        payload.insert("next_action".to_string(), "none".to_string());
        payload.insert("sandbox".to_string(), "bwrap".to_string());
        payload.insert("trust_level".to_string(), manifest.trust_level.as_str().to_string());
        payload.insert("runtime_path".to_string(), runtime_path);
        payload.insert("mounts".to_string(), applied_mounts.join("|"));
        payload.insert("env_allowlist".to_string(), filtered_env.join(","));
        Ok(payload)
    }
}

#[zbus::interface(name = "com.velyx.Launcher1")]
impl LauncherApi {
    async fn launch(
        &self,
        app_id: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let sender = Self::sender_from_header(&header);
        self.launch_internal(app_id, &sender).await
    }

    async fn get_app_info(&self, app_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let entry = self.registry.get(app_id).map_err(zbus::fdo::Error::Failed)?.ok_or_else(|| {
            zbus::fdo::Error::Failed(format!("app manifest not found for {app_id}"))
        })?;
        let manifest = entry.to_manifest();
        let mut payload = entry.to_map();
        Self::enrich_manifest_security(&manifest, &mut payload);
        let tracker = self.tracker.lock().await;
        payload.insert(
            "running_instances".to_string(),
            tracker.running_for_app(app_id).to_string(),
        );
        if let Some(identity) = tracker.latest_for_app(app_id) {
            payload.insert("last_pid".to_string(), identity.pid.to_string());
            payload.insert("runtime_pid".to_string(), identity.pid.to_string());
            payload.insert("last_launched_at".to_string(), identity.launch_time.clone());
            payload.insert("last_launch_status".to_string(), identity.launch_status.clone());
            payload.insert("runtime_state".to_string(), identity.state.clone());
            payload.insert(
                "runtime_exited_at".to_string(),
                identity.exited_at.clone().unwrap_or_default(),
            );
            payload.insert(
                "runtime_exit_code".to_string(),
                identity.exit_code.map(|value| value.to_string()).unwrap_or_default(),
            );
            payload.insert(
                "runtime_failure_reason".to_string(),
                identity.failure_reason.clone().unwrap_or_default(),
            );
        }
        Ok(payload)
    }

    async fn list_apps(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let mut apps = Vec::new();
        let tracker = self.tracker.lock().await;
        for entry in self.registry.list().map_err(zbus::fdo::Error::Failed)? {
            let manifest = entry.to_manifest();
            let mut payload = entry.to_map();
            Self::enrich_manifest_security(&manifest, &mut payload);
            payload.insert(
                "running_instances".to_string(),
                tracker.running_for_app(&manifest.app_id).to_string(),
            );
            if let Some(identity) = tracker.latest_for_app(&manifest.app_id) {
                payload.insert("last_pid".to_string(), identity.pid.to_string());
                payload.insert("last_launched_at".to_string(), identity.launch_time.clone());
                payload.insert("last_launch_status".to_string(), identity.launch_status.clone());
                payload.insert("runtime_state".to_string(), identity.state.clone());
                payload.insert("runtime_pid".to_string(), identity.pid.to_string());
            }
            apps.push(payload);
        }
        Ok(apps)
    }

    async fn stop_app(&self, app_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let runtime = {
            let tracker = self.tracker.lock().await;
            tracker.latest_for_app_cloned(app_id)
        };
        let Some(runtime) = runtime else {
            return Ok(Self::build_terminal_payload(
                "not_running",
                &format!("{} не запущено", app_id),
                app_id,
                "",
                "not_running",
                "launch",
            ));
        };
        if runtime.state != "running" && runtime.state != "starting" {
            let mut payload = Self::build_runtime_payload(&runtime);
            payload.insert("status".to_string(), "not_running".to_string());
            payload.insert(
                "message".to_string(),
                format!("{} не находится в running state", app_id),
            );
            payload.insert("reason".to_string(), "not_running".to_string());
            payload.insert("next_action".to_string(), "launch".to_string());
            return Ok(payload);
        }

        let _ = self.audit.log_process_spawn(
            app_id,
            "process_stop_begin",
            "pending",
            &format!("pid={}", runtime.pid),
        );
        {
            let mut tracker = self.tracker.lock().await;
            let _ = tracker.mark_stop_requested(app_id);
        }
        match Self::stop_process_by_pid(runtime.pid) {
            Ok(()) => {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "process_stop_ok",
                    "signalled",
                    &format!("pid={}", runtime.pid),
                );
                let updated = {
                    let tracker = self.tracker.lock().await;
                    tracker.latest_for_app_cloned(app_id).unwrap_or(runtime)
                };
                let mut payload = Self::build_runtime_payload(&updated);
                payload.insert("status".to_string(), "stopping".to_string());
                payload.insert(
                    "message".to_string(),
                    format!("Остановка {} запрошена, pid={}", app_id, updated.pid),
                );
                payload.insert("reason".to_string(), "stop_requested".to_string());
                payload.insert("next_action".to_string(), "poll_runtime".to_string());
                Ok(payload)
            }
            Err(err) => {
                let _ = self.audit.log_process_spawn(
                    app_id,
                    "process_stop_failed",
                    "failed",
                    &err,
                );
                Ok(Self::build_terminal_payload(
                    "failed",
                    &format!("Не удалось остановить {}: {}", app_id, err),
                    app_id,
                    "",
                    "process_stop_failed",
                    "inspect_failure_reason",
                ))
            }
        }
    }

    async fn restart_app(
        &self,
        app_id: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let sender = Self::sender_from_header(&header);
        let existing = {
            let tracker = self.tracker.lock().await;
            tracker.latest_for_app_cloned(app_id)
        };
        let _ = self.audit.log_process_spawn(
            app_id,
            "process_restart_begin",
            "pending",
            "",
        );
        if let Some(runtime) = existing {
            if runtime.state == "running" || runtime.state == "starting" {
                let stop_payload = self.stop_app(app_id).await?;
                let stop_status = stop_payload
                    .get("status")
                    .cloned()
                    .unwrap_or_else(|| "failed".to_string());
                if stop_status == "failed" {
                    let _ = self.audit.log_process_spawn(
                        app_id,
                        "process_restart_failed",
                        "failed",
                        "stop phase failed",
                    );
                    return Ok(stop_payload);
                }
                if !self.wait_until_not_running(app_id).await {
                    let _ = self.audit.log_process_spawn(
                        app_id,
                        "process_restart_failed",
                        "failed",
                        "timeout waiting for stop",
                    );
                    return Ok(Self::build_terminal_payload(
                        "failed",
                        &format!("{} не успело остановиться перед restart", app_id),
                        app_id,
                        "",
                        "restart_stop_timeout",
                        "inspect_runtime",
                    ));
                }
            }
        }

        let payload = self.launch_internal(app_id, &sender).await?;
        let result = payload
            .get("status")
            .cloned()
            .unwrap_or_else(|| "failed".to_string());
        let stage = if result == "launched" || result == "already_running" {
            "process_restart_ok"
        } else {
            "process_restart_failed"
        };
        let _ = self.audit.log_process_spawn(app_id, stage, &result, "");
        Ok(payload)
    }

    async fn get_app_runtime(&self, app_id: &str) -> zbus::fdo::Result<HashMap<String, String>> {
        let tracker = self.tracker.lock().await;
        if let Some(runtime) = tracker.latest_for_app_cloned(app_id) {
            let mut payload = Self::build_runtime_payload(&runtime);
            payload.insert("reason".to_string(), runtime.launch_status.clone());
            payload.insert("next_action".to_string(), "none".to_string());
            Ok(payload)
        } else {
            Ok(Self::build_terminal_payload(
                "idle",
                &format!("{} ещё не запускалось", app_id),
                app_id,
                "",
                "no_runtime_state",
                "launch",
            ))
        }
    }

    async fn list_running_apps(&self) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let tracker = self.tracker.lock().await;
        Ok(tracker
            .list_running()
            .into_iter()
            .map(|runtime| Self::build_runtime_payload(&runtime))
            .collect())
    }
}
