// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::PathBuf;
use anyhow::{Result, Context};

use crate::application::dto::{InitConfigDto, AddDependencyDto, IncludePathsDto, UpdateDependenciesDto};
use crate::application::use_cases::{
    InitConfigUseCase, AddDependencyUseCase, IncludePathsUseCase, UpdateDependenciesUseCase
};
use crate::infrastructure::configuration::TomlConfigurationRepository;
use crate::infrastructure::git::{GitOperationsImpl, GitRepositoryFetcher};
use crate::infrastructure::file_system::FileSystemManagerImpl;

/// Adapter for the CLI interface
pub struct CliAdapter {
    config_path: PathBuf,
}

impl CliAdapter {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }
    
    /// Initialize a new configuration
    pub fn init(&self, default_location: Option<String>) -> Result<()> {
        let config_repo = TomlConfigurationRepository::new();
        let use_case = InitConfigUseCase::new(config_repo);
        
        use_case.execute(InitConfigDto {
            config_path: self.config_path.clone(),
            default_location,
        }).context("Failed to initialize configuration")
    }
    
    /// Add a new dependency
    pub fn add_dependency(
        &self,
        name: String,
        repository_url: String,
        revision: String,
        target_location: String,
    ) -> Result<()> {
        let config_repo = TomlConfigurationRepository::new();
        let use_case = AddDependencyUseCase::new(config_repo);
        
        use_case.execute(&self.config_path, AddDependencyDto {
            name,
            repository_url,
            revision,
            repository_type: "git".to_string(),
            target_location,
        }).context("Failed to add dependency")
    }
    
    /// Include paths in a dependency
    pub fn include_paths(&self, dependency_name: String, paths: Vec<String>) -> Result<()> {
        let config_repo = TomlConfigurationRepository::new();
        let use_case = IncludePathsUseCase::new(config_repo);
        
        use_case.execute(&self.config_path, IncludePathsDto {
            dependency_name,
            paths,
        }).context("Failed to include paths")
    }
    
    /// Update dependencies
    pub fn update_dependencies(
        &self,
        dependencies: Option<Vec<String>>,
        commit_message: Option<String>,
    ) -> Result<()> {
        let config_repo = TomlConfigurationRepository::new();
        let repository_fetcher = GitRepositoryFetcher::new();
        let file_system_manager = FileSystemManagerImpl::new();
        let git_operations = GitOperationsImpl::new();
        
        let use_case = UpdateDependenciesUseCase::new(
            config_repo,
            repository_fetcher,
            file_system_manager,
            git_operations,
        );
        
        use_case.execute(UpdateDependenciesDto {
            config_path: self.config_path.clone(),
            dependencies,
            commit_message,
        }).context("Failed to update dependencies")
    }
}
