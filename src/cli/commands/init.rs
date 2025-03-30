// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use anyhow::Result;
use clap::Args;

use crate::interfaces::cli::CliAdapter;

/// Initialize a new configuration
#[derive(Args)]
pub struct InitCommand {
    /// Default location for vendored content
    #[clap(long)]
    location: Option<String>,
}

impl InitCommand {
    pub fn execute(&self, config_path: &Path) -> Result<()> {
        println!("Initializing new configuration at {}", config_path.display());
        
        let adapter = CliAdapter::new(config_path.to_path_buf());
        adapter.init(self.location.clone())?;
        
        println!("Configuration initialized successfully");
        Ok(())
    }
}
