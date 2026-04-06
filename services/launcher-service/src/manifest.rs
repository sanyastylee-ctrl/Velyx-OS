use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_category() -> String {
    "utility".to_string()
}

fn default_permissions() -> Vec<String> {
    Vec::new()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    System,
    Trusted,
    Unknown,
}

impl TrustLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Trusted => "trusted",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallSource {
    System,
    User,
}

impl InstallSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AppStatus {
    Installed,
    Removed,
    Broken,
}

impl AppStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Removed => "removed",
            Self::Broken => "broken",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppManifest {
    pub app_id: String,
    pub display_name: String,
    #[serde(default = "default_version")]
    pub version: String,
    pub executable_path: String,
    #[serde(default = "default_permissions")]
    pub requested_permissions: Vec<String>,
    pub trust_level: TrustLevel,
    #[serde(default = "default_category")]
    pub category: String,
    pub sandbox_profile: String,
}

impl AppManifest {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("app_id".to_string(), self.app_id.clone());
        map.insert("display_name".to_string(), self.display_name.clone());
        map.insert("version".to_string(), self.version.clone());
        map.insert("executable_path".to_string(), self.executable_path.clone());
        map.insert(
            "requested_permissions".to_string(),
            self.requested_permissions.join(","),
        );
        map.insert("trust_level".to_string(), self.trust_level.as_str().to_string());
        map.insert("category".to_string(), self.category.clone());
        map.insert("sandbox_profile".to_string(), self.sandbox_profile.clone());
        map
    }

    pub fn validate_for_launch(&self) -> Result<(), String> {
        if self.app_id.trim().is_empty() {
            return Err("app_id_missing".to_string());
        }
        if self.display_name.trim().is_empty() {
            return Err("display_name_missing".to_string());
        }
        if self.version.trim().is_empty() {
            return Err("version_missing".to_string());
        }
        if self.executable_path.trim().is_empty() {
            return Err("executable_path_missing".to_string());
        }
        if self.executable_path.contains("__TODO__") || self.executable_path.contains("<placeholder>") {
            return Err("executable_path_placeholder".to_string());
        }
        if self.sandbox_profile.trim().is_empty() {
            return Err("sandbox_profile_missing".to_string());
        }
        if self.requested_permissions.iter().any(|permission| permission.trim().is_empty()) {
            return Err("permission_entry_invalid".to_string());
        }
        Ok(())
    }

    pub fn validate_executable(&self) -> Result<(), String> {
        let path = PathBuf::from(&self.executable_path);
        if !path.is_absolute() {
            return Err("executable_path_not_absolute".to_string());
        }
        if !path.exists() {
            return Err("executable_path_missing".to_string());
        }
        if !path.is_file() {
            return Err("executable_path_not_file".to_string());
        }
        #[cfg(unix)]
        {
            let permissions = fs::metadata(&path)
                .map_err(|_| "executable_metadata_unreadable".to_string())?
                .permissions();
            if permissions.mode() & 0o111 == 0 {
                return Err("executable_not_executable".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppRegistryEntry {
    pub app_id: String,
    pub display_name: String,
    pub version: String,
    pub executable_path: String,
    #[serde(default = "default_permissions")]
    pub requested_permissions: Vec<String>,
    pub trust_level: TrustLevel,
    #[serde(default = "default_category")]
    pub category: String,
    pub sandbox_profile: String,
    pub install_source: InstallSource,
    pub install_time: String,
    pub status: AppStatus,
    #[serde(default)]
    pub manifest_path: String,
    #[serde(default)]
    pub payload_root: String,
}

impl AppRegistryEntry {
    pub fn to_manifest(&self) -> AppManifest {
        AppManifest {
            app_id: self.app_id.clone(),
            display_name: self.display_name.clone(),
            version: self.version.clone(),
            executable_path: self.executable_path.clone(),
            requested_permissions: self.requested_permissions.clone(),
            trust_level: self.trust_level.clone(),
            category: self.category.clone(),
            sandbox_profile: self.sandbox_profile.clone(),
        }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = self.to_manifest().to_map();
        map.insert(
            "install_source".to_string(),
            self.install_source.as_str().to_string(),
        );
        map.insert("install_time".to_string(), self.install_time.clone());
        map.insert("status".to_string(), self.status.as_str().to_string());
        map.insert("manifest_path".to_string(), self.manifest_path.clone());
        map.insert("payload_root".to_string(), self.payload_root.clone());
        map
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppRegistryFile {
    #[serde(default)]
    pub apps: Vec<AppRegistryEntry>,
}

#[derive(Clone)]
pub struct ManifestRegistry {
    registry_path: PathBuf,
    system_manifests_dir: PathBuf,
}

impl ManifestRegistry {
    pub fn load() -> Result<Self, String> {
        let state_dir = Self::state_dir();
        fs::create_dir_all(&state_dir)
            .map_err(|err| format!("не удалось создать state dir {}: {err}", state_dir.display()))?;
        let user_apps_dir = state_dir.join("apps");
        fs::create_dir_all(&user_apps_dir)
            .map_err(|err| format!("не удалось создать apps dir {}: {err}", user_apps_dir.display()))?;

        let system_manifests_dir = Self::discover_system_manifest_dir().ok_or_else(|| {
            "не найден каталог apps-manifests; launcher-service не может подготовить registry"
                .to_string()
        })?;
        let registry = Self {
            registry_path: state_dir.join("apps_registry.json"),
            system_manifests_dir,
        };
        registry.ensure_seeded()?;
        Ok(registry)
    }

    pub fn get(&self, app_id: &str) -> Result<Option<AppRegistryEntry>, String> {
        let registry = self.read_registry_file()?;
        Ok(registry
            .apps
            .into_iter()
            .find(|entry| entry.app_id == app_id && entry.status != AppStatus::Removed))
    }

    pub fn list(&self) -> Result<Vec<AppRegistryEntry>, String> {
        let mut apps = self
            .read_registry_file()?
            .apps
            .into_iter()
            .filter(|entry| entry.status != AppStatus::Removed)
            .collect::<Vec<_>>();
        apps.sort_by(|left, right| left.display_name.cmp(&right.display_name));
        Ok(apps)
    }

    fn ensure_seeded(&self) -> Result<(), String> {
        if self.registry_path.exists() {
            let existing = self.read_registry_file()?;
            if !existing.apps.is_empty() {
                return Ok(());
            }
        }
        let apps = self.scan_system_apps()?;
        self.write_registry_file(&AppRegistryFile { apps })
    }

    fn read_registry_file(&self) -> Result<AppRegistryFile, String> {
        if !self.registry_path.exists() {
            return Ok(AppRegistryFile::default());
        }
        let raw = fs::read_to_string(&self.registry_path)
            .map_err(|err| format!("не удалось прочитать registry {}: {err}", self.registry_path.display()))?;
        serde_json::from_str::<AppRegistryFile>(&raw)
            .map_err(|err| format!("невалидный registry {}: {err}", self.registry_path.display()))
    }

    fn write_registry_file(&self, registry: &AppRegistryFile) -> Result<(), String> {
        let tmp = self.registry_path.with_extension("json.tmp");
        let raw = serde_json::to_string_pretty(registry)
            .map_err(|err| format!("не удалось сериализовать app registry: {err}"))?;
        fs::write(&tmp, raw)
            .map_err(|err| format!("не удалось записать временный app registry {}: {err}", tmp.display()))?;
        fs::rename(&tmp, &self.registry_path)
            .map_err(|err| format!("не удалось заменить app registry {}: {err}", self.registry_path.display()))
    }

    fn scan_system_apps(&self) -> Result<Vec<AppRegistryEntry>, String> {
        let mut apps = Vec::new();
        let entries = fs::read_dir(&self.system_manifests_dir).map_err(|err| {
            format!(
                "не удалось прочитать system manifests {}: {err}",
                self.system_manifests_dir.display()
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|err| format!("ошибка чтения manifest entry: {err}"))?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }

            let raw = fs::read_to_string(&path)
                .map_err(|err| format!("не удалось прочитать manifest {}: {err}", path.display()))?;
            let manifest: AppManifest = serde_json::from_str(&raw)
                .map_err(|err| format!("невалидный manifest {}: {err}", path.display()))?;

            let status = if manifest.validate_for_launch().is_ok() && manifest.validate_executable().is_ok() {
                AppStatus::Installed
            } else {
                AppStatus::Broken
            };

            apps.push(AppRegistryEntry {
                app_id: manifest.app_id.clone(),
                display_name: manifest.display_name.clone(),
                version: manifest.version.clone(),
                executable_path: manifest.executable_path.clone(),
                requested_permissions: manifest.requested_permissions.clone(),
                trust_level: manifest.trust_level.clone(),
                category: manifest.category.clone(),
                sandbox_profile: manifest.sandbox_profile.clone(),
                install_source: InstallSource::System,
                install_time: Utc::now().to_rfc3339(),
                status,
                manifest_path: path.to_string_lossy().to_string(),
                payload_root: self.system_manifests_dir.to_string_lossy().to_string(),
            });
        }

        Ok(apps)
    }

    fn discover_system_manifest_dir() -> Option<PathBuf> {
        if let Ok(value) = std::env::var("VELYX_MANIFESTS_DIR") {
            let path = PathBuf::from(value);
            if path.exists() {
                return Some(path);
            }
        }

        let current = std::env::current_dir().ok()?;
        for base in current.ancestors() {
            let candidate = base.join("app-manifests");
            if candidate.exists() {
                return Some(candidate);
            }
        }

        let fallback = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("app-manifests");
        if fallback.exists() {
            return Some(fallback);
        }

        None
    }

    fn state_dir() -> PathBuf {
        if let Ok(value) = std::env::var("VELYX_STATE_DIR") {
            return PathBuf::from(value);
        }
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".velyx")
    }
}
