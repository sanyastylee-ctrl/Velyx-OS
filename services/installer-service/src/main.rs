mod api;
mod audit;
mod errors;
mod handoff;
mod model;
mod store;

use api::InstallerApi;
use audit::InstallerAuditLogger;
use handoff::InstallerHandoffStore;
use std::env;
use std::path::PathBuf;
use store::InstallerStore;
use zbus::connection::Builder;

fn velyx_home() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".velyx")
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let base_dir = velyx_home();
    let store = InstallerStore::load(&base_dir);
    let handoff = InstallerHandoffStore::load(&base_dir);
    let audit = InstallerAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.Installer")?
        .serve_at("/com/velyx/Installer", InstallerApi::new(store, handoff, audit))?
        .build()
        .await?;

    println!(
        "[installer-service] running service=com.velyx.Installer base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
