// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use anyhow::Result;
use clap::Args;

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
}

impl AddCommand {
    pub fn execute(&self, config_path: &Path) -> Result<()> {
        println!("Adding dependency '{}' from {}", self.name, self.repository_url);
        
        let adapter = CliAdapter::new(config_path.to_path_buf());
        adapter.add_dependency(
            self.name.clone(),
            self.repository_url.clone(),
            self.rev.clone(),
            self.target.clone(),
        )?;
        
        println!("Dependency added successfully");
        Ok(())
    }
}
