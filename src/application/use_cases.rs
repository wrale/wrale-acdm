// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

use crate::domain::{Dependency, Configuration, RepositoryType};
use crate::domain::repositories::{ConfigurationRepository, RepositoryFetcher, FileSystemManager, GitOperations};
use crate::domain::services::DependencyManager;
use crate::application::dto::{InitConfigDto, AddDependencyDto, IncludePathsDto, UpdateDependenciesDto};

/// Use case for initializing a new configuration
pub struct InitConfigUseCase<C: ConfigurationRepository> {
    config_repo: C,
}

impl<C: ConfigurationRepository> InitConfigUseCase<C> {
    pub fn new(config_repo: C) -> Self {
        Self { config_repo }
    }
    
    pub fn execute(&self, dto: InitConfigDto) -> Result<()> {
        // Initialize the configuration
        self.config_repo.init(&dto.config_path)
            .context("Failed to initialize configuration")?;
            
        // If a default location was provided, update the config
        if let Some(location) = dto.default_location {
            let mut config = self.config_repo.load(&dto.config_path)
                .context("Failed to load configuration")?;
                
            config.default_location = Some(PathBuf::from(location));
            
            self.config_repo.save(&config, &dto.config_path)
                .context("Failed to save configuration")?;
        }
        
        Ok(())
    }
}

/// Use case for adding a new dependency
pub struct AddDependencyUseCase<C: ConfigurationRepository> {
    config_repo: C,
}

impl<C: ConfigurationRepository> AddDependencyUseCase<C> {
    pub fn new(config_repo: C) -> Self {
        Self { config_repo }
    }
    
    pub fn execute(&self, config_path: &Path, dto: AddDependencyDto) -> Result<()> {
        // Load the configuration
        let mut config = self.config_repo.load(config_path)
            .context("Failed to load configuration")?;
            
        // Check if a dependency with the same name already exists
        if config.dependencies.iter().any(|d| d.name == dto.name) {
            return Err(anyhow::anyhow!("A dependency with the name '{}' already exists", dto.name));
        }
        
        // Parse the repository type
        let repo_type = match dto.repository_type.to_lowercase().as_str() {
            "git" => RepositoryType::Git,
            _ => return Err(anyhow::anyhow!("Unsupported repository type: {}", dto.repository_type)),
        };
        
        // Create a new dependency
        let dependency = Dependency {
            name: dto.name,
            repository_url: dto.repository_url,
            revision: dto.revision,
            repository_type: repo_type,
            sparse_paths: Vec::new(),
            target_location: PathBuf::from(dto.target_location),
        };
        
        // Add the dependency to the configuration
        config.dependencies.push(dependency);
        
        // Save the configuration
        self.config_repo.save(&config, config_path)
            .context("Failed to save configuration")?;
            
        Ok(())
    }
}

/// Use case for including paths in a dependency
pub struct IncludePathsUseCase<C: ConfigurationRepository> {
    config_repo: C,
}

impl<C: ConfigurationRepository> IncludePathsUseCase<C> {
    pub fn new(config_repo: C) -> Self {
        Self { config_repo }
    }
    
    pub fn execute(&self, config_path: &Path, dto: IncludePathsDto) -> Result<()> {
        // Load the configuration
        let mut config = self.config_repo.load(config_path)
            .context("Failed to load configuration")?;
            
        // Find the dependency by name
        let dep_idx = config.dependencies.iter().position(|d| d.name == dto.dependency_name)
            .ok_or_else(|| anyhow::anyhow!("Dependency '{}' not found", dto.dependency_name))?;
            
        // Add the paths to the dependency
        for path in dto.paths {
            if !config.dependencies[dep_idx].sparse_paths.contains(&path) {
                config.dependencies[dep_idx].sparse_paths.push(path);
            }
        }
        
        // Save the configuration
        self.config_repo.save(&config, config_path)
            .context("Failed to save configuration")?;
            
        Ok(())
    }
}

/// Use case for updating dependencies
pub struct UpdateDependenciesUseCase<C, R, F, G>
where
    C: ConfigurationRepository,
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    config_repo: C,
    dependency_manager: DependencyManager<R, F, G>,
}

impl<C, R, F, G> UpdateDependenciesUseCase<C, R, F, G>
where
    C: ConfigurationRepository,
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    pub fn new(
        config_repo: C,
        repository_fetcher: R,
        file_system_manager: F,
        git_operations: G,
    ) -> Self {
        Self {
            config_repo,
            dependency_manager: DependencyManager::new(
                repository_fetcher,
                file_system_manager,
                git_operations,
            ),
        }
    }
    
    pub fn execute(&self, dto: UpdateDependenciesDto) -> Result<()> {
        // Load the configuration
        let config = self.config_repo.load(&dto.config_path)
            .context("Failed to load configuration")?;
            
        // Get the dependencies to update
        let dependencies_to_update = if let Some(dep_names) = dto.dependencies {
            config.dependencies.iter()
                .filter(|d| dep_names.contains(&d.name))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            config.dependencies.clone()
        };
        
        if dependencies_to_update.is_empty() {
            return Err(anyhow::anyhow!("No dependencies found to update"));
        }
        
        // Get the repository root (the directory containing the config file)
        let repo_root = dto.config_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine repository root"))?;
            
        // Update all dependencies
        self.dependency_manager.update_all(
            &dependencies_to_update,
            repo_root,
            dto.commit_message.as_deref(),
        ).map_err(|e| anyhow::anyhow!("Failed to update dependencies: {}", e))?;
        
        Ok(())
    }
}
