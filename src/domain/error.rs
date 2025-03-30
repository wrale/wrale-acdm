// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Repository operation failed: {0}")]
    RepositoryError(String),
    
    #[error("File system operation failed: {0}")]
    FileSystemError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Invalid dependency: {0}")]
    InvalidDependencyError(String),
    
    #[error("Git operation failed: {0}")]
    GitError(String),
    
    #[error("Path pattern error: {0}")]
    PathPatternError(String),
    
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
