// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

// Using raw strings to avoid prefixed identifier errors in tests
macro_rules! raw {
    ($s:expr) => {
        concat!("", $s, "")
    };
}

use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use tempfile::tempdir;

use wrale_acdm::domain::repositories::GitOperations;
use wrale_acdm::infrastructure::git::GitOperationsImpl;

/// This test reproduces the issue where git repository detection fails
/// despite having a valid git repository
#[test]
fn test_git_repository_detection_works_with_new_repo() {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Initialize git repository
    let init_status = StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");
    assert!(init_status.success(), "Git init should succeed");

    // Create a test file and commit it to make sure repo is properly initialized
    fs::write(temp_dir.path().join("test.txt"), "test content").expect("Failed to write test file");

    // Configure git user for the test repository
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");

    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");

    // Disable GPG signing for just this test repository (not globally)
    StdCommand::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to disable GPG signing");

    // Add and commit the file
    StdCommand::new("git")
        .args(["add", "test.txt"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to add test file");

    StdCommand::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to commit test file");

    // Create an acdm.toml file
    fs::write(temp_dir.path().join("acdm.toml"), "sources = []")
        .expect("Failed to write acdm.toml file");

    // Add and commit the file
    StdCommand::new("git")
        .args(["add", "acdm.toml"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to add acdm.toml file");

    StdCommand::new("git")
        .args(["commit", "-m", "Add acdm.toml"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to commit acdm.toml file");

    // Test git repository detection
    let git_ops = GitOperationsImpl::new();

    // Get full path to verify it's correct
    let repo_path = temp_dir.path().to_path_buf();
    println!("Testing git repository at: {}", repo_path.display());

    // Check if it's detected as a git repository
    let is_repo = git_ops
        .is_git_repository(&repo_path)
        .expect("is_git_repository should not error");

    assert!(is_repo, "Directory should be detected as a git repository");

    // Check that status works too
    let status = git_ops
        .get_status(&repo_path)
        .expect("get_status should not error");

    assert!(status.is_clean(), "Repository should be clean");
}

/// Test that git repository detection works with a relative path (no parent directory)
#[test]
fn test_git_repository_detection_with_relative_path() {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Initialize git repository
    let init_status = StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");
    assert!(init_status.success(), "Git init should succeed");

    // Configure git user for the test repository
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");

    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");

    // Disable GPG signing for just this test repository (not globally)
    StdCommand::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to disable GPG signing");

    // Create a config file with just a filename (no parent directory)
    let _config_path = PathBuf::from("acdm.toml");

    // We need to temporarily change into the temp directory to test relative paths
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change directory");

    // Create Git operations
    let git_operations = GitOperationsImpl::new();

    // Instead of calling the private method, test directly with GitOperationsImpl
    let current_dir = std::env::current_dir().unwrap();
    let is_repo = git_operations
        .is_git_repository(&current_dir)
        .expect("is_git_repository should not error");
    assert!(
        is_repo,
        "Current directory should be detected as a git repository"
    );

    // Also check that status works
    let status = git_operations
        .get_status(&current_dir)
        .expect("get_status should not error");
    assert!(status.is_clean(), "Repository should be clean");

    // Change back to the original directory
    std::env::set_current_dir(original_dir).expect("Failed to restore original directory");
}

/// Test that CLI commands work with a config file in the current directory
#[test]
fn test_cli_with_config_in_current_directory() {
    use assert_cmd::Command;

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Initialize git repository
    StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");

    // Configure git user for the test repository
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");

    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");

    // Disable GPG signing for just this test repository (not globally)
    StdCommand::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to disable GPG signing");

    // Save the original directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");

    // Change to the temp directory to test with relative paths
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change directory");

    // Run the init command with the default config name (acdm.toml in current directory)
    let init_output = Command::cargo_bin("acdm")
        .expect("Failed to find binary")
        .arg("init")
        .output()
        .expect("Failed to run acdm init");

    // Verify the command succeeded
    assert!(
        init_output.status.success(),
        "Init command failed with: {:?}",
        String::from_utf8_lossy(&init_output.stderr)
    );

    // Verify the file was created
    assert!(
        std::path::Path::new("acdm.toml").exists(),
        "acdm.toml should exist"
    );

    // Add a dependency to test the update command
    let content = r#"[[sources]]
repo = "git@github.com:example/repo.git"
name = "test-dep"
rev = "main"
type = "git"
sparse_paths = []
target = "vendor/test"
"#;
    fs::write("acdm.toml", content).expect("Failed to write acdm.toml");

    // Commit the config file to get a clean git state
    StdCommand::new("git")
        .args(["add", "acdm.toml"])
        .status()
        .expect("Failed to add acdm.toml");

    StdCommand::new("git")
        .args(["commit", "-m", "Add acdm.toml"])
        .status()
        .expect("Failed to commit acdm.toml");

    // Now run the status command with the default config path
    let status_output = Command::cargo_bin("acdm")
        .expect("Failed to find binary")
        .arg("status")
        .output()
        .expect("Failed to run acdm status");

    // The command should succeed - IMPORTANT: Check this BEFORE changing directory
    assert!(
        status_output.status.success(),
        "Status command failed with: {:?}",
        String::from_utf8_lossy(&status_output.stderr)
    );

    // The output should contain the dependency status - IMPORTANT: Check this BEFORE changing directory
    let stdout = String::from_utf8_lossy(&status_output.stdout);
    assert!(
        stdout.contains("test-dep"),
        "Output missing dependency: {}",
        stdout
    );

    // Change back to the original directory AFTER we've checked everything we need from the temp directory
    std::env::set_current_dir(original_dir).expect("Failed to restore original directory");
}

/// This test reproduces the exact scenario from the bug report
#[test]
fn test_reproduce_bug_report_scenario() {
    use assert_cmd::Command;

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("acdm.toml");

    // Initialize git repository (like in the bug report)
    let init_status = StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to run git init");
    assert!(init_status.success(), "Git init failed");

    // Create acdm.toml with content similar to bug report
    let acdm_content = r#"[[sources]]
repo = "git@github.com:modelcontextprotocol/specification.git"
name = "mcp-specification"
rev = "main"
type = "git"
sparse_paths = [
    "docs/specification/2025-03-26/**",
    "schema/2025-03-26/**",
]
target = "vendor/mcp-specification"
"#;

    // Write the config file
    fs::write(&config_path, acdm_content).expect("Failed to write file");

    // Run status command - this should NOT fail with our fix
    let output = Command::cargo_bin("acdm")
        .expect("Failed to find binary")
        .arg("--config")
        .arg(&config_path)
        .arg("status")
        .output()
        .expect("Failed to run acdm");

    // The command should succeed
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The output should contain the dependency status
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mcp"),
        "Output missing dependency: {}",
        stdout
    );
}

/// This test specifically focuses on the parent directory issue mentioned in the bug report
#[test]
fn test_git_repository_parent_directory_detection() {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Initialize git repository
    let init_status = StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");
    assert!(init_status.success(), "Git init should succeed");

    // Configure git user for the test repository
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");

    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");

    // Create a subdirectory
    let subdir_path = temp_dir.path().join("subdir");
    fs::create_dir(&subdir_path).expect("Failed to create subdirectory");

    // Create acdm.toml in subdirectory
    fs::write(subdir_path.join("acdm.toml"), "sources = []")
        .expect("Failed to write acdm.toml file");

    // Test git repository detection from subdirectory
    let git_ops = GitOperationsImpl::new();

    // Check if subdirectory is detected as part of a git repository
    let is_repo = git_ops
        .is_git_repository(&subdir_path)
        .expect("is_git_repository should not error");

    assert!(
        is_repo,
        "Subdirectory should be detected as part of a git repository"
    );
}
