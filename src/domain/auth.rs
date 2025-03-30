// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use crate::domain::models::{AuthType, RepositoryAuth};

/// Authentication service for repository access
pub struct AuthenticationService;

impl Default for AuthenticationService {
    fn default() -> Self {
        Self
    }
}

impl AuthenticationService {
    /// Create a new authentication service
    pub fn new() -> Self {
        Self
    }

    /// Get authentication information for a repository URL
    pub fn get_auth_for_repository(&self, url: &str) -> Option<RepositoryAuth> {
        // For SSH URLs, use SSH authentication
        if url.starts_with("git@") {
            return Some(RepositoryAuth {
                auth_type: AuthType::Ssh,
                credentials: None, // SSH keys are handled by the system
            });
        }

        // For HTTPS URLs, check if a token is available
        if url.starts_with("https://") {
            // This would typically check environment variables or keychain for tokens
            let token = std::env::var("GIT_TOKEN").ok();
            if token.is_some() {
                return Some(RepositoryAuth {
                    auth_type: AuthType::HttpsToken,
                    credentials: token,
                });
            }

            // For public repositories, no auth is needed
            return Some(RepositoryAuth {
                auth_type: AuthType::None,
                credentials: None,
            });
        }

        // Unsupported URL scheme
        None
    }

    /// Get HTTP Basic auth from username/password
    #[allow(dead_code)] // This will be used in a future version
    pub fn create_basic_auth(username: &str, password: &str) -> RepositoryAuth {
        RepositoryAuth {
            auth_type: AuthType::HttpsBasic,
            credentials: Some(format!("{}:{}", username, password)),
        }
    }
}
