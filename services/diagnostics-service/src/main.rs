mod api;
mod audit;
mod collectors;
mod errors;
mod model;
mod summary;

use api::DiagnosticsApi;
use audit::DiagnosticsAuditLogger;
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
    let user_id = env::var("VELYX_USER_ID").unwrap_or_else(|_| "user".to_string());
    let audit = DiagnosticsAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.Diagnostics")?
        .serve_at("/com/velyx/Diagnostics", DiagnosticsApi::new(audit, user_id.clone()))?
        .build()
        .await?;

    println!(
        "[diagnostics-service] running service=com.velyx.Diagnostics user_id={} base_dir={}",
        user_id,
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
