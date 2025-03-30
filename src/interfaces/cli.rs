// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use crate::domain::repositories::{ConfigurationRepository, GitOperations};
use anyhow::{anyhow, Context, Result};
use log::{debug, info, warn};
use std::io::Write;
use std::path::PathBuf;

use crate::application::dto::{
    AddDependencyDto, IncludePathsDto, InitConfigDto, UpdateDependenciesDto,
};
use crate::application::use_cases::{
    AddDependencyUseCase, IncludePathsUseCase, InitConfigUseCase, UpdateDependenciesUseCase,
};
use crate::infrastructure::configuration::TomlConfigurationRepository;
use crate::infrastructure::file_system::FileSystemManagerImpl;
use crate::infrastructure::git::{GitOperationsImpl, GitRepositoryFetcher};

/// Adapter for the CLI interface
pub struct CliAdapter {
    config_path: PathBuf,
}

impl CliAdapter {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    /// Initialize a new configuration
    pub fn init(&self, default_location: Option<String>, force: bool) -> Result<()> {
        debug!(
            "Initializing configuration with default location: {:?}, force: {}",
            default_location, force
        );

        // Check if config file already exists and force is not enabled
        if self.config_path.exists() && !force {
            return Err(anyhow!(
                "Configuration file already exists at {}. Use --force to overwrite.",
                self.config_path.display()
            ));
        }

        let config_repo = TomlConfigurationRepository::new();
        let use_case = InitConfigUseCase::new(config_repo);

        use_case
            .execute(InitConfigDto {
                config_path: self.config_path.clone(),
                default_location,
            })
            .context("Failed to initialize configuration")
    }

    /// Add a new dependency
    pub fn add_dependency(
        &self,
        name: String,
        repository_url: String,
        revision: String,
        target_location: String,
        force: bool,
    ) -> Result<()> {
        debug!(
            "Adding dependency: name={}, url={}, rev={}, target={}",
            name, repository_url, revision, target_location
        );

        // Create Git operations and verify clean status
        let git_operations = GitOperationsImpl::new();

        // Get the repository root - if we have a parent directory, use it, otherwise use the current directory
        let repo_root = if let Some(parent) = self.config_path.parent() {
            // If parent is empty, use current directory
            if parent.as_os_str().is_empty() {
                std::env::current_dir()
                    .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
            } else {
                parent.to_path_buf()
            }
        } else {
            // No parent means the config is in the current directory
            std::env::current_dir()
                .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
        };

        debug!("Using repository root path: {}", repo_root.display());

        // Skip this check if force is enabled
        if !force {
            if let Err(e) = self.ensure_clean_git_status(&git_operations, &repo_root) {
                warn!("Git repository status is not clean: {}", e);
                return Err(e);
            }
        }

        let config_repo = TomlConfigurationRepository::new();
        let use_case = AddDependencyUseCase::new(config_repo);

        use_case
            .execute(
                &self.config_path,
                AddDependencyDto {
                    name: name.clone(),
                    repository_url,
                    revision,
                    repository_type: "git".to_string(),
                    target_location,
                },
            )
            .context("Failed to add dependency")?;

        info!("Remember to commit your changes manually with 'git add . && git commit -m \"Add dependency {name}\"'");

        Ok(())
    }

    /// Include paths in a dependency
    pub fn include_paths(
        &self,
        dependency_name: String,
        paths: Vec<String>,
        force: bool,
    ) -> Result<()> {
        debug!(
            "Including paths for dependency: {}, paths: {:?}",
            dependency_name, paths
        );

        // Create Git operations and verify clean status
        let git_operations = GitOperationsImpl::new();

        // Get the repository root - if we have a parent directory, use it, otherwise use the current directory
        let repo_root = if let Some(parent) = self.config_path.parent() {
            // If parent is empty, use current directory
            if parent.as_os_str().is_empty() {
                std::env::current_dir()
                    .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
            } else {
                parent.to_path_buf()
            }
        } else {
            // No parent means the config is in the current directory
            std::env::current_dir()
                .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
        };

        debug!("Using repository root path: {}", repo_root.display());

        // Skip this check if force is enabled
        if !force {
            if let Err(e) = self.ensure_clean_git_status(&git_operations, &repo_root) {
                warn!("Git repository status is not clean: {}", e);
                return Err(e);
            }
        }

        let config_repo = TomlConfigurationRepository::new();
        let use_case = IncludePathsUseCase::new(config_repo);

        use_case
            .execute(
                &self.config_path,
                IncludePathsDto {
                    dependency_name: dependency_name.clone(),
                    paths: paths.clone(),
                },
            )
            .context("Failed to include paths")?;

        info!("Remember to commit your changes manually with 'git add . && git commit -m \"Include paths for {dependency_name}\"'");

        Ok(())
    }

    /// Update dependencies
    pub fn update_dependencies(
        &self,
        dependencies: Option<Vec<String>>,
        force: bool,
    ) -> Result<()> {
        debug!(
            "Updating dependencies: {:?}, force: {}",
            dependencies, force
        );

        // Create required components
        let config_repo = TomlConfigurationRepository::new();
        let repository_fetcher = GitRepositoryFetcher::new();
        let file_system_manager = FileSystemManagerImpl::new();
        let git_operations = GitOperationsImpl::new();

        // Get the repository root - if we have a parent directory, use it, otherwise use the current directory
        let repo_root = if let Some(parent) = self.config_path.parent() {
            // If parent is empty, use current directory
            if parent.as_os_str().is_empty() {
                std::env::current_dir()
                    .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
            } else {
                parent.to_path_buf()
            }
        } else {
            // No parent means the config is in the current directory
            std::env::current_dir()
                .map_err(|e| anyhow!("Failed to get current directory: {}", e))?
        };

        debug!("Using repository root path: {}", repo_root.display());

        // Verify Git status if not in force mode
        if !force {
            if let Err(e) = self.ensure_clean_git_status(&git_operations, &repo_root) {
                warn!("Git repository status is not clean: {}", e);
                return Err(e);
            }
        }

        // Load configuration and determine what will be updated
        let config = config_repo
            .load(&self.config_path)
            .context("Failed to load configuration")?;

        // Get the dependencies to update
        let dependencies_to_update = if let Some(dep_names) = dependencies.as_ref() {
            let filtered_deps = config
                .dependencies
                .iter()
                .filter(|d| dep_names.contains(&d.name))
                .collect::<Vec<_>>();

            if filtered_deps.is_empty() {
                return Err(anyhow!("No matching dependencies found to update"));
            }

            filtered_deps
        } else {
            if config.dependencies.is_empty() {
                return Err(anyhow!("No dependencies found to update"));
            }

            config.dependencies.iter().collect()
        };

        // Show warning about what mount points will be purged
        if !force {
            info!("The following mount points will be purged:");
            for dep in dependencies_to_update.iter() {
                info!("  - {}", dep.target_location.display());
            }

            if !self.prompt_yes_no("Do you want to continue with the update?")? {
                info!("Update canceled by user");
                return Ok(());
            }
        }

        // Run the update
        let use_case = UpdateDependenciesUseCase::new(
            config_repo,
            repository_fetcher,
            file_system_manager,
            git_operations,
        );

        debug!("Executing update dependencies use case");
        use_case
            .execute(UpdateDependenciesDto {
                config_path: self.config_path.clone(),
                dependencies: dependencies.clone(),
                force,
            })
            .context("Failed to update dependencies")?;

        info!("Remember to commit your changes manually with 'git add . && git commit -m \"Update dependencies\"'");

        Ok(())
    }

    /// Show dependency status
    pub fn show_dependency_status(
        &self,
        dependencies: Option<Vec<String>>,
        detailed: bool,
    ) -> Result<()> {
        // Add error handler for better error messages
        let error_handler = crate::domain::examples::ErrorHandler::new();
        // Create components
        let config_repo = TomlConfigurationRepository::new();
        let status_query = crate::application::status::GetDependencyStatusQuery::new(config_repo);

        // Get statuses
        let statuses = match status_query.get_all_statuses(&self.config_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "{}",
                    error_handler.display_error(&crate::domain::DomainError::ConfigurationError(
                        e.to_string()
                    ))
                );
                return Err(e);
            }
        };

        // Filter by dependencies if specified
        let filtered_statuses = if let Some(dep_names) = dependencies {
            statuses
                .into_iter()
                .filter(|s| dep_names.contains(&s.name))
                .collect::<Vec<_>>()
        } else {
            statuses
        };

        if filtered_statuses.is_empty() {
            println!("No dependencies found");
            return Ok(());
        }

        // Display the statuses
        println!("Dependencies:");
        for status in filtered_statuses {
            println!("  - {}: {}", status.name, status.status);

            if detailed {
                println!("    Repository: {}", status.repository_url);
                println!("    Revision:   {}", status.revision);
                println!("    Target:     {}", status.target_location);
                println!("    Paths:      {}", status.sparse_paths.join(", "));

                // Use auth service to get auth info
                let auth_service = crate::domain::auth::AuthenticationService::new();
                if let Some(auth) = auth_service.get_auth_for_repository(&status.repository_url) {
                    let auth_helper = crate::domain::examples::AuthHelper::new();
                    let auth_info = auth_helper.format_credentials(&auth);
                    println!("    Auth:       {}", auth_info);
                }
            }
        }

        Ok(())
    }

    /// Prompt user for confirmation with yes/no
    fn prompt_yes_no(&self, message: &str) -> Result<bool> {
        let mut input = String::new();
        print!("{} [y/N]: ", message);
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y")
    }

    /// Ensure the Git repository has a clean status and exists
    fn ensure_clean_git_status(
        &self,
        git_ops: &GitOperationsImpl,
        repo_path: &std::path::Path,
    ) -> Result<()> {
        debug!(
            "Verifying Git repository status for path: {}",
            repo_path.display()
        );

        // First check if it's a git repository at all
        if !git_ops.is_git_repository(repo_path)? {
            return Err(anyhow!("Directory is not a Git repository. This tool requires operations to be performed within a Git repository."));
        }

        // Check if it has a clean status
        let status = git_ops.get_status(repo_path)?;
        if !status.is_clean() {
            // Run git status to show the user what's going on
            let git_status_output = std::process::Command::new("git")
                .args(["status", "--short"])
                .current_dir(repo_path)
                .output();

            let mut error_msg = String::from("Git repository has uncommitted changes. Please commit your changes before proceeding or use the --force flag (NOT RECOMMENDED)");

            // Add git status output if available
            if let Ok(output) = git_status_output {
                if output.status.success() {
                    let status_text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !status_text.is_empty() {
                        error_msg.push_str("\n\nGit status:\n");
                        error_msg.push_str(&status_text);
                    }
                }
            }

            return Err(anyhow!(error_msg));
        }

        Ok(())
    }
}
