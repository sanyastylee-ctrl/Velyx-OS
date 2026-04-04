mod api;
mod apply;
mod audit;
mod errors;
mod model;
mod orchestration;
mod reconciliation;
mod snapshot;
mod store;
mod verification;

use api::UpdateApi;
use audit::UpdateAuditLogger;
use std::env;
use std::path::PathBuf;
use store::UpdateStore;
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
    let store = UpdateStore::load(&base_dir);
    let audit = UpdateAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.UpdateEngine")?
        .serve_at(
            "/com/velyx/UpdateEngine",
            UpdateApi::new(store, audit, base_dir.clone()),
        )?
        .build()
        .await?;

    println!(
        "[update-engine] running service=com.velyx.UpdateEngine base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
