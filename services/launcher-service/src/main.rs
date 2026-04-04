mod audit;
mod manifest;
mod portals;
mod seccomp;
mod launcher;
mod sandbox;
mod tracking;

use audit::LauncherAuditLogger;
use launcher::LauncherApi;
use manifest::ManifestRegistry;
use std::path::PathBuf;
use zbus::connection::Builder;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let manifests = match ManifestRegistry::load() {
        Ok(manifests) => manifests,
        Err(err) => {
            eprintln!("[launcher-service] startup_error={err}");
            std::process::exit(1);
        }
    };
    let base_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".velyx");
    let audit = match LauncherAuditLogger::new(base_dir) {
        Ok(audit) => audit,
        Err(err) => {
            eprintln!("[launcher-service] startup_error={err}");
            std::process::exit(1);
        }
    };

    let _connection = Builder::session()?
        .name("com.velyx.Launcher")?
        .serve_at("/com/velyx/Launcher", LauncherApi::new(manifests, audit))?
        .build()
        .await?;

    println!("[launcher-service] running service=com.velyx.Launcher mode=v2");

    std::future::pending::<()>().await;
    Ok(())
}
