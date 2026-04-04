mod api;
mod audit;
mod context;
mod errors;
mod explain;
mod intent;
mod model;
mod parser;
mod pending;
mod policy_guard;
mod session;
mod tool_executor;
mod tool_registry;

use api::AiServiceApi;
use audit::AiAuditLogger;
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
    let audit = AiAuditLogger::new(&base_dir).map_err(zbus::Error::Failure)?;

    let _connection = Builder::session()?
        .name("com.velyx.AI")?
        .serve_at("/com/velyx/AI", AiServiceApi::new(audit))?
        .build()
        .await?;

    println!(
        "[ai-service] running service=com.velyx.AI base_dir={}",
        base_dir.display()
    );

    std::future::pending::<()>().await;
    Ok(())
}
