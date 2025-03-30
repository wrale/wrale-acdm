// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use std::process::Command as StdCommand;
use std::fs;
use tempfile::tempdir;

use wrale_acdm::infrastructure::git::GitOperationsImpl;
use wrale_acdm::domain::repositories::GitOperations;

/// This test reproduces the issue where git repository detection fails
/// despite having a valid git repository
#[test]
fn test_git_repository_detection_works_with_new_repo() {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    
    // Initialize git repository
    let init_status = StdCommand::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");
    assert!(init_status.success(), "Git init should succeed");
    
    // Create a test file and commit it to make sure repo is properly initialized
    fs::write(temp_dir.path().join("test.txt"), "test content").expect("Failed to write test file");
    
    // Configure git user for the test repository
    StdCommand::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");
        
    StdCommand::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");
    
    // Add and commit the file
    StdCommand::new("git")
        .args(&["add", "test.txt"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to add test file");
        
    StdCommand::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to commit test file");
    
    // Create an acdm.toml file
    fs::write(temp_dir.path().join("acdm.toml"), "sources = []").expect("Failed to write acdm.toml file");
    
    // Add and commit the file
    StdCommand::new("git")
        .args(&["add", "acdm.toml"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to add acdm.toml file");
        
    StdCommand::new("git")
        .args(&["commit", "-m", "Add acdm.toml"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to commit acdm.toml file");
    
    // Test git repository detection
    let git_ops = GitOperationsImpl::new();
    
    // Get full path to verify it's correct
    let repo_path = temp_dir.path().to_path_buf();
    println!("Testing git repository at: {}

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
    assert!(output.status.success(), 
        "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
        
    // The output should contain the dependency status
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mcp"), 
        "Output missing dependency: {}", stdout);
}", repo_path.display());
    
    // Check if it's detected as a git repository
    let is_repo = git_ops.is_git_repository(&repo_path)
        .expect("is_git_repository should not error");
    
    assert!(is_repo, "Directory should be detected as a git repository");
    
    // Check that status works too
    let status = git_ops.get_status(&repo_path)
        .expect("get_status should not error");
    
    assert!(status.is_clean(), "Repository should be clean");
}

/// This test specifically focuses on the parent directory issue mentioned in the bug report
#[test]
fn test_git_repository_parent_directory_detection() {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    
    // Initialize git repository
    let init_status = StdCommand::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to initialize git repository");
    assert!(init_status.success(), "Git init should succeed");
    
    // Configure git user for the test repository
    StdCommand::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user name");
        
    StdCommand::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to configure git user email");
    
    // Create a subdirectory
    let subdir_path = temp_dir.path().join("subdir");
    fs::create_dir(&subdir_path).expect("Failed to create subdirectory");
    
    // Create acdm.toml in subdirectory
    fs::write(subdir_path.join("acdm.toml"), "sources = []").expect("Failed to write acdm.toml file");
    
    // Test git repository detection from subdirectory
    let git_ops = GitOperationsImpl::new();
    
    // Check if subdirectory is detected as part of a git repository
    let is_repo = git_ops.is_git_repository(&subdir_path)
        .expect("is_git_repository should not error");
    
    assert!(is_repo, "Subdirectory should be detected as part of a git repository");
}
