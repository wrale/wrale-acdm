// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use anyhow::Result;
use clap::Args;

use crate::interfaces::cli::CliAdapter;

/// Include paths in a dependency
#[derive(Args)]
pub struct IncludeCommand {
    /// Name of the dependency
    dependency_name: String,
    
    /// Paths to include
    paths: Vec<String>,
}

impl IncludeCommand {
    pub fn execute(&self, config_path: &Path) -> Result<()> {
        println!("Including paths in dependency '{}'", self.dependency_name);
        
        let adapter = CliAdapter::new(config_path.to_path_buf());
        adapter.include_paths(self.dependency_name.clone(), self.paths.clone())?;
        
        println!("Paths included successfully");
        Ok(())
    }
}
