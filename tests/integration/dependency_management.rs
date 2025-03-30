// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile::tempdir;

use mockall::mock;
use mockall::predicate::*;
use wrale_acdm::domain::error::DomainError;
use wrale_acdm::domain::models::{Dependency, RepositoryType};
use wrale_acdm::domain::repositories::{FileSystemManager, RepositoryFetcher};
use wrale_acdm::domain::services::DependencyUpdater;

// Mock the repository fetcher
mock! {
    pub RepositoryFetcherMock {}
    impl RepositoryFetcher for RepositoryFetcherMock {
        fn fetch(&self, url: &str, revision: &str, temp_path: &Path) -> Result<(), DomainError>;
        fn extract_paths(&self, repo_path: &Path, patterns: &[String], target_path: &Path) -> Result<(), DomainError>;
    }
}

// Mock the file system manager
mock! {
    pub FileSystemManagerMock {}
    impl FileSystemManager for FileSystemManagerMock {
        fn clean_directory(&self, path: &Path) -> Result<(), DomainError>;
        fn copy_content(&self, source: &Path, destination: &Path) -> Result<(), DomainError>;
        fn create_temp_directory(&self) -> Result<PathBuf, DomainError>;
        fn remove_temp_directory(&self, path: &Path) -> Result<(), DomainError>;
    }
}

// Mock for git operations
mock! {
    pub GitOperationsMock {}
    impl wrale_acdm::domain::repositories::GitOperations for GitOperationsMock {
        fn stage_all(&self, repo_path: &Path) -> Result<(), DomainError>;
        fn commit(&self, repo_path: &Path, message: &str) -> Result<(), DomainError>;
        fn is_git_repository(&self, path: &Path) -> Result<bool, DomainError>;
        fn get_status(&self, repo_path: &Path) -> Result<wrale_acdm::domain::repositories::GitStatus, DomainError>;
    }
}

#[test]
fn test_dependency_update_with_mocks() {
    // Create mock instances
    let mut repo_fetcher = MockRepositoryFetcherMock::new();
    let mut fs_manager = MockFileSystemManagerMock::new();
    let mut git_ops = MockGitOperationsMock::new();

    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Set up expectations for the mocks

    // Expect create_temp_directory to be called once and return our temp path
    fs_manager
        .expect_create_temp_directory()
        .times(1)
        .returning(move || Ok(temp_path.clone()));

    // Expect fetch to be called with specific parameters
    repo_fetcher
        .expect_fetch()
        .with(eq("git@github.com:example/repo.git"), eq("main"), always())
        .times(1)
        .returning(|_, _, _| Ok(()));

    // Expect extract_paths to be called and return success
    repo_fetcher
        .expect_extract_paths()
        .times(1)
        .returning(|_, _, _| Ok(()));

    // Expect clean_directory to be called and return success
    fs_manager
        .expect_clean_directory()
        .times(1)
        .returning(|_| Ok(()));

    // Expect is_git_repository to be called and return true
    git_ops
        .expect_is_git_repository()
        .times(1)
        .returning(|_| Ok(true));

    // Expect stage_all to be called and return success
    git_ops.expect_stage_all().times(1).returning(|_| Ok(()));

    // Expect remove_temp_directory to be called and return success
    fs_manager
        .expect_remove_temp_directory()
        .times(1)
        .returning(|_| Ok(()));

    // Create a dependency
    let dependency = Dependency {
        name: "example-dep".to_string(),
        repository_url: "git@github.com:example/repo.git".to_string(),
        revision: "main".to_string(),
        repository_type: RepositoryType::Git,
        sparse_paths: vec!["docs/**".to_string(), "src/**".to_string()],
        target_location: PathBuf::from("vendor/example"),
    };

    // Create the updater with our mocks
    let updater = DependencyUpdater::new(repo_fetcher, fs_manager, git_ops);

    // Create a test repo root
    let repo_root = tempdir().unwrap();

    // Create vendor/example directory in our test repo
    let target_dir = repo_root.path().join("vendor/example");
    fs::create_dir_all(&target_dir).unwrap();

    // Call the updater
    let result = updater.update(&dependency, repo_root.path());

    // Assert that the update was successful
    assert!(
        result.is_ok(),
        "Expected update to succeed, got: {:?}",
        result
    );
}

#[test]
fn test_real_command_with_temp_dir() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("acdm.toml");

    // Create an initial config file
    fs::write(
        &config_path,
        r#"
[[sources]]
repo = "git@github.com:example/repo.git"
name = "example-dependency"
rev = "main"
type = "git"
sparse_paths = []
target = "vendor/example"
"#,
    )
    .unwrap();

    // Run the include command
    let mut cmd = Command::cargo_bin("acdm").unwrap();
    let output = cmd
        .arg("--config")
        .arg(&config_path)
        .arg("include")
        .arg("example-dependency")
        .arg("docs/**")
        .arg("src/**")
        .output()
        .expect("Failed to run command");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with: {:?}",
        std::str::from_utf8(&output.stderr).unwrap_or("Unknown error")
    );

    // Read the updated config file
    let config_content = fs::read_to_string(&config_path).unwrap();

    // Assert that the paths were added
    assert!(
        config_content.contains("docs/**"),
        "docs/** path not found in config"
    );
    assert!(
        config_content.contains("src/**"),
        "src/** path not found in config"
    );
}
