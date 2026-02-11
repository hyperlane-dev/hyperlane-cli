use crate::*;

/// Check if cargo-clippy is installed
///
/// # Returns
///
/// - `bool`: True if cargo-clippy is available
async fn is_cargo_clippy_installed() -> bool {
    Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok_and(|status: ExitStatus| status.success())
}

/// Install cargo-clippy using rustup
///
/// # Returns
///
/// - `Result<(), std::io::Error>`: Success or error
async fn install_cargo_clippy() -> Result<(), std::io::Error> {
    println!("cargo-clippy not found, installing...");
    let mut cmd: Command = Command::new("rustup");
    cmd.arg("component").arg("add").arg("clippy");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status: ExitStatus = cmd.status().await?;
    if !status.success() {
        return Err(std::io::Error::other("failed to install cargo-clippy"));
    }
    Ok(())
}

/// Execute clippy fix command
///
/// # Arguments
/// - `args`: The parsed arguments
///
/// # Returns
/// - `Result<(), std::io::Error>`: Success or error
async fn execute_clippy_fix(args: &Args) -> Result<(), std::io::Error> {
    if !is_cargo_clippy_installed().await {
        install_cargo_clippy().await?;
    }
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("clippy")
        .arg("--fix")
        .arg("--workspace")
        .arg("--all-targets")
        .arg("--allow-dirty");
    if let Some(ref manifest_path) = args.manifest_path {
        cmd.arg("--manifest-path").arg(manifest_path);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status: ExitStatus = cmd.status().await?;
    if !status.success() {
        return Err(std::io::Error::other("cargo clippy --fix failed"));
    }
    Ok(())
}

/// Execute fmt command
///
/// # Arguments
/// - `args`: The parsed arguments
///
/// # Returns
///
/// - `Result<(), std::io::Error>`: Success or error
pub(crate) async fn execute_fmt(args: &Args) -> Result<(), std::io::Error> {
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("fmt");
    if args.check {
        cmd.arg("--check");
    }
    if let Some(ref manifest_path) = args.manifest_path {
        cmd.arg("--manifest-path").arg(manifest_path);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status: ExitStatus = cmd.status().await?;
    if !status.success() {
        return Err(std::io::Error::other("cargo fmt failed"));
    }
    if !args.check {
        execute_clippy_fix(args).await?;
    }
    Ok(())
}
