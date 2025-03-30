// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::Result;
use clap::Args;
use log::{debug, info};
use std::path::Path;

use crate::interfaces::cli::CliAdapter;

/// Update dependencies
#[derive(Args)]
pub struct UpdateCommand {
    /// Dependencies to update (all if not specified)
    dependencies: Vec<String>,

    /// Commit message for the update
    #[clap(short, long)]
    message: Option<String>,
}

impl UpdateCommand {
    pub fn execute(&self, config_path: &Path, force: bool) -> Result<()> {
        if self.dependencies.is_empty() {
            info!("Updating all dependencies");
        } else {
            info!("Updating dependencies: {}", self.dependencies.join(", "));
        }

        debug!("Using config file: {}", config_path.display());
        debug!("Force mode: {}", force);

        let adapter = CliAdapter::new(config_path.to_path_buf());
        let deps = if self.dependencies.is_empty() {
            None
        } else {
            Some(self.dependencies.clone())
        };

        adapter.update_dependencies(deps, self.message.clone(), force)?;

        info!("Dependencies updated successfully");
        Ok(())
    }
}
