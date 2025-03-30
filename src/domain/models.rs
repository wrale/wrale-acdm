// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::PathBuf;

/// Represents an external content dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Unique identifier for the dependency
    pub name: String,
    
    /// Git repository URL (SSH or HTTPS)
    pub repository_url: String,
    
    /// Git revision (branch, tag, or commit)
    pub revision: String,
    
    /// Repository type (currently only 'git' is supported)
    pub repository_type: RepositoryType,
    
    /// Patterns for selecting specific paths from the repository
    pub sparse_paths: Vec<String>,
    
    /// Target location in the project where content will be placed
    pub target_location: PathBuf,
}

/// Type of repository for the dependency
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryType {
    Git,
    // Future support for other repository types can be added here
}

/// Repository authentication information
#[derive(Debug, Clone)]
pub struct RepositoryAuth {
    pub auth_type: AuthType,
    pub credentials: Option<String>,
}

/// Type of authentication for the repository
#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    None,
    Ssh,
    HttpsBasic,
    HttpsToken,
}

/// Configuration for the application
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Default location for vendored content
    pub default_location: Option<PathBuf>,
    
    /// List of all dependencies
    pub dependencies: Vec<Dependency>,
}
