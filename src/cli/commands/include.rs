// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::Result;
use clap::Args;
use log::{debug, info};
use std::path::Path;

use crate::interfaces::cli::CliAdapter;

/// Include paths in a dependency
#[derive(Args)]
pub struct IncludeCommand {
    /// Name of the dependency
    dependency_name: String,

    /// Paths to include
    paths: Vec<String>,

    /// Skip automatically committing changes
    #[clap(long)]
    skip_commit: bool,
}

impl IncludeCommand {
    pub fn execute(&self, config_path: &Path, force: bool) -> Result<()> {
        info!("Including paths in dependency '{}'", self.dependency_name);
        debug!("Paths: {:?}", self.paths);
        debug!("Force mode: {}", force);

        let adapter = CliAdapter::new(config_path.to_path_buf());
        adapter.include_paths(
            self.dependency_name.clone(),
            self.paths.clone(),
            self.skip_commit,
            force,
        )?;

        info!("Paths included successfully");
        Ok(())
    }
}
