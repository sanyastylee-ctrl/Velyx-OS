use crate::audit::DiagnosticsAuditLogger;
use crate::collectors::{collect_resource_snapshot, collect_service_health};
use crate::summary::build_summary;
use zbus::message::Header;

pub struct DiagnosticsApi {
    audit: DiagnosticsAuditLogger,
    user_id: String,
}

impl DiagnosticsApi {
    pub fn new(audit: DiagnosticsAuditLogger, user_id: String) -> Self {
        Self { audit, user_id }
    }

    fn audit_request(&self, action: &str, result: &str) {
        if let Err(err) = self.audit.log(&self.user_id, action, result) {
            eprintln!("[diagnostics-service] audit_error={err}");
        }
    }
}

#[zbus::interface(name = "com.velyx.Diagnostics1")]
impl DiagnosticsApi {
    async fn get_system_summary(&self, #[zbus(header)] _header: Header<'_>) -> zbus::fdo::Result<std::collections::HashMap<String, String>> {
        match collect_resource_snapshot().await {
            Ok(snapshot) => {
                let health = collect_service_health().await;
                let summary = build_summary(&snapshot, &health).to_map();
                self.audit_request("GetSystemSummary", "success");
                Ok(summary)
            }
            Err(err) => {
                self.audit_request("GetSystemSummary", "error");
                Err(zbus::fdo::Error::Failed(err.message()))
            }
        }
    }

    async fn get_resource_snapshot(&self, #[zbus(header)] _header: Header<'_>) -> zbus::fdo::Result<std::collections::HashMap<String, String>> {
        match collect_resource_snapshot().await {
            Ok(snapshot) => {
                self.audit_request("GetResourceSnapshot", "success");
                Ok(snapshot.to_map())
            }
            Err(err) => {
                self.audit_request("GetResourceSnapshot", "error");
                Err(zbus::fdo::Error::Failed(err.message()))
            }
        }
    }

    async fn get_service_health(&self, #[zbus(header)] _header: Header<'_>) -> zbus::fdo::Result<std::collections::HashMap<String, String>> {
        let health = collect_service_health().await;
        self.audit_request("GetServiceHealth", "success");
        Ok(health.to_map())
    }
}
