use crate::sandbox::ProcessIdentity;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProcessTracker {
    processes: HashMap<String, ProcessIdentity>,
}

impl ProcessTracker {
    pub fn insert(&mut self, identity: ProcessIdentity) {
        self.processes.insert(identity.app_id.clone(), identity);
    }

    pub fn running_for_app(&self, app_id: &str) -> usize {
        self.processes
            .get(app_id)
            .filter(|identity| identity.state == "running" || identity.state == "starting")
            .map(|_| 1usize)
            .unwrap_or(0)
    }

    pub fn latest_for_app(&self, app_id: &str) -> Option<&ProcessIdentity> {
        self.processes.get(app_id)
    }

    pub fn latest_for_app_cloned(&self, app_id: &str) -> Option<ProcessIdentity> {
        self.processes.get(app_id).cloned()
    }

    pub fn list_known(&self) -> Vec<ProcessIdentity> {
        let mut entries = self.processes.values().cloned().collect::<Vec<_>>();
        entries.sort_by(|left, right| left.app_id.cmp(&right.app_id));
        entries
    }

    pub fn list_running(&self) -> Vec<ProcessIdentity> {
        let mut entries = self
            .processes
            .values()
            .filter(|identity| identity.state == "running" || identity.state == "starting")
            .cloned()
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| left.app_id.cmp(&right.app_id));
        entries
    }

    pub fn is_running(&self, app_id: &str) -> bool {
        self.latest_for_app(app_id)
            .map(|identity| identity.state == "running" || identity.state == "starting")
            .unwrap_or(false)
    }

    pub fn mark_stop_requested(&mut self, app_id: &str) -> Option<ProcessIdentity> {
        let identity = self.processes.get_mut(app_id)?;
        identity.stop_requested = true;
        identity.launch_status = "stop_requested".to_string();
        Some(identity.clone())
    }

    pub fn mark_exited(
        &mut self,
        app_id: &str,
        exit_code: Option<i32>,
        stopped: bool,
        failure_reason: Option<String>,
    ) -> Option<ProcessIdentity> {
        let identity = self.processes.get_mut(app_id)?;
        identity.exited_at = Some(Utc::now().to_rfc3339());
        identity.exit_code = exit_code;
        identity.failure_reason = failure_reason.clone();
        identity.state = if stopped {
            "stopped".to_string()
        } else if exit_code.unwrap_or_default() == 0 {
            "exited".to_string()
        } else {
            "failed".to_string()
        };
        identity.launch_status = identity.state.clone();
        Some(identity.clone())
    }

    pub fn mark_launch_failed(&mut self, app_id: &str, failure_reason: String) -> Option<ProcessIdentity> {
        let identity = self.processes.get_mut(app_id)?;
        identity.exited_at = Some(Utc::now().to_rfc3339());
        identity.exit_code = None;
        identity.state = "failed".to_string();
        identity.launch_status = "failed".to_string();
        identity.failure_reason = Some(failure_reason);
        Some(identity.clone())
    }
}
