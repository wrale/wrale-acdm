// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::{Result, anyhow};
use clap::Args;
use log::{debug, info, warn};
use std::path::Path;

use crate::interfaces::cli::CliAdapter;

/// Initialize a new configuration
#[derive(Args)]
pub struct InitCommand {
    /// Default location for vendored content
    #[clap(long)]
    location: Option<String>,
}

impl InitCommand {
    pub fn execute(&self, config_path: &Path, force: bool) -> Result<()> {
        info!(
            "Initializing new configuration at {}",
            config_path.display()
        );
        debug!("Force mode: {}", force);

        let adapter = CliAdapter::new(config_path.to_path_buf());
        
        // Check if file exists and handle force flag
        if config_path.exists() && !force {
            warn!("Configuration file already exists: {}", config_path.display());
            return Err(anyhow::anyhow!("Configuration file already exists. Use --force to overwrite."));
        }

        adapter.init(self.location.clone())?;

        info!("Configuration initialized successfully");
        Ok(())
    }
}
