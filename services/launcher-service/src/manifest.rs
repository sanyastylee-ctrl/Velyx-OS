use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppManifest {
    pub app_id: String,
    pub display_name: String,
    pub executable_path: String,
    pub requested_permissions: Vec<String>,
    pub trust_level: TrustLevel,
    pub category: String,
    pub sandbox_profile: String,
}

impl AppManifest {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("app_id".to_string(), self.app_id.clone());
        map.insert("display_name".to_string(), self.display_name.clone());
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
}

#[derive(Clone, Default)]
pub struct ManifestRegistry {
    manifests: HashMap<String, AppManifest>,
}

impl ManifestRegistry {
    pub fn load() -> Result<Self, String> {
        let dir = Self::discover_dir().ok_or_else(|| {
            "не найден каталог apps-manifests; launcher-service не может запускать приложения"
                .to_string()
        })?;
        let mut manifests = HashMap::new();

        let entries = fs::read_dir(&dir)
            .map_err(|err| format!("не удалось прочитать каталог manifests {}: {err}", dir.display()))?;

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
            manifests.insert(manifest.app_id.clone(), manifest);
        }

        Ok(Self { manifests })
    }

    pub fn get(&self, app_id: &str) -> Option<&AppManifest> {
        self.manifests.get(app_id)
    }

    pub fn list(&self) -> Vec<AppManifest> {
        self.manifests.values().cloned().collect()
    }

    fn discover_dir() -> Option<PathBuf> {
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
}
