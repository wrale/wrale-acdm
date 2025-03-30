// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("acdm.toml");

    let mut cmd = Command::cargo_bin("acdm")?;
    cmd.arg("--config").arg(&config_path).arg("init");

    cmd.assert().success();

    // Check that the configuration file was created
    assert!(config_path.exists());

    Ok(())
}
