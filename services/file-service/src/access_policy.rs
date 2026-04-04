use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct AccessDecision {
    pub allowed: bool,
    pub access_level: String,
    pub reason: String,
}

pub fn metadata_read_allowed(path: &Path) -> AccessDecision {
    if visible_roots().iter().any(|root| path.starts_with(root)) {
        return AccessDecision {
            allowed: true,
            access_level: "metadata_only".to_string(),
            reason: "path_within_visible_roots".to_string(),
        };
    }

    AccessDecision {
        allowed: false,
        access_level: "denied".to_string(),
        reason: "path_outside_visible_roots".to_string(),
    }
}

pub fn visible_roots() -> Vec<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let home = PathBuf::from(home);
    vec![
        home.join("Desktop"),
        home.join("Documents"),
        home.join("Downloads"),
        home.join("Pictures"),
    ]
}

pub fn future_content_read_policy_placeholder() -> &'static str {
    "future_content_read_requires_portal_or_permission"
}

pub fn future_file_picker_portal_placeholder() -> &'static str {
    "future_file_picker_portal"
}
