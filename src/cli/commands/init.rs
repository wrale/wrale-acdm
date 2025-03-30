// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::{anyhow, Result};
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

        adapter.init(self.location.clone(), force)?;

        info!("Configuration initialized successfully");
        Ok(())
    }
}
