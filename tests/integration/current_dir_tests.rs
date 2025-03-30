// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
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

    // Restore original directory - important for test environment consistency
    std::env::set_current_dir(&current_dir).unwrap_or_else(|e| {
        panic!("Failed to restore original directory: {}", e);
    });

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Status command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The output should contain "No dependencies found" or similar
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Dependencies:") || stdout.contains("No dependencies"),
        "Unexpected output: {}",
        stdout
    );
}

/// Test that commands with a config file in a different path work correctly
#[test]
fn test_config_file_with_simple_name() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let config_path = temp_path.join("myconfig.toml");

    // Initialize a git repository
    let status = std::process::Command::new("git")
        .args(["init"])
        .current_dir(&temp_path)
        .status()
        .unwrap();
    assert!(status.success());

    // Configure git user
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&temp_path)
        .status()
        .unwrap();

    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&temp_path)
        .status()
        .unwrap();

    // Create an empty config file
    fs::write(&config_path, "").unwrap();

    // Initialize the config file
    let output = Command::cargo_bin("acdm")
        .unwrap()
        .arg("--config")
        .arg(&config_path)
        .arg("--force") // Use force to overwrite existing file
        .arg("init")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Init command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify file exists and has expected content
    assert!(config_path.exists(), "Config file doesn't exist after init");

    // Read the file content to ensure it was initialized
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("sources"),
        "File doesn't contain expected content: {}",
        content
    );

    // Commit the file so git status is clean
    std::process::Command::new("git")
        .args(["add", "myconfig.toml"])
        .current_dir(&temp_path)
        .status()
        .unwrap();

    std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(&temp_path)
        .status()
        .unwrap();

    std::process::Command::new("git")
        .args(["commit", "-m", "Add config file"])
        .current_dir(&temp_path)
        .status()
        .unwrap();

    // Run the status command with the file
    let output = Command::cargo_bin("acdm")
        .unwrap()
        .arg("--config")
        .arg(&config_path)
        .arg("status")
        .output()
        .unwrap();

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Status command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
