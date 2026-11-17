// SPDX-License-Identifier: GPL-3.0-only

use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hika", version, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Acquire {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    Reacquire {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    Transcend,
}

pub fn preprocess_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();

    if let Some(command) = args.get_mut(1) {
        match command.as_str() {
            "-Acquire" => *command = "acquire".into(),
            "-Reacquire" => *command = "reacquire".into(),
            "-Transcend" => *command = "transcend".into(),
            _ => {}
        }
    }

    args
}

pub fn run() {
    let args = preprocess_args();

    if args.len() == 1 || args.get(1).map(String::as_str) == Some("--help") {
        crate::output::help();
        return;
    }

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Acquire { packages } => commands::acquire(packages),
        Commands::Reacquire { packages } => commands::reacquire(packages),
        Commands::Transcend => commands::transcend(),
    }
}
