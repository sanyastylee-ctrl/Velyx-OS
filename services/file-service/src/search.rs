use crate::access_policy::{metadata_read_allowed, visible_roots};
use crate::metadata::FileMetadataRecord;
use std::fs;
use std::path::Path;

pub fn search_files(query: &str) -> Vec<FileMetadataRecord> {
    let lower = query.trim().to_lowercase();
    let extension_filter = extract_extension_filter(&lower);
    let mut results = Vec::new();

    for root in visible_roots() {
        visit_dir(&root, &lower, extension_filter.as_deref(), 0, &mut results);
    }

    results.sort_by(|left, right| right.modified_time.cmp(&left.modified_time));
    results.truncate(25);
    results
}

fn visit_dir(
    dir: &Path,
    query: &str,
    extension_filter: Option<&str>,
    depth: usize,
    results: &mut Vec<FileMetadataRecord>,
) {
    if depth > 3 {
        return;
    }
    let decision = metadata_read_allowed(dir);
    if !decision.allowed {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(record) = FileMetadataRecord::from_path(&path) else {
            continue;
        };

        let name_match = query.is_empty()
            || record.name.to_lowercase().contains(query)
            || record.path.to_lowercase().contains(query);
        let extension_match = extension_filter
            .map(|expected| record.extension.eq_ignore_ascii_case(expected))
            .unwrap_or(true);
        if name_match && extension_match {
            results.push(record.clone());
        }

        if record.is_dir {
            visit_dir(&path, query, extension_filter, depth + 1, results);
        }
    }
}

fn extract_extension_filter(query: &str) -> Option<String> {
    let trimmed = query.trim();
    if let Some(rest) = trimmed.strip_prefix("*.") {
        return Some(rest.to_string());
    }
    if let Some(rest) = trimmed.strip_prefix('.') {
        return Some(rest.to_string());
    }
    None
}
