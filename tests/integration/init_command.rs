// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

/// Test that init does not overwrite existing configuration without --force
#[test]
fn test_init_respects_existing_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("acdm.toml");
    
    // Create an existing configuration
    let existing_content = r#"[[sources]]
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
    
    fs::write(&config_path, existing_content)?;
    
    // Attempt to initialize without force flag
    let mut cmd = Command::cargo_bin("acdm")?;
    let assert = cmd
        .arg("--config")
        .arg(&config_path)
        .arg("init")
        .assert();
        
    // Should fail because file exists and no --force flag
    assert.failure();
    
    // Verify the content wasn't changed
    let content_after = fs::read_to_string(&config_path)?;
    assert_eq!(content_after, existing_content, "Config file should not have been modified");
    
    // Now try with --force flag
    let mut cmd = Command::cargo_bin("acdm")?;
    cmd.arg("--config")
        .arg(&config_path)
        .arg("--force")
        .arg("init")
        .assert()
        .success();
        
    // Verify the content was overwritten
    let content_after_force = fs::read_to_string(&config_path)?;
    assert_ne!(content_after_force, existing_content, "Config file should have been overwritten with --force");
    assert!(content_after_force.contains("sources = []"), "New config should contain empty sources");
    
    Ok(())
}
