mod api;
mod audit;
mod errors;
mod model;
mod orchestration;
mod rollback;
mod store;

use api::RecoveryApi;
use audit::RecoveryAuditLogger;
use std::env;
use std::path::PathBuf;
use store::RecoveryStore;
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
    let store = RecoveryStore::load(&base_dir);
    let audit = RecoveryAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.Recovery")?
        .serve_at(
            "/com/velyx/Recovery",
            RecoveryApi::new(store, audit, base_dir.clone()),
        )?
        .build()
        .await?;

    println!(
        "[recovery-service] running service=com.velyx.Recovery base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
