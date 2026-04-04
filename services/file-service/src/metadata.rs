use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Clone, Debug)]
pub struct FileMetadataRecord {
    pub id: String,
    pub name: String,
    pub path: String,
    pub extension: String,
    pub modified_time: String,
    pub size: u64,
    pub is_dir: bool,
}

impl FileMetadataRecord {
    pub fn from_path(path: &Path) -> Option<Self> {
        let metadata = fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let modified_secs = modified.duration_since(UNIX_EPOCH).ok()?.as_secs();
        let canonical = path.canonicalize().ok().unwrap_or_else(|| PathBuf::from(path));
        let name = canonical.file_name()?.to_string_lossy().to_string();
        let extension = canonical
            .extension()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default();
        let path_string = canonical.to_string_lossy().to_string();

        Some(Self {
            id: path_string.clone(),
            name,
            path: path_string,
            extension,
            modified_time: modified_secs.to_string(),
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        })
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), self.id.clone());
        map.insert("name".to_string(), self.name.clone());
        map.insert("path".to_string(), self.path.clone());
        map.insert("extension".to_string(), self.extension.clone());
        map.insert("modified_time".to_string(), self.modified_time.clone());
        map.insert("size".to_string(), self.size.to_string());
        map.insert("is_dir".to_string(), self.is_dir.to_string());
        map
    }
}
