use crate::*;

/// Check if cargo-watch is installed
///
/// # Returns
///
/// - `bool`: True if cargo-watch is available
async fn is_cargo_watch_installed() -> bool {
    Command::new("cargo-watch")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok_and(|status: ExitStatus| status.success())
}

/// Install cargo-watch using cargo install
///
/// # Returns
///
/// - `Result<(), std::io::Error>`: Success or error
async fn install_cargo_watch() -> Result<(), std::io::Error> {
    println!("cargo-watch not found, installing...");
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("install").arg("cargo-watch");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status: ExitStatus = cmd.status().await?;
    if !status.success() {
        return Err(std::io::Error::other("failed to install cargo-watch"));
    }
    Ok(())
}

/// Execute watch command using cargo-watch
///
/// # Returns
///
/// - `Result<(), std::io::Error>`: Success or error
pub(crate) async fn execute_watch() -> Result<(), std::io::Error> {
    if !is_cargo_watch_installed().await {
        install_cargo_watch().await?;
    }
    let mut cmd: Command = Command::new("cargo-watch");
    cmd.arg("--clear")
        .arg("--skip-local-deps")
        .arg("-q")
        .arg("-x")
        .arg("run");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status: ExitStatus = cmd.status().await?;
    if !status.success() {
        return Err(std::io::Error::other("cargo-watch failed"));
    }
    Ok(())
}
