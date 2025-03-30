// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};
use log::debug;
use std::path::PathBuf;

use crate::cli::commands::{
    add::AddCommand, include::IncludeCommand, init::InitCommand, status::StatusCommand,
    update::UpdateCommand,
};

/// Wrale Agnostic Content Dependency Manager
#[derive(Parser)]
#[clap(name = "acdm", version)]
struct Cli {
    /// Path to the configuration file
    #[clap(short, long, default_value = "acdm.toml", global = true)]
    config: PathBuf,

    /// Suppress verbose output
    #[clap(short, long, global = true)]
    quiet: bool,

    /// Force operations without prompting
    #[clap(short, long, global = true)]
    force: bool,

    /// Subcommand to run
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration
    Init(InitCommand),

    /// Add a new dependency
    Add(AddCommand),

    /// Include paths in a dependency
    Include(IncludeCommand),

    /// Update dependencies
    Update(UpdateCommand),

    /// Show status of dependencies
    Status(StatusCommand),
}

// Function moved to CliAdapter implementation

/// Run the CLI application
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging with proper level based on quiet flag
    let env = Env::default().filter_or("RUST_LOG", if cli.quiet { "warn" } else { "debug" });

    Builder::from_env(env)
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .format_module_path(true)
        .init();

    debug!("Starting acdm with config path: {}", cli.config.display());

    match &cli.command {
        Commands::Init(cmd) => cmd.execute(&cli.config, cli.force),
        Commands::Add(cmd) => cmd.execute(&cli.config, cli.force),
        Commands::Include(cmd) => cmd.execute(&cli.config, cli.force),
        Commands::Update(cmd) => cmd.execute(&cli.config, cli.force),
        Commands::Status(cmd) => cmd.execute(&cli.config, cli.force),
    }
}
