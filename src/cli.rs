// SPDX-License-Identifier: GPL-3.0-only

use crate::commands;
use clap::{CommandFactory, Parser, Subcommand, builder::styling::Style};

#[derive(Parser)]
#[command(name = "hika", version, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download and install packages
    Acquire { packages: Vec<String> },

    /// Reinstall packages
    Reacquire { packages: Vec<String> },

    /// Update installed packages
    Transcend,

    /// Reconcile system configuration with garden
    Reconcile,
}

pub fn preprocess_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();

    if let Some(command) = args.get_mut(1) {
        // Translate hika's command names for Clap.
        match command.as_str() {
            "-Acquire" => *command = "acquire".into(),
            "-Reacquire" => *command = "reacquire".into(),
            "-Transcend" => *command = "transcend".into(),
            "-Reconcile" => *command = "reconcile".into(),
            _ => {}
        }
    }

    args
}

pub fn run() {
    let args = preprocess_args();

    if args.len() == 1 || matches!(args.get(1).map(String::as_str), Some("--help" | "-h")) {
        print_help();
        return;
    }

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Acquire { packages } => commands::acquire(packages),
        Commands::Reacquire { packages } => commands::reacquire(packages),
        Commands::Transcend => commands::transcend(),
        Commands::Reconcile => commands::reconcile(),
    }
}

fn print_help() {
    let cmd = Cli::command();

    let bold = Style::new().bold();
    let underline = Style::new().underline();

    println!("{bold}hika {}{bold:#}", env!("CARGO_PKG_VERSION"));
    println!();

    println!("{underline}Usage:{underline:#} hika <COMMAND> [ARGS]");
    println!();

    println!("{underline}Commands:{underline:#}");

    for command in cmd.get_subcommands() {
        let name = command.get_name();
        let description = command
            .get_about()
            .map(|about| about.to_string())
            .unwrap_or_default();

        let displayed_name = match name {
            "acquire" => "-Acquire",
            "reacquire" => "-Reacquire",
            "transcend" => "-Transcend",
            "reconcile" => "-Reconcile",
            _ => name,
        };

        println!("  {bold}{displayed_name:<16}{bold:#} {description}");
    }

    println!();

    println!("{underline}Options:{underline:#}");
    println!("  {bold}{:<16}{bold:#} {}", "-h, --help", "Print help");
    println!(
        "  {bold}{:<16}{bold:#} {}",
        "-V, --version", "Print version"
    );
}
