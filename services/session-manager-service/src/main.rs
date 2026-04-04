mod api;
mod audit;
mod errors;
mod first_boot;
mod health;
mod handoff;
mod model;
mod shell_launch;
mod startup;
mod state;
mod units;

use api::SessionManagerApi;
use audit::SessionAuditLogger;
use state::SessionStateStore;
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
    let state = SessionStateStore::load(&base_dir);
    let audit = SessionAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.SessionManager")?
        .serve_at(
            "/com/velyx/SessionManager",
            SessionManagerApi::new(state, audit, base_dir.clone()),
        )?
        .build()
        .await?;

    println!(
        "[session-manager-service] running service=com.velyx.SessionManager base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
