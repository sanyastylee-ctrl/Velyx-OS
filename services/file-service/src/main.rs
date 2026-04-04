mod access_policy;
mod api;
mod audit;
mod errors;
mod metadata;
mod recent;
mod search;

use api::FileApi;
use audit::FileAuditLogger;
use std::env;
use std::path::PathBuf;
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
    let audit = FileAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.FileService")?
        .serve_at("/com/velyx/FileService", FileApi::new(audit))?
        .build()
        .await?;

    println!(
        "[file-service] running service=com.velyx.FileService base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
