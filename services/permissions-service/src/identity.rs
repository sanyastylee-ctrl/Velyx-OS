use std::collections::HashMap;

#[derive(Default)]
pub struct IdentityTracker {
    last_seen: HashMap<String, String>,
}

pub struct IdentityCheck {
    pub mismatch: bool,
    pub first_seen: bool,
    pub previous_app_id: Option<String>,
}

impl IdentityTracker {
    pub fn observe(&mut self, sender: &str, app_id: &str) -> IdentityCheck {
        match self.last_seen.get(sender) {
            Some(previous) if previous != app_id => {
                let previous_app_id = previous.clone();
                self.last_seen
                    .insert(sender.to_string(), app_id.to_string());
                IdentityCheck {
                    mismatch: true,
                    first_seen: false,
                    previous_app_id: Some(previous_app_id),
                }
            }
            Some(_) => IdentityCheck {
                mismatch: false,
                first_seen: false,
                previous_app_id: None,
            },
            None => {
                self.last_seen
                    .insert(sender.to_string(), app_id.to_string());
                IdentityCheck {
                    mismatch: false,
                    first_seen: true,
                    previous_app_id: None,
                }
            }
        }
    }
}
