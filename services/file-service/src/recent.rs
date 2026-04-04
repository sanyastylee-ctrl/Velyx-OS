use crate::access_policy::{metadata_read_allowed, visible_roots};
use crate::metadata::FileMetadataRecord;
use std::fs;

pub fn list_recent_files() -> Vec<FileMetadataRecord> {
    let mut results = Vec::new();

    for root in visible_roots() {
        let decision = metadata_read_allowed(&root);
        if !decision.allowed {
            continue;
        }
        let entries = match fs::read_dir(&root) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(record) = FileMetadataRecord::from_path(&path) else {
                continue;
            };
            if !record.is_dir {
                results.push(record);
            }
        }
    }

    results.sort_by(|left, right| right.modified_time.cmp(&left.modified_time));
    results.truncate(12);
    results
}
