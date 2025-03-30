// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use anyhow::Result;
use clap::Args;

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
    pub fn execute(&self, config_path: &Path) -> Result<()> {
        if self.dependencies.is_empty() {
            println!("Updating all dependencies");
        } else {
            println!("Updating dependencies: {}", self.dependencies.join(", "));
        }
        
        let adapter = CliAdapter::new(config_path.to_path_buf());
        let deps = if self.dependencies.is_empty() {
            None
        } else {
            Some(self.dependencies.clone())
        };
        
        adapter.update_dependencies(deps, self.message.clone())?;
        
        println!("Dependencies updated successfully");
        Ok(())
    }
}
