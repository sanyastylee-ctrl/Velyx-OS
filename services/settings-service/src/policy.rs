use crate::errors::SettingsError;
use crate::schema::metadata_for_key;

pub fn validate_key_and_value(key: &str, value: &str) -> Result<(), SettingsError> {
    let metadata = metadata_for_key(key).ok_or_else(|| SettingsError::UnknownKey(key.to_string()))?;
    if metadata.allowed_values.contains(&value) {
        Ok(())
    } else {
        Err(SettingsError::InvalidValue(format!(
            "key={} value={} allowed={}",
            key,
            value,
            metadata.allowed_values.join(",")
        )))
    }
}
