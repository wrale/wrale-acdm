// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use crate::domain::error::DomainError;
use crate::domain::models::{AuthType, RepositoryAuth};
use std::fmt;

/// Example domain error handler
pub struct ErrorHandler;

impl Default for ErrorHandler {
    fn default() -> Self {
        Self
    }
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new() -> Self {
        Self
    }

    /// Generate example errors for testing
    #[allow(dead_code)] // This method is for testing
    pub fn generate_error(&self, error_type: &str) -> DomainError {
        match error_type {
            "repository" => DomainError::RepositoryError("Example repository error".to_string()),
            "filesystem" => DomainError::FileSystemError("Example file system error".to_string()),
            "config" => DomainError::ConfigurationError("Example configuration error".to_string()),
            "dependency" => {
                DomainError::InvalidDependencyError("Example dependency error".to_string())
            }
            "git" => DomainError::GitError("Example git error".to_string()),
            "pattern" => DomainError::PathPatternError("Example path pattern error".to_string()),
            _ => DomainError::UnknownError("Unknown error type".to_string()),
        }
    }

    /// Display error information
    pub fn display_error(&self, error: &DomainError) -> String {
        format!("Error: {}", error)
    }
}

/// Example authentication helper
pub struct AuthHelper;

impl Default for AuthHelper {
    fn default() -> Self {
        Self
    }
}

impl AuthHelper {
    /// Create a new auth helper
    pub fn new() -> Self {
        Self
    }

    /// Create a simple credentials formatter
    pub fn format_credentials(&self, auth: &RepositoryAuth) -> String {
        match auth.auth_type {
            AuthType::None => "No authentication".to_string(),
            AuthType::Ssh => "SSH key authentication".to_string(),
            AuthType::HttpsBasic => {
                if let Some(creds) = &auth.credentials {
                    // Mask the password part for security
                    if let Some(idx) = creds.find(':') {
                        let username = &creds[..idx];
                        format!("HTTP Basic: {}:******", username)
                    } else {
                        "Invalid HTTP Basic format".to_string()
                    }
                } else {
                    "Missing HTTP Basic credentials".to_string()
                }
            }
            AuthType::HttpsToken => {
                if auth.credentials.is_some() {
                    "HTTP Token: ******".to_string()
                } else {
                    "Missing HTTP Token".to_string()
                }
            }
        }
    }
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthType::None => write!(f, "None"),
            AuthType::Ssh => write!(f, "SSH"),
            AuthType::HttpsBasic => write!(f, "HTTPS Basic"),
            AuthType::HttpsToken => write!(f, "HTTPS Token"),
        }
    }
}
