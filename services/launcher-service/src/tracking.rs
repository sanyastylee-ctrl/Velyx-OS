use crate::sandbox::ProcessIdentity;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProcessTracker {
    processes: HashMap<u32, ProcessIdentity>,
}

impl ProcessTracker {
    pub fn insert(&mut self, identity: ProcessIdentity) {
        self.processes.insert(identity.pid, identity);
    }

    pub fn running_for_app(&self, app_id: &str) -> usize {
        self.processes
            .values()
            .filter(|identity| identity.app_id == app_id)
            .count()
    }

    pub fn latest_for_app(&self, app_id: &str) -> Option<&ProcessIdentity> {
        self.processes
            .values()
            .filter(|identity| identity.app_id == app_id)
            .max_by_key(|identity| identity.launch_time.as_str())
    }
}
