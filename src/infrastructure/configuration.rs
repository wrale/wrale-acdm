// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::domain::repositories::ConfigurationRepository;
use crate::domain::{Configuration, Dependency, DomainError, RepositoryType};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    location: Option<String>,
    sources: Vec<SourceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SourceConfig {
    repo: String,
    name: String,
    rev: String,
    #[serde(rename = "type")]
    repo_type: String,
    sparse_paths: Vec<String>,
    target: String,
}

pub struct TomlConfigurationRepository;

impl Default for TomlConfigurationRepository {
    fn default() -> Self {
        Self
    }
}

impl TomlConfigurationRepository {
    pub fn new() -> Self {
        Self
    }

    fn domain_to_toml(&self, config: &Configuration) -> ConfigFile {
        let sources = config
            .dependencies
            .iter()
            .map(|dep| {
                let repo_type = match dep.repository_type {
                    RepositoryType::Git => "git".to_string(),
                    // Add other repository types here
                };

                SourceConfig {
                    repo: dep.repository_url.clone(),
                    name: dep.name.clone(),
                    rev: dep.revision.clone(),
                    repo_type,
                    sparse_paths: dep.sparse_paths.clone(),
                    target: dep.target_location.to_string_lossy().to_string(),
                }
            })
            .collect();

        ConfigFile {
            location: config
                .default_location
                .as_ref()
                .map(|l| l.to_string_lossy().to_string()),
            sources,
        }
    }

    fn toml_to_domain(&self, config_file: ConfigFile) -> Result<Configuration, DomainError> {
        let dependencies = config_file
            .sources
            .into_iter()
            .map(|source| {
                let repository_type = match source.repo_type.as_str() {
                    "git" => RepositoryType::Git,
                    unsupported => {
                        return Err(DomainError::ConfigurationError(format!(
                            "Unsupported repository type: {}",
                            unsupported
                        )))
                    }
                };

                Ok(Dependency {
                    name: source.name,
                    repository_url: source.repo,
                    revision: source.rev,
                    repository_type,
                    sparse_paths: source.sparse_paths,
                    target_location: PathBuf::from(source.target),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Configuration {
            default_location: config_file.location.map(PathBuf::from),
            dependencies,
        })
    }
}

impl ConfigurationRepository for TomlConfigurationRepository {
    fn load(&self, path: &Path) -> Result<Configuration, DomainError> {
        let mut file = File::open(path).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to open configuration file: {}", e))
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to read configuration file: {}", e))
        })?;

        let config_file: ConfigFile = toml::from_str(&contents).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to parse configuration file: {}", e))
        })?;

        self.toml_to_domain(config_file)
    }

    fn save(&self, config: &Configuration, path: &Path) -> Result<(), DomainError> {
        let config_file = self.domain_to_toml(config);

        let toml_string = toml::to_string_pretty(&config_file).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to serialize configuration: {}", e))
        })?;

        // Ensure the directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                DomainError::FileSystemError(format!("Failed to create directory: {}", e))
            })?;
        }

        let mut file = File::create(path).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to create configuration file: {}", e))
        })?;

        file.write_all(toml_string.as_bytes()).map_err(|e| {
            DomainError::ConfigurationError(format!("Failed to write configuration file: {}", e))
        })?;

        Ok(())
    }

    fn init(&self, path: &Path) -> Result<(), DomainError> {
        // Create an empty configuration
        let config = Configuration {
            default_location: None,
            dependencies: vec![],
        };

        self.save(&config, path)
    }
}
