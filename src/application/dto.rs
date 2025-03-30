// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::PathBuf;

/// DTO for initializing a new configuration
pub struct InitConfigDto {
    pub config_path: PathBuf,
    pub default_location: Option<String>,
}

/// DTO for adding a new dependency
pub struct AddDependencyDto {
    pub name: String,
    pub repository_url: String,
    pub revision: String,
    pub repository_type: String,
    pub target_location: String,
}

/// DTO for including paths in a dependency
pub struct IncludePathsDto {
    pub dependency_name: String,
    pub paths: Vec<String>,
}

/// DTO for updating dependencies
pub struct UpdateDependenciesDto {
    pub config_path: PathBuf,
    pub dependencies: Option<Vec<String>>,
    pub commit_message: Option<String>,
}

/// DTO for dependency status
pub struct DependencyStatusDto {
    pub name: String,
    pub repository_url: String,
    pub revision: String,
    pub target_location: String,
    pub sparse_paths: Vec<String>,
    pub status: String,
}
