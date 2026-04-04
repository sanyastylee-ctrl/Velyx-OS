use crate::identity::{IdentityCheck, IdentityTracker};
use crate::model::{CheckResult, Decision, PermissionKind};
use crate::logger::AuditLogger;
use crate::policy::{evaluate, trust_level_for_app, PolicyContext};
use crate::store::PermissionStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::message::Header;

pub struct PermissionsApi {
    store: Arc<Mutex<PermissionStore>>,
    identities: Arc<Mutex<IdentityTracker>>,
    logger: AuditLogger,
    user_id: String,
}

impl PermissionsApi {
    pub fn new(store: PermissionStore, logger: AuditLogger, user_id: String) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            identities: Arc::new(Mutex::new(IdentityTracker::default())),
            logger,
            user_id,
        }
    }

    fn sender_from_header(header: &Header<'_>) -> String {
        header
            .sender()
            .map(|sender| sender.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    fn write_audit(
        &self,
        app_id: &str,
        permission: &str,
        action: &str,
        result: &str,
        sender: &str,
        trust_level: &str,
        policy_decision_source: &str,
    ) {
        if let Err(err) = self
            .logger
            .log(
                app_id,
                permission,
                action,
                result,
                sender,
                &self.user_id,
                trust_level,
                policy_decision_source,
            )
        {
            eprintln!("[permissions-service] audit_error={err}");
        }
    }

    async fn observe_identity(&self, sender: &str, app_id: &str) -> IdentityCheck {
        let mut identities = self.identities.lock().await;
        identities.observe(sender, app_id)
    }

    fn audit_identity_event(
        &self,
        app_id: &str,
        permission: &str,
        sender: &str,
        trust_level: &str,
        identity: &IdentityCheck,
    ) {
        if identity.first_seen {
            self.write_audit(
                app_id,
                permission,
                "identity_bind",
                "first_seen_sender_binding",
                sender,
                trust_level,
                "policy",
            );
        }

        if identity.mismatch {
            let previous = identity
                .previous_app_id
                .clone()
                .unwrap_or_else(|| "<unknown>".to_string());
            self.write_audit(
                app_id,
                permission,
                "identity_warning",
                &format!("sender_app_mismatch previous_app_id={previous}"),
                sender,
                trust_level,
                "policy",
            );
        }
    }

    fn parse_permission(permission: &str) -> zbus::fdo::Result<PermissionKind> {
        PermissionKind::from_str(permission).ok_or_else(|| {
            zbus::fdo::Error::InvalidArgs(format!("unknown permission '{}'", permission))
        })
    }

    fn parse_decision(decision: &str) -> zbus::fdo::Result<Decision> {
        Decision::from_str(decision).ok_or_else(|| {
            zbus::fdo::Error::InvalidArgs(format!("unknown decision '{}'", decision))
        })
    }
}

#[zbus::interface(name = "com.velyx.Permissions1")]
impl PermissionsApi {
    async fn check_permission(
        &self,
        app_id: &str,
        permission: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<String> {
        let permission_kind = Self::parse_permission(permission)?;
        let sender = Self::sender_from_header(&header);
        let trust_level = trust_level_for_app(app_id);
        let identity = self.observe_identity(&sender, app_id).await;
        self.audit_identity_event(
            app_id,
            permission_kind.as_str(),
            &sender,
            trust_level.as_str(),
            &identity,
        );

        let store = self.store.lock().await;
        let store_result = store.check_permission(&self.user_id, app_id, &permission_kind);
        drop(store);

        let context = PolicyContext {
            app_id: app_id.to_string(),
            sender,
            trust_level: trust_level.clone(),
            sender_mismatch: identity.mismatch,
        };
        let decision = evaluate(store_result, &context);
        self.write_audit(
            app_id,
            permission_kind.as_str(),
            "check",
            &decision.result.to_string(),
            &context.sender,
            trust_level.as_str(),
            decision.source.as_str(),
        );
        Ok(decision.result.to_string())
    }

    async fn request_permission(
        &self,
        app_id: &str,
        permission: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let permission_kind = Self::parse_permission(permission)?;
        let sender = Self::sender_from_header(&header);
        let trust_level = trust_level_for_app(app_id);
        let identity = self.observe_identity(&sender, app_id).await;
        self.audit_identity_event(
            app_id,
            permission_kind.as_str(),
            &sender,
            trust_level.as_str(),
            &identity,
        );
        let store = self.store.lock().await;
        if !store.is_available() {
            drop(store);
            self.write_audit(
                app_id,
                permission_kind.as_str(),
                "request",
                "deny_store_unavailable",
                &sender,
                trust_level.as_str(),
                "policy",
            );
            return Err(zbus::fdo::Error::Failed("permission store unavailable".into()));
        }
        drop(store);
        self.write_audit(
            app_id,
            permission_kind.as_str(),
            "request",
            "prompt",
            &sender,
            trust_level.as_str(),
            if identity.mismatch { "policy" } else { "default" },
        );

        let mut payload = HashMap::new();
        payload.insert("user_id".to_string(), self.user_id.clone());
        payload.insert("app_id".to_string(), app_id.to_string());
        payload.insert("permission".to_string(), permission_kind.as_str().to_string());
        payload.insert("trust_level".to_string(), trust_level.as_str().to_string());
        payload.insert(
            "permission_display".to_string(),
            permission_kind.display_name().to_string(),
        );
        payload.insert(
            "explanation".to_string(),
            permission_kind.explanation().to_string(),
        );
        payload.insert("status".to_string(), "prompt".to_string());

        Ok(payload)
    }

    async fn store_decision(
        &self,
        app_id: &str,
        permission: &str,
        decision: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<bool> {
        let permission_kind = Self::parse_permission(permission)?;
        let parsed_decision = Self::parse_decision(decision)?;
        let sender = Self::sender_from_header(&header);
        let trust_level = trust_level_for_app(app_id);
        let identity = self.observe_identity(&sender, app_id).await;
        self.audit_identity_event(
            app_id,
            permission_kind.as_str(),
            &sender,
            trust_level.as_str(),
            &identity,
        );
        let mut store = self.store.lock().await;
        store
            .save_decision(
                &self.user_id,
                app_id,
                permission_kind.clone(),
                parsed_decision.clone(),
            )
            .map_err(zbus::fdo::Error::Failed)?;
        self.write_audit(
            app_id,
            permission_kind.as_str(),
            "store",
            parsed_decision.as_status(),
            &sender,
            trust_level.as_str(),
            "store",
        );
        Ok(true)
    }

    async fn reset_permissions(
        &self,
        app_id: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<u32> {
        let sender = Self::sender_from_header(&header);
        let trust_level = trust_level_for_app(app_id);
        let identity = self.observe_identity(&sender, app_id).await;
        self.audit_identity_event(app_id, "*", &sender, trust_level.as_str(), &identity);
        let mut store = self.store.lock().await;
        let removed = store
            .reset_permissions(&self.user_id, app_id)
            .map_err(zbus::fdo::Error::Failed)?;
        self.write_audit(
            app_id,
            "*",
            "reset",
            &format!("removed:{removed}"),
            &sender,
            trust_level.as_str(),
            "store",
        );
        Ok(removed as u32)
    }
}
