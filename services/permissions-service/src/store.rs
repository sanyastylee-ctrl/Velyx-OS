use crate::model::{CheckResult, Decision, PermissionKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, rename, remove_file, write};
use std::path::{Path, PathBuf};

#[derive(Default, Serialize, Deserialize)]
struct PersistedPermissions {
    users: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

pub struct PermissionStore {
    decisions: HashMap<(String, String, PermissionKind), Decision>,
    path: PathBuf,
    available: bool,
}

impl PermissionStore {
    pub fn load(base_dir: &Path) -> Self {
        let path = base_dir.join("permissions.json");
        if let Err(err) = create_dir_all(base_dir) {
            eprintln!("[permissions-service] store_error=create_dir_failed err={err}");
            return Self {
                decisions: HashMap::new(),
                path,
                available: false,
            };
        }

        let decisions = match read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<PersistedPermissions>(&content) {
                Ok(parsed) => Self::from_persisted(parsed),
                Err(err) => {
                    eprintln!("[permissions-service] store_error=invalid_json path={} err={err}", path.display());
                    HashMap::new()
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
            Err(err) => {
                eprintln!("[permissions-service] store_error=read_failed path={} err={err}", path.display());
                return Self {
                    decisions: HashMap::new(),
                    path,
                    available: false,
                };
            }
        };

        Self {
            decisions,
            path,
            available: true,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn check_permission(
        &self,
        user_id: &str,
        app_id: &str,
        permission: &PermissionKind,
    ) -> CheckResult {
        if !self.available {
            return CheckResult::Deny;
        }

        match self
            .decisions
            .get(&(user_id.to_string(), app_id.to_string(), permission.clone()))
        {
            Some(Decision::Allow) => CheckResult::Allow,
            Some(Decision::Deny) => CheckResult::Deny,
            None => CheckResult::Prompt,
        }
    }

    pub fn get_permission_state(
        &self,
        user_id: &str,
        app_id: &str,
        permission: &PermissionKind,
    ) -> Option<Decision> {
        self.decisions
            .get(&(user_id.to_string(), app_id.to_string(), permission.clone()))
            .cloned()
    }

    pub fn list_app_permissions(
        &self,
        user_id: &str,
        app_id: &str,
    ) -> HashMap<String, String> {
        let mut permissions = HashMap::new();
        for permission in [
            PermissionKind::Camera,
            PermissionKind::Microphone,
            PermissionKind::Filesystem,
            PermissionKind::ScreenCapture,
        ] {
            let value = self
                .get_permission_state(user_id, app_id, &permission)
                .map(|decision| decision.as_status().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            permissions.insert(permission.as_str().to_string(), value);
        }
        permissions
    }

    pub fn save_decision(
        &mut self,
        user_id: &str,
        app_id: &str,
        permission: PermissionKind,
        decision: Decision,
    ) -> Result<(), String> {
        if !self.available {
            return Err("permission store unavailable".to_string());
        }

        self.decisions
            .insert((user_id.to_string(), app_id.to_string(), permission), decision);
        self.persist()
    }

    pub fn reset_permissions(&mut self, user_id: &str, app_id: &str) -> Result<usize, String> {
        if !self.available {
            return Err("permission store unavailable".to_string());
        }

        let before = self.decisions.len();
        self.decisions.retain(|(stored_user_id, stored_app_id, _), _| {
            stored_user_id != user_id || stored_app_id != app_id
        });
        let removed = before.saturating_sub(self.decisions.len());
        self.persist()?;
        Ok(removed)
    }

    fn persist(&mut self) -> Result<(), String> {
        let payload = Self::to_persisted(&self.decisions);
        let encoded = serde_json::to_string_pretty(&payload)
            .map_err(|err| format!("failed to encode permissions store: {err}"))?;

        let temp_path = self.path.with_extension("json.tmp");
        write(&temp_path, encoded.as_bytes())
            .map_err(|err| format!("failed to write temp store: {err}"))?;

        if self.path.exists() {
            remove_file(&self.path)
                .map_err(|err| format!("failed to replace permissions store: {err}"))?;
        }

        rename(&temp_path, &self.path)
            .map_err(|err| format!("failed to commit permissions store: {err}"))?;

        Ok(())
    }

    fn from_persisted(
        persisted: PersistedPermissions,
    ) -> HashMap<(String, String, PermissionKind), Decision> {
        let mut decisions = HashMap::new();

        for (user_id, apps) in persisted.users {
            for (app_id, permissions) in apps {
                for (permission, decision) in permissions {
                    if let (Some(permission_kind), Some(parsed_decision)) = (
                        PermissionKind::from_str(&permission),
                        Decision::from_str(&decision),
                    ) {
                        decisions.insert(
                            (user_id.clone(), app_id.clone(), permission_kind),
                            parsed_decision,
                        );
                    }
                }
            }
        }

        decisions
    }

    fn to_persisted(
        decisions: &HashMap<(String, String, PermissionKind), Decision>,
    ) -> PersistedPermissions {
        let mut users: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();

        for ((user_id, app_id, permission), decision) in decisions {
            users.entry(user_id.clone())
                .or_default()
                .entry(app_id.clone())
                .or_default()
                .insert(permission.as_str().to_string(), decision.as_status().to_string());
        }

        PersistedPermissions { users }
    }
}
