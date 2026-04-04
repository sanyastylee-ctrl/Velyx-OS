use crate::access_policy::{
    future_content_read_policy_placeholder, future_file_picker_portal_placeholder,
    metadata_read_allowed,
};
use crate::audit::FileAuditLogger;
use crate::errors::FileServiceError;
use crate::metadata::FileMetadataRecord;
use crate::recent::list_recent_files;
use crate::search::search_files;
use std::collections::HashMap;
use std::path::PathBuf;
use zbus::message::Header;

pub struct FileApi {
    audit: FileAuditLogger,
}

impl FileApi {
    pub fn new(audit: FileAuditLogger) -> Self {
        Self { audit }
    }

    fn requester(header: &Header<'_>) -> String {
        header
            .sender()
            .map(|sender| sender.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    fn metadata_for_target(target: &str) -> Result<FileMetadataRecord, FileServiceError> {
        let path = PathBuf::from(target);
        let decision = metadata_read_allowed(&path);
        if !decision.allowed {
            return Err(FileServiceError::AccessDenied(format!(
                "metadata access denied: {}",
                decision.reason
            )));
        }

        FileMetadataRecord::from_path(&path).ok_or_else(|| {
            FileServiceError::InvalidPath(format!("metadata not available for target {}", target))
        })
    }
}

#[zbus::interface(name = "com.velyx.FileService1")]
impl FileApi {
    async fn search_files(
        &self,
        query: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let requester = Self::requester(&header);
        let results = search_files(query);
        let _ = self
            .audit
            .log(&requester, "search", query, results.len(), "metadata_only");
        Ok(results.into_iter().map(|entry| entry.to_map()).collect())
    }

    async fn list_recent_files(
        &self,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<Vec<HashMap<String, String>>> {
        let requester = Self::requester(&header);
        let results = list_recent_files();
        let _ = self
            .audit
            .log(&requester, "recent", "recent_documents", results.len(), "metadata_only");
        Ok(results.into_iter().map(|entry| entry.to_map()).collect())
    }

    async fn get_metadata(
        &self,
        path_or_id: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let requester = Self::requester(&header);
        let metadata =
            Self::metadata_for_target(path_or_id).map_err(|err| zbus::fdo::Error::Failed(err.message()))?;
        let mut payload = metadata.to_map();
        payload.insert(
            "content_read_policy".to_string(),
            future_content_read_policy_placeholder().to_string(),
        );
        payload.insert(
            "file_picker_portal".to_string(),
            future_file_picker_portal_placeholder().to_string(),
        );
        let _ = self
            .audit
            .log(&requester, "metadata", path_or_id, 1, "metadata_only");
        Ok(payload)
    }
}
