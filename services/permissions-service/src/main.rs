mod api;
mod identity;
mod logger;
mod model;
mod policy;
mod store;

use api::PermissionsApi;
use logger::AuditLogger;
use std::env;
use std::path::PathBuf;
use store::PermissionStore;
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
    let store = PermissionStore::load(&base_dir);
    let logger = AuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.Permissions")?
        .serve_at(
            "/com/velyx/Permissions",
            PermissionsApi::new(store, logger, user_id.clone()),
        )?
        .build()
        .await?;

    println!(
        "[permissions-service] running service=com.velyx.Permissions user_id={} base_dir={}",
        user_id,
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
