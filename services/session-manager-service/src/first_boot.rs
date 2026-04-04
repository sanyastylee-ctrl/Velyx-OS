use crate::audit::SessionAuditLogger;
use crate::errors::SessionManagerError;
use crate::handoff::{InstallHandoffState, InstallHandoffStore};
use crate::model::{FirstBootSnapshot, FirstBootState, StartupOutcome, UserBootstrapRecord};
use crate::startup::bootstrap_session;
use crate::state::SessionStateStore;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FirstBootStore {
    state_path: PathBuf,
    user_bootstrap_path: PathBuf,
    snapshot: FirstBootSnapshot,
}

impl FirstBootStore {
    pub fn load(base_dir: &Path) -> Self {
        let _ = fs::create_dir_all(base_dir);
        let state_path = base_dir.join("first_boot_state.json");
        let user_bootstrap_path = base_dir.join("user_bootstrap.json");
        let snapshot = fs::read_to_string(&state_path)
            .ok()
            .and_then(|raw| serde_json::from_str::<FirstBootSnapshot>(&raw).ok())
            .unwrap_or_default();
        Self {
            state_path,
            user_bootstrap_path,
            snapshot,
        }
    }

    pub fn snapshot(&self) -> FirstBootSnapshot {
        self.snapshot.clone()
    }

    pub fn replace_from_handoff(&mut self, handoff: &InstallHandoffState) -> Result<(), String> {
        self.snapshot = FirstBootSnapshot {
            state: FirstBootState::Pending,
            install_id: handoff.install_id.clone(),
            target_id: handoff.target_id.clone(),
            requested_username: handoff.requested_username.clone(),
            requested_locale: handoff.requested_locale.clone(),
            baseline_settings_applied: false,
            user_created: false,
            handoff_ready: false,
            completed_at: None,
            failed_reason: None,
        };
        self.persist_state()
    }

    pub fn update_state(&mut self, state: FirstBootState) -> Result<(), String> {
        self.snapshot.state = state;
        self.persist_state()
    }

    pub fn mark_user_created(&mut self) -> Result<(), String> {
        self.snapshot.user_created = true;
        self.persist_state()
    }

    pub fn mark_baseline_applied(&mut self) -> Result<(), String> {
        self.snapshot.baseline_settings_applied = true;
        self.persist_state()
    }

    pub fn mark_handoff_ready(&mut self) -> Result<(), String> {
        self.snapshot.handoff_ready = true;
        self.persist_state()
    }

    pub fn mark_completed(&mut self) -> Result<(), String> {
        self.snapshot.state = FirstBootState::Completed;
        self.snapshot.completed_at = Some(Utc::now().to_rfc3339());
        self.snapshot.failed_reason = None;
        self.persist_state()
    }

    pub fn mark_failed(&mut self, reason: String) -> Result<(), String> {
        self.snapshot.state = FirstBootState::Failed;
        self.snapshot.failed_reason = Some(reason);
        self.persist_state()
    }

    pub fn write_user_bootstrap(
        &self,
        user_id: &str,
        username: &str,
        locale: &str,
    ) -> Result<(), String> {
        let record = UserBootstrapRecord {
            user_id: user_id.to_string(),
            username: username.to_string(),
            locale: locale.to_string(),
            created_via_first_boot: true,
            home_state_initialized: true,
            created_at: Utc::now().to_rfc3339(),
        };
        persist_json(&self.user_bootstrap_path, &record)
    }

    fn persist_state(&self) -> Result<(), String> {
        persist_json(&self.state_path, &self.snapshot)
    }
}

pub fn has_pending_first_boot(base_dir: &Path) -> bool {
    let handoff = InstallHandoffStore::load(base_dir).state();
    handoff.map(|entry| entry.first_boot_pending).unwrap_or(false)
}

pub async fn run_first_boot_flow(
    base_dir: &Path,
    store: &mut SessionStateStore,
    audit: &SessionAuditLogger,
    user_id_hint: &str,
) -> Result<StartupOutcome, SessionManagerError> {
    let mut handoff_store = InstallHandoffStore::load(base_dir);
    let mut first_boot_store = FirstBootStore::load(base_dir);
    let handoff = handoff_store
        .state()
        .ok_or_else(|| SessionManagerError::FirstBootFailed("install handoff missing".to_string()))?;
    if !handoff.first_boot_pending {
        return Err(SessionManagerError::FirstBootFailed(
            "first boot handoff is not pending".to_string(),
        ));
    }

    audit.log_transition(
        "first_boot_started",
        "none",
        FirstBootState::InitialSetupStarted.as_str(),
        &handoff.requested_username,
        "first_boot",
        "ok",
        "source=installer_handoff",
    )
    .map_err(SessionManagerError::StartupFailed)?;
    first_boot_store
        .replace_from_handoff(&handoff)
        .map_err(SessionManagerError::FirstBootFailed)?;
    first_boot_store
        .update_state(FirstBootState::InitialSetupStarted)
        .map_err(SessionManagerError::FirstBootFailed)?;

    first_boot_store
        .update_state(FirstBootState::UserCreationPending)
        .map_err(SessionManagerError::FirstBootFailed)?;
    audit.log_transition(
        "first_boot_user_creation_begin",
        FirstBootState::InitialSetupStarted.as_str(),
        FirstBootState::UserCreationPending.as_str(),
        &handoff.requested_username,
        "first_boot",
        "ok",
        "creating bootstrap user record",
    )
    .map_err(SessionManagerError::StartupFailed)?;
    let effective_user_id = if user_id_hint.is_empty() {
        handoff.requested_username.as_str()
    } else {
        user_id_hint
    };
    first_boot_store
        .write_user_bootstrap(
            effective_user_id,
            &handoff.requested_username,
            &handoff.requested_locale,
        )
        .map_err(SessionManagerError::FirstBootFailed)?;
    first_boot_store
        .mark_user_created()
        .map_err(SessionManagerError::FirstBootFailed)?;
    audit.log_transition(
        "first_boot_user_creation_ok",
        FirstBootState::UserCreationPending.as_str(),
        FirstBootState::BaselineConfigPending.as_str(),
        effective_user_id,
        "first_boot",
        "ok",
        "user bootstrap record written",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    first_boot_store
        .update_state(FirstBootState::BaselineConfigPending)
        .map_err(SessionManagerError::FirstBootFailed)?;
    audit.log_transition(
        "first_boot_baseline_apply_begin",
        FirstBootState::UserCreationPending.as_str(),
        FirstBootState::BaselineConfigPending.as_str(),
        effective_user_id,
        "settings-service",
        "ok",
        "applying baseline settings via settings-service",
    )
    .map_err(SessionManagerError::StartupFailed)?;
    apply_baseline_settings(&handoff, effective_user_id)
        .await
        .map_err(|err| {
            let reason = err.message();
            let _ = first_boot_store.mark_failed(reason.clone());
            let _ = audit.log_transition(
                "first_boot_failed",
                FirstBootState::BaselineConfigPending.as_str(),
                FirstBootState::Failed.as_str(),
                effective_user_id,
                "settings-service",
                "failed",
                &reason,
            );
            SessionManagerError::FirstBootFailed(reason)
        })?;
    first_boot_store
        .mark_baseline_applied()
        .map_err(SessionManagerError::FirstBootFailed)?;
    audit.log_transition(
        "first_boot_baseline_apply_ok",
        FirstBootState::BaselineConfigPending.as_str(),
        FirstBootState::ServiceBootstrapPending.as_str(),
        effective_user_id,
        "settings-service",
        "ok",
        "baseline settings applied via settings-service",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    first_boot_store
        .update_state(FirstBootState::ServiceBootstrapPending)
        .map_err(SessionManagerError::FirstBootFailed)?;
    first_boot_store
        .update_state(FirstBootState::HandoffToSessionPending)
        .map_err(SessionManagerError::FirstBootFailed)?;
    first_boot_store
        .mark_handoff_ready()
        .map_err(SessionManagerError::FirstBootFailed)?;
    audit.log_transition(
        "first_boot_handoff_begin",
        FirstBootState::ServiceBootstrapPending.as_str(),
        FirstBootState::HandoffToSessionPending.as_str(),
        effective_user_id,
        "session-manager",
        "ok",
        "handoff to session runtime",
    )
    .map_err(SessionManagerError::StartupFailed)?;

    let outcome = bootstrap_session(store, audit, effective_user_id, "first_boot")
        .await
        .map_err(|err| {
            let reason = err.message();
            let _ = first_boot_store.mark_failed(reason.clone());
            let _ = audit.log_transition(
                "first_boot_failed",
                FirstBootState::HandoffToSessionPending.as_str(),
                FirstBootState::Failed.as_str(),
                effective_user_id,
                "session-manager",
                "failed",
                &reason,
            );
            SessionManagerError::FirstBootFailed(reason)
        })?;

    match outcome.state {
        crate::model::SessionState::Ready | crate::model::SessionState::Degraded => {
            let mut updated_handoff = handoff.clone();
            updated_handoff.first_boot_pending = false;
            updated_handoff.baseline_settings_pending = false;
            updated_handoff.session_start_pending = false;
            handoff_store
                .update(updated_handoff)
                .map_err(SessionManagerError::FirstBootFailed)?;
            first_boot_store
                .mark_completed()
                .map_err(SessionManagerError::FirstBootFailed)?;
            audit.log_transition(
                "first_boot_handoff_ok",
                FirstBootState::HandoffToSessionPending.as_str(),
                FirstBootState::HandoffToSessionPending.as_str(),
                effective_user_id,
                "session-manager",
                outcome.health.as_str(),
                "session handoff completed",
            )
            .map_err(SessionManagerError::StartupFailed)?;
            audit.log_transition(
                "first_boot_completed",
                FirstBootState::HandoffToSessionPending.as_str(),
                FirstBootState::Completed.as_str(),
                effective_user_id,
                "session-manager",
                outcome.health.as_str(),
                "first boot lifecycle completed",
            )
            .map_err(SessionManagerError::StartupFailed)?;
            Ok(outcome)
        }
        _ => {
            let reason = outcome
                .failed_reason
                .clone()
                .unwrap_or_else(|| "session handoff failed".to_string());
            first_boot_store
                .mark_failed(reason.clone())
                .map_err(SessionManagerError::FirstBootFailed)?;
            audit.log_transition(
                "first_boot_failed",
                FirstBootState::HandoffToSessionPending.as_str(),
                FirstBootState::Failed.as_str(),
                effective_user_id,
                "session-manager",
                "failed",
                &reason,
            )
            .map_err(SessionManagerError::StartupFailed)?;
            Err(SessionManagerError::FirstBootFailed(reason))
        }
    }
}

async fn apply_baseline_settings(
    handoff: &InstallHandoffState,
    user_id: &str,
) -> Result<(), SessionManagerError> {
    let connection = zbus::Connection::session()
        .await
        .map_err(|_| SessionManagerError::FirstBootFailed("settings bus unavailable".to_string()))?;
    let proxy = zbus::Proxy::new(
        &connection,
        "com.velyx.Settings",
        "/com/velyx/Settings",
        "com.velyx.Settings1",
    )
    .await
    .map_err(|err| SessionManagerError::FirstBootFailed(format!("settings proxy init failed: {err}")))?;

    let ai_mode = match handoff.profile_id.as_str() {
        "gaming-ready" => "disabled",
        _ => "local_only",
    };
    let ai_enabled = if ai_mode == "disabled" { "false" } else { "true" };
    let entries = [
        ("appearance.theme", "dark"),
        ("system.locale", handoff.requested_locale.as_str()),
        ("ai.enabled", ai_enabled),
        ("ai.privacy_mode", ai_mode),
        ("bluetooth.enabled", "false"),
    ];

    for (key, value) in entries {
        let success: bool = proxy
            .call("SetValue", &(key, value))
            .await
            .map_err(|err| {
                SessionManagerError::FirstBootFailed(format!(
                    "baseline settings apply failed for {}: {}",
                    key, err
                ))
            })?;
        if !success {
            return Err(SessionManagerError::FirstBootFailed(format!(
                "baseline settings apply returned false for {} user={}",
                key, user_id
            )));
        }
    }

    Ok(())
}

fn persist_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(value)
        .map_err(|err| format!("first boot serialize failed: {err}"))?;
    fs::write(&tmp, raw).map_err(|err| format!("first boot temp write failed: {err}"))?;
    fs::rename(&tmp, path).map_err(|err| format!("first boot rename failed: {err}"))
}
