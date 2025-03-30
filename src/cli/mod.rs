// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

mod commands;

use std::path::PathBuf;
use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::commands::{
    init::InitCommand,
    add::AddCommand,
    include::IncludeCommand,
    update::UpdateCommand,
};

/// Wrale Agnostic Content Dependency Manager
#[derive(Parser)]
#[clap(name = "acdm", version)]
struct Cli {
    /// Path to the configuration file
    #[clap(short, long, default_value = "acdm.toml", global = true)]
    config: PathBuf,
    
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
}

/// Run the CLI application
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Init(cmd) => cmd.execute(&cli.config),
        Commands::Add(cmd) => cmd.execute(&cli.config),
        Commands::Include(cmd) => cmd.execute(&cli.config),
        Commands::Update(cmd) => cmd.execute(&cli.config),
    }
}
