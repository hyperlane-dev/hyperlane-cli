//! Hyperlane CLI
//!
//! A command-line tool for Hyperlane framework.

mod bump;
mod command;
mod config;
mod fmt;
mod help;
mod new;
mod publish;
mod version;
mod watch;

pub(crate) use {
    bump::*, command::*, config::*, fmt::*, help::*, new::*, publish::*, version::*, watch::*,
};

pub(crate) use std::{
    collections::{HashMap, VecDeque},
    env::args,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio, exit},
};

pub(crate) use tokio::process::Command;

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
        CommandType::Bump => {
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let bump_type: BumpVersionType = args.bump_type.unwrap_or(BumpVersionType::Patch);
            match execute_bump(&manifest_path, bump_type) {
                Ok(new_version) => {
                    println!("Version bumped to {new_version}");
                }
                Err(error) => {
                    eprintln!("bump failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::Publish => {
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let max_retries: u32 = args.max_retries;
            match execute_publish(&manifest_path, max_retries).await {
                Ok(results) => {
                    let failed_count: usize = results
                        .iter()
                        .filter(|r: &&PublishResult| !r.success)
                        .count();
                    if failed_count > 0 {
                        eprintln!("Publish completed with {failed_count} failures");
                        exit(1);
                    } else {
                        println!("All packages published successfully");
                    }
                }
                Err(error) => {
                    eprintln!("publish failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::New => {
            if let Some(project_name) = args.project_name {
                if let Err(error) = execute_new(&project_name).await {
                    eprintln!("new failed: {error}");
                    exit(1);
                }
            } else {
                eprintln!(
                    "Error: Project name is required. Usage: hyperlane-cli new <PROJECT_NAME>"
                );
                exit(1);
            }
        }
        CommandType::Help => print_help(),
        CommandType::Version => print_version(),
    }
}
