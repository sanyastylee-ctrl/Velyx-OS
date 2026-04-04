mod api;
mod audit;
mod errors;
mod model;
mod policy;
mod schema;
mod store;

use api::SettingsApi;
use audit::SettingsAuditLogger;
use std::env;
use std::path::PathBuf;
use store::SettingsStore;
use zbus::connection::Builder;

fn velyx_home() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".velyx")
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let user_id = env::var("VELYX_USER_ID").unwrap_or_else(|_| "user".to_string());
    let base_dir = velyx_home();
    let store = SettingsStore::load(&base_dir);
    let audit = SettingsAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.Settings")?
        .serve_at("/com/velyx/Settings", SettingsApi::new(store, audit, user_id.clone()))?
        .build()
        .await?;

    println!(
        "[settings-service] running service=com.velyx.Settings user_id={} base_dir={}",
        user_id,
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
