// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::{Path, PathBuf};
use crate::domain::error::DomainError;

/// Interface for repository operations
pub trait RepositoryFetcher {
    /// Fetches content from a remote repository to a local temporary directory
    fn fetch(&self, url: &str, revision: &str, temp_path: &Path) -> Result<(), DomainError>;
    
    /// Extracts specific paths from a repository based on glob patterns
    fn extract_paths(&self, 
                    repo_path: &Path, 
                    patterns: &[String], 
                    target_path: &Path) -> Result<(), DomainError>;
}

/// Interface for file system operations
pub trait FileSystemManager {
    /// Cleans a directory by removing all its contents
    fn clean_directory(&self, path: &Path) -> Result<(), DomainError>;
    
    /// Copies content from source to destination
    fn copy_content(&self, 
                   source: &Path, 
                   destination: &Path) -> Result<(), DomainError>;
                   
    /// Creates a temporary directory
    fn create_temp_directory(&self) -> Result<PathBuf, DomainError>;
    
    /// Removes a temporary directory
    fn remove_temp_directory(&self, path: &Path) -> Result<(), DomainError>;
}

/// Interface for Git operations
pub trait GitOperations {
    /// Stages all changes in a directory
    fn stage_all(&self, repo_path: &Path) -> Result<(), DomainError>;
    
    /// Commits changes with a message
    fn commit(&self, repo_path: &Path, message: &str) -> Result<(), DomainError>;
    
    /// Checks if a directory is inside a Git repository
    fn is_git_repository(&self, path: &Path) -> Result<bool, DomainError>;
}

/// Interface for configuration operations
pub trait ConfigurationRepository {
    /// Loads configuration from a file
    fn load(&self, path: &Path) -> Result<crate::domain::Configuration, DomainError>;
    
    /// Saves configuration to a file
    fn save(&self, 
           config: &crate::domain::Configuration, 
           path: &Path) -> Result<(), DomainError>;
           
    /// Initializes a new configuration file
    fn init(&self, path: &Path) -> Result<(), DomainError>;
}
