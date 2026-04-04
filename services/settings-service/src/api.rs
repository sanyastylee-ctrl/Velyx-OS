use crate::audit::SettingsAuditLogger;
use crate::policy::validate_key_and_value;
use crate::schema::metadata_for_key;
use crate::store::SettingsStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::message::Header;

pub struct SettingsApi {
    store: Arc<Mutex<SettingsStore>>,
    audit: SettingsAuditLogger,
    user_id: String,
}

impl SettingsApi {
    pub fn new(store: SettingsStore, audit: SettingsAuditLogger, user_id: String) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            audit,
            user_id,
        }
    }

    fn sender_source(header: &Header<'_>) -> String {
        header
            .sender()
            .map(|sender| sender.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }
}

#[zbus::interface(name = "com.velyx.Settings1")]
impl SettingsApi {
    async fn get_value(&self, key: &str, #[zbus(header)] _header: Header<'_>) -> zbus::fdo::Result<String> {
        let store = self.store.lock().await;
        store.get_value(key).map_err(|err| zbus::fdo::Error::Failed(err.message()))
    }

    async fn set_value(
        &self,
        key: &str,
        value: &str,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<bool> {
        validate_key_and_value(key, value).map_err(|err| zbus::fdo::Error::Failed(err.message()))?;

        let mut store = self.store.lock().await;
        let (old_value, new_value) = store
            .set_value(key, value)
            .map_err(|err| zbus::fdo::Error::Failed(err.message()))?;
        let source = Self::sender_source(&header);
        self.audit
            .log(&self.user_id, key, &old_value, &new_value, &source, "success")
            .map_err(zbus::fdo::Error::Failed)?;
        Ok(true)
    }

    async fn list_keys(&self, #[zbus(header)] _header: Header<'_>) -> zbus::fdo::Result<Vec<String>> {
        let store = self.store.lock().await;
        Ok(store.list_keys())
    }

    async fn get_metadata(
        &self,
        key: &str,
        #[zbus(header)] _header: Header<'_>,
    ) -> zbus::fdo::Result<HashMap<String, String>> {
        let metadata = metadata_for_key(key)
            .ok_or_else(|| zbus::fdo::Error::Failed(format!("unknown key: {key}")))?;
        let mut result = HashMap::new();
        result.insert("display_name".to_string(), metadata.display_name.to_string());
        result.insert("description".to_string(), metadata.description.to_string());
        result.insert("type".to_string(), metadata.value_type.to_string());
        result.insert("allowed_values".to_string(), metadata.allowed_values.join(","));
        result.insert("risk_level".to_string(), metadata.risk_level.as_str().to_string());
        result.insert(
            "requires_confirmation".to_string(),
            if metadata.requires_confirmation { "true" } else { "false" }.to_string(),
        );
        Ok(result)
    }
}
