use crate::errors::SettingsError;
use crate::schema::SETTINGS_SCHEMA;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, rename, write};
use std::path::{Path, PathBuf};

#[derive(Default, Serialize, Deserialize)]
struct PersistedSettings {
    values: HashMap<String, String>,
}

pub struct SettingsStore {
    path: PathBuf,
    values: HashMap<String, String>,
    available: bool,
}

impl SettingsStore {
    pub fn load(base_dir: &Path) -> Self {
        let path = base_dir.join("settings.json");
        if let Err(err) = create_dir_all(base_dir) {
            eprintln!("[settings-service] store_error=create_dir_failed err={err}");
            return Self {
                path,
                values: default_values(),
                available: false,
            };
        }

        let mut values = default_values();
        let available = match read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<PersistedSettings>(&content) {
                Ok(parsed) => {
                    for (key, value) in parsed.values {
                        if values.contains_key(&key) {
                            values.insert(key, value);
                        }
                    }
                    true
                }
                Err(err) => {
                    eprintln!("[settings-service] store_error=invalid_json path={} err={err}", path.display());
                    true
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
            Err(err) => {
                eprintln!("[settings-service] store_error=read_failed path={} err={err}", path.display());
                false
            }
        };

        Self { path, values, available }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn get_value(&self, key: &str) -> Result<String, SettingsError> {
        self.values
            .get(key)
            .cloned()
            .ok_or_else(|| SettingsError::UnknownKey(key.to_string()))
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(String, String), SettingsError> {
        if !self.available {
            return Err(SettingsError::StoreUnavailable(
                "settings store is unavailable".to_string(),
            ));
        }

        let old_value = self.get_value(key)?;
        self.values.insert(key.to_string(), value.to_string());
        self.persist()
            .map_err(SettingsError::StoreUnavailable)?;
        Ok((old_value, value.to_string()))
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    fn persist(&self) -> Result<(), String> {
        let encoded = serde_json::to_string_pretty(&PersistedSettings {
            values: self.values.clone(),
        })
        .map_err(|err| format!("failed to encode settings store: {err}"))?;

        let temp_path = self.path.with_extension("json.tmp");
        write(&temp_path, encoded.as_bytes())
            .map_err(|err| format!("failed to write temp settings store: {err}"))?;
        rename(&temp_path, &self.path)
            .map_err(|err| format!("failed to commit settings store: {err}"))?;
        Ok(())
    }
}

fn default_values() -> HashMap<String, String> {
    let mut values = HashMap::new();
    for item in SETTINGS_SCHEMA {
        values.insert(item.key.to_string(), item.default_value.to_string());
    }
    values
}
