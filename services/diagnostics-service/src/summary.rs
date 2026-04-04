use crate::model::{ResourceSnapshot, ServiceHealth, SummaryModel};

pub fn build_summary(snapshot: &ResourceSnapshot, health: &ServiceHealth) -> SummaryModel {
    let cpu_value = snapshot.cpu_usage_percent.parse::<u64>().unwrap_or(0);
    let memory_pressure = snapshot.memory_pressure.clone();

    let cpu_state = if cpu_value >= 80 {
        "high"
    } else if cpu_value >= 50 {
        "medium"
    } else {
        "low"
    }
    .to_string();

    let memory_state = memory_pressure.clone();
    let hottest_component = if cpu_state == "high" {
        "cpu".to_string()
    } else if memory_state == "high" {
        "memory".to_string()
    } else {
        "none".to_string()
    };

    let healthy_services = [
        &health.launcher_service_status,
        &health.permissions_service_status,
        &health.ai_service_status,
        &health.settings_service_status,
    ]
    .iter()
    .filter(|status| ***status == "available")
    .count();

    SummaryModel {
        human_summary: format!(
            "CPU: {}%, память: {}/{} МБ, состояние сервисов: {} из 4 доступны.",
            snapshot.cpu_usage_percent,
            snapshot.memory_used_mb,
            snapshot.memory_total_mb,
            healthy_services
        ),
        cpu_state,
        memory_state,
        hottest_component,
        service_health: if healthy_services == 4 {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
    }
}
