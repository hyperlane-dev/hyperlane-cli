//! Hyperlane CLI
//!
//! A command-line tool for Hyperlane framework.

mod command;
mod config;

use {command::*, config::*};

use std::{
    env::args,
    process::{ExitStatus, Stdio, exit},
};

use tokio::process::Command;

/// Print help message
fn print_help() {
    println!("hyperlane-cli [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  fmt       Format Rust code using cargo fmt");
    println!("  watch     Watch files and run cargo run using cargo-watch");
    println!("  -h, --help      Print this help message");
    println!("  -v, --version   Print version information");
    println!();
    println!("Fmt Options:");
    println!("  --check         Check formatting without making changes");
    println!("  --manifest-path <PATH>  Path to Cargo.toml");
}

/// Print version
fn print_version() {
    println!("hyperlane-cli {}", env!("CARGO_PKG_VERSION"));
}

/// Execute fmt command
///
/// # Arguments
/// - `args`: The parsed arguments
///
/// # Returns
/// - `Result<(), std::io::Error>`: Success or error
async fn execute_fmt(args: &Args) -> Result<(), std::io::Error> {
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
    Ok(())
}

/// Check if cargo-watch is installed
///
/// # Returns
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
/// - `Result<(), std::io::Error>`: Success or error
async fn execute_watch() -> Result<(), std::io::Error> {
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

#[tokio::main]
async fn main() {
    let args: Args = parse_args();
    match args.command {
        CommandType::Fmt => {
            if let Err(error) = execute_fmt(&args).await {
                eprintln!("fmt failed: {error}");
                exit(1);
            }
        }
        CommandType::Watch => {
            if let Err(error) = execute_watch().await {
                eprintln!("watch failed: {error}");
                exit(1);
            }
        }
        CommandType::Help => print_help(),
        CommandType::Version => print_version(),
    }
}
