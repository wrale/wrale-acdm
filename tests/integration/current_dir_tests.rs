// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::fs;
use std::path::PathBuf;
use assert_cmd::Command;
use tempfile::tempdir;

/// Test that git repository detection works with a config file in the current directory
#[test]
fn test_config_file_in_current_directory() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Initialize a git repository in the temp directory
    let status = std::process::Command::new("git")
        .args(["init"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    assert!(status.success());
    
    // Configure git user
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_path)
        .status()
        .unwrap();
        
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    // Create acdm.toml in the root
    let config_content = r#"sources = []"#;
    fs::write(temp_path.join("acdm.toml"), config_content).unwrap();
    
    // Commit the file so git status is clean
    std::process::Command::new("git")
        .args(["add", "acdm.toml"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_path)
        .status()
        .unwrap();
        
    std::process::Command::new("git")
        .args(["commit", "-m", "Add config file"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    // Save current directory
    let current_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_path).unwrap();
    
    // Test that status command works with a simple config file name (no path)
    // This would previously fail because the parent of "acdm.toml" is empty
    let output = Command::cargo_bin("acdm")
        .unwrap()
        .arg("status")
        .output()
        .unwrap();
        
    // Restore original directory
    std::env::set_current_dir(current_dir).unwrap();
    
    // Check that the command succeeded
    assert!(output.status.success(), 
            "Status command failed: {}",
            String::from_utf8_lossy(&output.stderr));
    
    // The output should contain "No dependencies found" or similar
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dependencies:") || stdout.contains("No dependencies"), 
            "Unexpected output: {}", stdout);
}

/// Test that commands with a config file argument in the current directory work correctly
#[test]
fn test_config_file_with_simple_name() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Initialize a git repository
    let status = std::process::Command::new("git")
        .args(["init"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    assert!(status.success());
    
    // Configure git user
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_path)
        .status()
        .unwrap();
        
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    // Save current directory
    let current_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_path).unwrap();
    
    // First initialize a config file
    let output = Command::cargo_bin("acdm")
        .unwrap()
        .arg("--config")
        .arg("myconfig.toml")
        .arg("init")
        .output()
        .unwrap();
        
    assert!(output.status.success(), 
            "Init command failed: {}", 
            String::from_utf8_lossy(&output.stderr));
            
    // Verify file exists
    assert!(PathBuf::from("myconfig.toml").exists());
    
    // Commit the file so git status is clean
    std::process::Command::new("git")
        .args(["add", "myconfig.toml"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_path)
        .status()
        .unwrap();
        
    std::process::Command::new("git")
        .args(["commit", "-m", "Add config file"])
        .current_dir(temp_path)
        .status()
        .unwrap();
    
    // Run the status command with the file
    let output = Command::cargo_bin("acdm")
        .unwrap()
        .arg("--config")
        .arg("myconfig.toml")
        .arg("status")
        .output()
        .unwrap();
        
    // Restore original directory
    std::env::set_current_dir(current_dir).unwrap();
    
    // Check that the command succeeded
    assert!(output.status.success(), 
            "Status command failed: {}",
            String::from_utf8_lossy(&output.stderr));
}
