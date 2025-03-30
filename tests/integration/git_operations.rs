// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use std::process::Command;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

use wrale_acdm::infrastructure::git::GitOperationsImpl;
use wrale_acdm::domain::repositories::GitOperations;

// Helper function to initialize a git repository
fn init_git_repo(path: &Path) {
    Command::new("git")
        .args(&["init"])
        .current_dir(path)
        .output()
        .expect("Failed to initialize git repository");
        
    // Configure git user for the test repository
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git user name");
        
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git user email");
}

// Helper function to make a commit
fn commit_file(repo_path: &Path, filename: &str, content: &str, message: &str) {
    // Create and write to file
    let file_path = repo_path.join(filename);
    let mut file = File::create(&file_path).expect("Failed to create file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
    
    // Stage and commit
    Command::new("git")
        .args(&["add", filename])
        .current_dir(repo_path)
        .output()
        .expect("Failed to stage file");
        
    Command::new("git")
        .args(&["commit", "-m", message])
        .current_dir(repo_path)
        .output()
        .expect("Failed to commit file");
}

#[test]
fn test_git_operations_status() {
    // Create a temporary directory for the test repository
    let repo_dir = tempdir().expect("Failed to create temporary directory");
    
    // Initialize git repository
    init_git_repo(repo_dir.path());
    
    // Create the GitOperationsImpl instance
    let git_ops = GitOperationsImpl::new();
    
    // Check that the repository is detected
    let is_repo = git_ops.is_git_repository(repo_dir.path()).expect("Failed to check repository status");
    assert!(is_repo, "Expected path to be a git repository");
    
    // Make an initial commit
    commit_file(repo_dir.path(), "file1.txt", "Initial content", "Initial commit");
    
    // Check the status after a clean commit
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.is_clean(), "Expected clean repository status after commit");
    
    // Create an unstaged file
    fs::write(repo_dir.path().join("file2.txt"), "Unstaged content").expect("Failed to create unstaged file");
    
    // Check the status with unstaged changes
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.has_untracked_files, "Expected status to show untracked files");
    assert!(!status.is_clean(), "Expected unclean repository status with untracked files");
    
    // Stage the file
    Command::new("git")
        .args(&["add", "file2.txt"])
        .current_dir(repo_dir.path())
        .output()
        .expect("Failed to stage file");
        
    // Check the status with staged changes
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.has_staged_changes, "Expected status to show staged changes");
    assert!(!status.is_clean(), "Expected unclean repository status with staged changes");
    
    // Modify a tracked file without staging
    fs::write(repo_dir.path().join("file1.txt"), "Modified content").expect("Failed to modify file");
    
    // Check the status with unstaged modifications
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.has_unstaged_changes, "Expected status to show unstaged changes");
    assert!(!status.is_clean(), "Expected unclean repository status with unstaged changes");
}

#[test]
fn test_git_stage_and_commit() {
    // Create a temporary directory for the test repository
    let repo_dir = tempdir().expect("Failed to create temporary directory");
    
    // Initialize git repository
    init_git_repo(repo_dir.path());
    
    // Create the GitOperationsImpl instance
    let git_ops = GitOperationsImpl::new();
    
    // Create a file
    fs::write(repo_dir.path().join("test-file.txt"), "Test content").expect("Failed to create file");
    
    // Stage all changes
    git_ops.stage_all(repo_dir.path()).expect("Failed to stage changes");
    
    // Check that changes are staged
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.has_staged_changes, "Expected status to show staged changes");
    
    // Commit the changes
    git_ops.commit(repo_dir.path(), "Test commit").expect("Failed to commit changes");
    
    // Check that the repository is clean after commit
    let status = git_ops.get_status(repo_dir.path()).expect("Failed to get status");
    assert!(status.is_clean(), "Expected clean repository status after commit");
}
