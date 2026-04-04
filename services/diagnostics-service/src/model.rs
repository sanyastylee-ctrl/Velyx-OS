use std::collections::HashMap;

pub struct ResourceSnapshot {
    pub cpu_usage_percent: String,
    pub memory_total_mb: String,
    pub memory_used_mb: String,
    pub memory_pressure: String,
    pub uptime_seconds: String,
    pub load_hint: String,
}

pub struct ServiceHealth {
    pub launcher_service_status: String,
    pub permissions_service_status: String,
    pub ai_service_status: String,
    pub settings_service_status: String,
}

pub struct SummaryModel {
    pub human_summary: String,
    pub cpu_state: String,
    pub memory_state: String,
    pub hottest_component: String,
    pub service_health: String,
}

impl ResourceSnapshot {
    pub fn to_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("cpu_usage_percent".to_string(), self.cpu_usage_percent.clone()),
            ("memory_total_mb".to_string(), self.memory_total_mb.clone()),
            ("memory_used_mb".to_string(), self.memory_used_mb.clone()),
            ("memory_pressure".to_string(), self.memory_pressure.clone()),
            ("uptime_seconds".to_string(), self.uptime_seconds.clone()),
            ("load_hint".to_string(), self.load_hint.clone()),
        ])
    }
}

impl ServiceHealth {
    pub fn to_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("launcher_service_status".to_string(), self.launcher_service_status.clone()),
            ("permissions_service_status".to_string(), self.permissions_service_status.clone()),
            ("ai_service_status".to_string(), self.ai_service_status.clone()),
            ("settings_service_status".to_string(), self.settings_service_status.clone()),
        ])
    }
}

impl SummaryModel {
    pub fn to_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("human_summary".to_string(), self.human_summary.clone()),
            ("cpu_state".to_string(), self.cpu_state.clone()),
            ("memory_state".to_string(), self.memory_state.clone()),
            ("hottest_component".to_string(), self.hottest_component.clone()),
            ("service_health".to_string(), self.service_health.clone()),
        ])
    }
}
