use crate::errors::DiagnosticsError;
use crate::model::{ResourceSnapshot, ServiceHealth};
use std::fs::read_to_string;

fn read_proc_stat() -> Result<(u64, u64), DiagnosticsError> {
    let stat = read_to_string("/proc/stat")
        .map_err(|err| DiagnosticsError::Io(format!("failed to read /proc/stat: {err}")))?;
    let first = stat
        .lines()
        .next()
        .ok_or_else(|| DiagnosticsError::Parse("missing cpu line".to_string()))?;
    let values: Vec<u64> = first
        .split_whitespace()
        .skip(1)
        .filter_map(|item| item.parse::<u64>().ok())
        .collect();
    if values.len() < 4 {
        return Err(DiagnosticsError::Parse("unexpected cpu line".to_string()));
    }
    let idle = values[3];
    let total: u64 = values.iter().sum();
    Ok((idle, total))
}

fn read_meminfo() -> Result<(u64, u64), DiagnosticsError> {
    let meminfo = read_to_string("/proc/meminfo")
        .map_err(|err| DiagnosticsError::Io(format!("failed to read /proc/meminfo: {err}")))?;
    let mut total_kb = 0_u64;
    let mut available_kb = 0_u64;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
        }
        if line.starts_with("MemAvailable:") {
            available_kb = line
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
        }
    }

    if total_kb == 0 {
        return Err(DiagnosticsError::Parse("MemTotal missing".to_string()));
    }
    Ok((total_kb / 1024, (total_kb.saturating_sub(available_kb)) / 1024))
}

fn read_uptime() -> Result<u64, DiagnosticsError> {
    let uptime = read_to_string("/proc/uptime")
        .map_err(|err| DiagnosticsError::Io(format!("failed to read /proc/uptime: {err}")))?;
    let seconds = uptime
        .split_whitespace()
        .next()
        .and_then(|value| value.split('.').next())
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| DiagnosticsError::Parse("failed to parse uptime".to_string()))?;
    Ok(seconds)
}

async fn service_status(service: &str, path: &str, interface: &str) -> String {
    let connection = match zbus::Connection::session().await {
        Ok(connection) => connection,
        Err(_) => return "dbus_unavailable".to_string(),
    };

    match zbus::Proxy::new(&connection, service, path, interface).await {
        Ok(_) => "available".to_string(),
        Err(_) => "unavailable".to_string(),
    }
}

pub async fn collect_resource_snapshot() -> Result<ResourceSnapshot, DiagnosticsError> {
    let (idle, total) = read_proc_stat()?;
    let cpu_usage = if total == 0 {
        0
    } else {
        100_u64.saturating_sub((idle.saturating_mul(100)) / total)
    };

    let (memory_total_mb, memory_used_mb) = read_meminfo()?;
    let uptime_seconds = read_uptime()?;
    let memory_pressure = if memory_total_mb == 0 {
        "unknown".to_string()
    } else {
        let used_ratio = (memory_used_mb.saturating_mul(100)) / memory_total_mb;
        if used_ratio >= 85 {
            "high".to_string()
        } else if used_ratio >= 65 {
            "medium".to_string()
        } else {
            "low".to_string()
        }
    };
    let load_hint = if cpu_usage >= 80 {
        "cpu_heavy".to_string()
    } else if memory_pressure == "high" {
        "memory_heavy".to_string()
    } else {
        "normal".to_string()
    };

    Ok(ResourceSnapshot {
        cpu_usage_percent: cpu_usage.to_string(),
        memory_total_mb: memory_total_mb.to_string(),
        memory_used_mb: memory_used_mb.to_string(),
        memory_pressure,
        uptime_seconds: uptime_seconds.to_string(),
        load_hint,
    })
}

pub async fn collect_service_health() -> ServiceHealth {
    ServiceHealth {
        launcher_service_status: service_status("com.velyx.Launcher", "/com/velyx/Launcher", "com.velyx.Launcher1").await,
        permissions_service_status: service_status("com.velyx.Permissions", "/com/velyx/Permissions", "com.velyx.Permissions1").await,
        ai_service_status: service_status("com.velyx.AI", "/com/velyx/AI", "com.velyx.AI1").await,
        settings_service_status: service_status("com.velyx.Settings", "/com/velyx/Settings", "com.velyx.Settings1").await,
    }
}
