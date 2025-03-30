// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::fs;
use std::process::Command;
use tempfile::tempdir;

/// Test that the --skip-commit flag prevents automatic commits
#[test]
fn test_skip_commit_flag() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("acdm.toml");

    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()?;

    // Set up git config for the test repository
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()?;

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()?;

    // Disable GPG signing for just this test repository (not globally)
    Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_dir.path())
        .output()?;

    // Create an initial config file with a dependency already configured
    let config_content = r#"[[sources]]
repo = "git@github.com:example/repo.git"
name = "test-dep"
rev = "main"
type = "git"
sparse_paths = []
target = "vendor/test"
"#;

    fs::write(&config_path, config_content)?;

    // Commit the initial config file
    Command::new("git")
        .args(["add", "acdm.toml"])
        .current_dir(temp_dir.path())
        .output()?;

    Command::new("git")
        .args(["commit", "-m", "Initial commit with config"])
        .current_dir(temp_dir.path())
        .output()?;

    // Now add a path with --skip-commit flag
    let mut cmd = assert_cmd::Command::cargo_bin("acdm")?;
    cmd.current_dir(temp_dir.path())
        .arg("--config")
        .arg(&config_path)
        .arg("include")
        .arg("test-dep")
        .arg("docs/**")
        .arg("--skip-commit")
        .arg("--force") // Add force flag to bypass git status check
        .assert()
        .success();

    // Verify that the file was changed but not committed
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(temp_dir.path())
        .output()?;

    let status_text = String::from_utf8(status_output.stdout)?;
    assert!(
        !status_text.is_empty(),
        "There should be uncommitted changes with --skip-commit"
    );
    assert!(
        status_text.contains("M acdm.toml"),
        "acdm.toml should be modified but not committed"
    );

    // Now add another path without the --skip-commit flag to test auto-commit

    // First commit the pending changes to get a clean state
    Command::new("git")
        .args(["add", "acdm.toml"])
        .current_dir(temp_dir.path())
        .output()?;

    Command::new("git")
        .args(["commit", "-m", "Manually commit the first change"])
        .current_dir(temp_dir.path())
        .output()?;

    // Now add another path without --skip-commit
    let mut cmd = assert_cmd::Command::cargo_bin("acdm")?;
    cmd.current_dir(temp_dir.path())
        .arg("--config")
        .arg(&config_path)
        .arg("include")
        .arg("test-dep")
        .arg("src/**")
        .arg("--force") // Add force flag to bypass git status check
        .assert()
        .success();

    // Verify that the changes were automatically committed
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(temp_dir.path())
        .output()?;

    let status_text = String::from_utf8(status_output.stdout)?;
    assert!(
        status_text.is_empty(),
        "There should be no uncommitted changes (auto-commit should have happened)"
    );

    // Check the commit message
    let log_output = Command::new("git")
        .args(["log", "-1", "--pretty=%s"])
        .current_dir(temp_dir.path())
        .output()?;

    let commit_msg = String::from_utf8(log_output.stdout)?;
    assert!(
        commit_msg.contains("Include") && commit_msg.contains("test-dep"),
        "Commit message should contain 'Include' and the dependency name"
    );

    Ok(())
}
