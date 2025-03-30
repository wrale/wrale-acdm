// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use crate::domain::{Dependency, DomainError};
use crate::domain::repositories::{RepositoryFetcher, FileSystemManager, GitOperations};

/// Service for updating a single dependency
pub struct DependencyUpdater<R, F, G>
where
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    repository_fetcher: R,
    file_system_manager: F,
    git_operations: G,
}

impl<R, F, G> DependencyUpdater<R, F, G>
where
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    pub fn new(repository_fetcher: R, file_system_manager: F, git_operations: G) -> Self {
        Self {
            repository_fetcher,
            file_system_manager,
            git_operations,
        }
    }
    
    /// Updates a single dependency
    pub fn update(&self, dependency: &Dependency, repo_root: &Path) -> Result<(), DomainError> {
        // Create a temporary directory for fetching the repository
        let temp_dir = self.file_system_manager.create_temp_directory()?;
        
        // Fetch the repository to the temporary directory
        self.repository_fetcher.fetch(
            &dependency.repository_url,
            &dependency.revision,
            &temp_dir,
        )?;
        
        // Determine the absolute target path
        let target_path = repo_root.join(&dependency.target_location);
        
        // Create the target directory if it doesn't exist
        if !target_path.exists() {
            std::fs::create_dir_all(&target_path).map_err(|e| {
                DomainError::FileSystemError(format!("Failed to create target directory: {}", e))
            })?;
        }
        
        // Clean the target directory
        self.file_system_manager.clean_directory(&target_path)?;
        
        // Extract paths from the repository to the target directory
        self.repository_fetcher.extract_paths(
            &temp_dir,
            &dependency.sparse_paths,
            &target_path,
        )?;
        
        // Check if we're in a git repository
        if self.git_operations.is_git_repository(repo_root)? {
            // Stage all changes
            self.git_operations.stage_all(repo_root)?;
        }
        
        // Clean up the temporary directory
        self.file_system_manager.remove_temp_directory(&temp_dir)?;
        
        Ok(())
    }
}

/// Service for managing all dependencies
pub struct DependencyManager<R, F, G>
where
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    dependency_updater: DependencyUpdater<R, F, G>,
}

impl<R, F, G> DependencyManager<R, F, G>
where
    R: RepositoryFetcher,
    F: FileSystemManager,
    G: GitOperations,
{
    pub fn new(repository_fetcher: R, file_system_manager: F, git_operations: G) -> Self {
        Self {
            dependency_updater: DependencyUpdater::new(
                repository_fetcher,
                file_system_manager,
                git_operations,
            ),
        }
    }
    
    /// Updates all dependencies
    pub fn update_all(
        &self,
        dependencies: &[Dependency],
        repo_root: &Path,
        commit_message: Option<&str>,
    ) -> Result<(), DomainError> {
        // Update each dependency
        for dependency in dependencies {
            self.dependency_updater.update(dependency, repo_root)?;
        }
        
        // Commit changes if a commit message is provided and we're in a git repository
        if let Some(message) = commit_message {
            if self.dependency_updater.git_operations.is_git_repository(repo_root)? {
                self.dependency_updater.git_operations.commit(repo_root, message)?;
            }
        }
        
        Ok(())
    }
}
