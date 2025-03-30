// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

use wrale_acdm::domain::models::{Configuration, Dependency, RepositoryType};
use wrale_acdm::domain::repositories::ConfigurationRepository;
use wrale_acdm::infrastructure::configuration::TomlConfigurationRepository;

#[test]
fn test_config_init_and_load() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let config_path = temp_dir.path().join("acdm.toml");

    // Create the repository
    let config_repo = TomlConfigurationRepository::new();

    // Initialize a new configuration
    config_repo
        .init(&config_path)
        .expect("Failed to initialize configuration");

    // Check that the file was created
    assert!(config_path.exists(), "Config file was not created");

    // Load the configuration
    let config = config_repo
        .load(&config_path)
        .expect("Failed to load configuration");

    // Check default values
    assert!(
        config.dependencies.is_empty(),
        "Expected empty dependencies"
    );
    assert!(
        config.default_location.is_none(),
        "Expected no default location"
    );
}

#[test]
fn test_config_save_and_load() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let config_path = temp_dir.path().join("acdm.toml");

    // Create the repository
    let config_repo = TomlConfigurationRepository::new();

    // Create a configuration
    let mut config = Configuration {
        default_location: Some(PathBuf::from("vendor")),
        dependencies: vec![],
    };

    // Add a dependency
    config.dependencies.push(Dependency {
        name: "test-dep".to_string(),
        repository_url: "git@github.com:example/repo.git".to_string(),
        revision: "main".to_string(),
        repository_type: RepositoryType::Git,
        sparse_paths: vec!["docs/**".to_string(), "src/**".to_string()],
        target_location: PathBuf::from("vendor/test"),
    });

    // Save the configuration
    config_repo
        .save(&config, &config_path)
        .expect("Failed to save configuration");

    // Check that the file was created
    assert!(config_path.exists(), "Config file was not created");

    // Read the file content for verification
    let content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(
        content.contains("vendor"),
        "Default location not found in config"
    );
    assert!(
        content.contains("test-dep"),
        "Dependency name not found in config"
    );
    assert!(
        content.contains("git@github.com:example/repo.git"),
        "Repository URL not found in config"
    );
    assert!(
        content.contains("docs/**"),
        "Sparse path not found in config"
    );

    // Load the configuration
    let loaded_config = config_repo
        .load(&config_path)
        .expect("Failed to load configuration");

    // Verify the loaded configuration
    assert_eq!(
        loaded_config.default_location,
        Some(PathBuf::from("vendor")),
        "Default location mismatch"
    );
    assert_eq!(
        loaded_config.dependencies.len(),
        1,
        "Expected one dependency"
    );
    assert_eq!(
        loaded_config.dependencies[0].name, "test-dep",
        "Dependency name mismatch"
    );
    assert_eq!(
        loaded_config.dependencies[0].sparse_paths.len(),
        2,
        "Expected two sparse paths"
    );
}

#[test]
fn test_invalid_config() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let config_path = temp_dir.path().join("acdm.toml");

    // Write invalid TOML content
    fs::write(&config_path, "this is not valid TOML").expect("Failed to write file");

    // Create the repository
    let config_repo = TomlConfigurationRepository::new();

    // Try to load the configuration
    let result = config_repo.load(&config_path);

    // Check that it fails
    assert!(
        result.is_err(),
        "Expected error loading invalid configuration"
    );
}
