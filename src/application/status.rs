// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use anyhow::{Context, Result};
use std::path::Path;

use crate::application::dto::DependencyStatusDto;
use crate::domain::repositories::ConfigurationRepository;
use crate::domain::DomainError;

/// Query for showing dependency status
pub struct GetDependencyStatusQuery<C: ConfigurationRepository> {
    config_repo: C,
}

impl<C: ConfigurationRepository> GetDependencyStatusQuery<C> {
    pub fn new(config_repo: C) -> Self {
        Self { config_repo }
    }

    /// Get the status of all dependencies
    pub fn get_all_statuses(&self, config_path: &Path) -> Result<Vec<DependencyStatusDto>> {
        // Convert the config_path to an absolute path if it's relative
        let absolute_config_path = if config_path.is_absolute() {
            config_path.to_path_buf()
        } else {
            // Join with current directory to get absolute path
            std::env::current_dir()
                .map_err(|e| {
                    DomainError::ConfigurationError(format!(
                        "Failed to get current directory: {}",
                        e
                    ))
                })?
                .join(config_path)
        };

        // Load the configuration
        let config = self
            .config_repo
            .load(&absolute_config_path)
            .context("Failed to load configuration")?;

        // Get the repository root - if we have a parent directory, use it, otherwise use the current directory
        let repo_root = if let Some(parent) = absolute_config_path.parent() {
            parent.to_path_buf()
        } else {
            // This should rarely happen with absolute paths, but handle it anyway
            std::env::current_dir().map_err(|e| {
                DomainError::ConfigurationError(format!("Failed to get current directory: {}", e))
            })?
        };

        // Create status DTOs for each dependency
        let mut statuses = Vec::new();

        for dep in &config.dependencies {
            // Determine the absolute target path
            let target_path = repo_root.join(&dep.target_location);

            // Determine status
            let status = if !target_path.exists() {
                "Not fetched".to_string()
            } else {
                "Fetched".to_string()
            };

            // Create the DTO
            let dto = DependencyStatusDto {
                name: dep.name.clone(),
                repository_url: dep.repository_url.clone(),
                revision: dep.revision.clone(),
                target_location: dep.target_location.to_string_lossy().to_string(),
                sparse_paths: dep.sparse_paths.clone(),
                status,
            };

            statuses.push(dto);
        }

        Ok(statuses)
    }
}
