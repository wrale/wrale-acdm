// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::Result;
use clap::Args;
use log::info;
use std::path::Path;

use crate::interfaces::cli::CliAdapter;

/// Display dependency status
#[derive(Args)]
pub struct StatusCommand {
    /// Dependencies to show status for (all if not specified)
    dependencies: Vec<String>,

    /// Show details about each dependency
    #[clap(short, long)]
    detailed: bool,
}

impl StatusCommand {
    pub fn execute(&self, config_path: &Path, _force: bool) -> Result<()> {
        info!("Displaying dependency status");

        let adapter = CliAdapter::new(config_path.to_path_buf());
        let deps = if self.dependencies.is_empty() {
            None
        } else {
            Some(self.dependencies.clone())
        };

        adapter.show_dependency_status(deps, self.detailed)?;

        Ok(())
    }
}
