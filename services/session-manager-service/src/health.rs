use crate::model::{ServiceHealth, SessionHealthStatus};
use crate::units::{is_active, UnitDefinition};
use zbus::names::BusName;

pub async fn wait_for_service(name: &str, timeout_ms: u64) -> String {
    let connection = match zbus::Connection::session().await {
        Ok(connection) => connection,
        Err(_) => return "dbus_unavailable".to_string(),
    };
    let proxy = match zbus::fdo::DBusProxy::new(&connection).await {
        Ok(proxy) => proxy,
        Err(_) => return "dbus_proxy_unavailable".to_string(),
    };
    let bus_name = match BusName::try_from(name) {
        Ok(bus_name) => bus_name,
        Err(_) => return "invalid_bus_name".to_string(),
    };

    let mut elapsed = 0;
    while elapsed < timeout_ms {
        if proxy.get_name_owner(bus_name.clone()).await.is_ok() {
            return "available".to_string();
        }
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        elapsed += 250;
    }
    "timeout".to_string()
}

pub async fn check_required_services(units: &[UnitDefinition], timeout_ms: u64) -> Vec<ServiceHealth> {
    let mut statuses = Vec::new();
    for unit in units.iter().filter(|unit| unit.required && !unit.dbus_name.is_empty()) {
        let unit_status = is_active(&unit.unit_name)
            .await
            .unwrap_or_else(|_| "systemd_unavailable".to_string());
        let dbus_status = wait_for_service(&unit.dbus_name, timeout_ms).await;
        let status = if unit_status.trim() == "active" && dbus_status == "available" {
            "available".to_string()
        } else if unit_status.trim() != "active" {
            format!("unit_{}", unit_status.trim())
        } else {
            dbus_status
        };
        statuses.push(ServiceHealth {
            service_name: unit.dbus_name.clone(),
            required: true,
            status,
            startup_order: unit.startup_order,
            restart_policy: unit.restart_policy.clone(),
        });
    }
    statuses
}

pub async fn check_optional_services(units: &[UnitDefinition], timeout_ms: u64) -> Vec<ServiceHealth> {
    let mut statuses = Vec::new();
    for unit in units.iter().filter(|unit| !unit.required && !unit.dbus_name.is_empty()) {
        let unit_status = is_active(&unit.unit_name)
            .await
            .unwrap_or_else(|_| "systemd_unavailable".to_string());
        let dbus_status = wait_for_service(&unit.dbus_name, timeout_ms).await;
        let status = if unit_status.trim() == "active" && dbus_status == "available" {
            "available".to_string()
        } else if unit_status.trim() != "active" {
            format!("unit_{}", unit_status.trim())
        } else {
            dbus_status
        };
        statuses.push(ServiceHealth {
            service_name: unit.dbus_name.clone(),
            required: false,
            status,
            startup_order: unit.startup_order,
            restart_policy: unit.restart_policy.clone(),
        });
    }
    statuses
}

pub fn compute_session_health(
    required: &[ServiceHealth],
    optional: &[ServiceHealth],
) -> (SessionHealthStatus, Option<String>, Option<String>) {
    if let Some(failed) = required.iter().find(|entry| entry.status != "available") {
        return (
            SessionHealthStatus::Failed,
            None,
            Some(format!("required service unavailable: {}", failed.service_name)),
        );
    }

    if let Some(degraded) = optional.iter().find(|entry| entry.status != "available") {
        return (
            SessionHealthStatus::Degraded,
            Some(format!("optional service unavailable: {}", degraded.service_name)),
            None,
        );
    }

    (SessionHealthStatus::Healthy, None, None)
}
