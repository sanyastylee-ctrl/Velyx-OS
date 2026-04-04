#[derive(Clone, Debug)]
pub struct SeccompProfile {
    pub id: String,
    pub mode: String,
}

pub fn placeholder_for_profile(profile: &str) -> SeccompProfile {
    SeccompProfile {
        id: format!("seccomp-placeholder-{profile}"),
        mode: "planned".to_string(),
    }
}
