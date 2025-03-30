// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::Result;
use clap::Args;
use log::{debug, info};
use std::path::Path;

use crate::interfaces::cli::CliAdapter;

/// Add a new dependency
#[derive(Args)]
pub struct AddCommand {
    /// Repository URL
    repository_url: String,

    /// Name for the dependency
    #[clap(long)]
    name: String,

    /// Revision (branch, tag, or commit)
    #[clap(long, default_value = "main")]
    rev: String,

    /// Target location for the dependency
    #[clap(long)]
    target: String,

    /// Skip automatically committing changes
    #[clap(long)]
    skip_commit: bool,
}

impl AddCommand {
    pub fn execute(&self, config_path: &Path, force: bool) -> Result<()> {
        info!(
            "Adding dependency '{}' from {}",
            self.name, self.repository_url
        );
        debug!("Using revision: {}, target: {}", self.rev, self.target);
        debug!("Force mode: {}", force);

        let adapter = CliAdapter::new(config_path.to_path_buf());
        adapter.add_dependency(
            self.name.clone(),
            self.repository_url.clone(),
            self.rev.clone(),
            self.target.clone(),
            self.skip_commit,
            force,
        )?;

        info!("Dependency added successfully");
        Ok(())
    }
}
