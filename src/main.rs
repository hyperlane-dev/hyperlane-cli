//! Hyperlane CLI
//!
//! A command-line tool for Hyperlane framework.

mod command;
mod config;
mod fmt;
mod help;
mod version;
mod watch;

pub(crate) use {command::*, config::*, fmt::*, help::*, version::*, watch::*};

pub(crate) use std::{
    env::args,
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
        CommandType::Help => print_help(),
        CommandType::Version => print_version(),
    }
}
