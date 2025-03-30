// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use glob::Pattern;
use log::{debug, error, info, warn};
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

use crate::domain::repositories::{FileSystemManager, GitOperations, RepositoryFetcher};
use crate::domain::DomainError;

pub struct GitOperationsImpl;

impl Default for GitOperationsImpl {
    fn default() -> Self {
        Self
    }
}

impl GitOperationsImpl {
    pub fn new() -> Self {
        Self
    }
}

impl GitOperations for GitOperationsImpl {
    fn stage_all(&self, repo_path: &Path) -> Result<(), DomainError> {
        let output = Command::new("git")
            .arg("add")
            .arg("--all")
            .current_dir(repo_path)
            .output()
            .map_err(|e| DomainError::GitError(format!("Failed to stage changes: {}", e)))?;

        if !output.status.success() {
            return Err(DomainError::GitError(format!(
                "Git add command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn commit(&self, repo_path: &Path, message: &str) -> Result<(), DomainError> {
        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(repo_path)
            .output()
            .map_err(|e| DomainError::GitError(format!("Failed to commit changes: {}", e)))?;

        if !output.status.success() {
            // Check if there were no changes to commit
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stderr.contains("nothing to commit") || stdout.contains("nothing to commit") {
                // This is not an error, just nothing to commit
                return Ok(());
            }

            return Err(DomainError::GitError(format!(
                "Git commit command failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    fn is_git_repository(&self, path: &Path) -> Result<bool, DomainError> {
        debug!("Checking if path is a git repository: {}", path.display());
        
        // First, verify the path exists to avoid unhelpful errors
        if !path.exists() {
            warn!("Path does not exist: {}", path.display());
            return Err(DomainError::GitError(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }
        
        // Execute the git command with better error handling
        let result = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .current_dir(path)
            .output();
            
        // Handle the command execution result
        match result {
            Ok(output) => {
                if output.status.success() {
                    debug!("Path is a git repository: {}", path.display());
                    Ok(true)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    debug!("Path is not a git repository: {} ({})", path.display(), stderr.trim());
                    Ok(false)
                }
            }
            Err(e) => {
                warn!("Error executing git command: {}", e);
                
                // Check if the error is because git is not installed
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Err(DomainError::GitError(
                        "Git executable not found. Please ensure git is installed and in your PATH.".to_string()
                    ));
                }
                
                Err(DomainError::GitError(format!(
                    "Failed to check if path is a git repository: {}",
                    e
                )))
            }
        }
    }

    fn get_status(
        &self,
        repo_path: &Path,
    ) -> Result<crate::domain::repositories::GitStatus, DomainError> {
        use crate::domain::repositories::GitStatus;

        // Check if there are staged changes
        let staged_output = Command::new("git")
            .args(["diff", "--cached", "--quiet"])
            .current_dir(repo_path)
            .status()
            .map_err(|e| DomainError::GitError(format!("Failed to check staged changes: {}", e)))?;

        let has_staged_changes = !staged_output.success();

        // Check if there are unstaged changes
        let unstaged_output = Command::new("git")
            .args(["diff", "--quiet"])
            .current_dir(repo_path)
            .status()
            .map_err(|e| {
                DomainError::GitError(format!("Failed to check unstaged changes: {}", e))
            })?;

        let has_unstaged_changes = !unstaged_output.success();

        // Check if there are untracked files
        let untracked_output = Command::new("git")
            .args([
                "ls-files",
                "--other",
                "--exclude-standard",
                "--directory",
                "--no-empty-directory",
            ])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                DomainError::GitError(format!("Failed to check untracked files: {}", e))
            })?;

        let has_untracked_files = !String::from_utf8_lossy(&untracked_output.stdout)
            .trim()
            .is_empty();

        Ok(GitStatus {
            has_staged_changes,
            has_unstaged_changes,
            has_untracked_files,
        })
    }
}

pub struct GitRepositoryFetcher {
    git_command_path: String,
}

impl Default for GitRepositoryFetcher {
    fn default() -> Self {
        Self {
            git_command_path: "git".to_string(),
        }
    }
}

impl GitRepositoryFetcher {
    pub fn new() -> Self {
        Self::default()
    }

    // Function removed as it was unused
}

impl RepositoryFetcher for GitRepositoryFetcher {
    fn fetch(&self, url: &str, revision: &str, temp_path: &Path) -> Result<(), DomainError> {
        debug!(
            "Fetching repository: {} revision: {} to {}",
            url,
            revision,
            temp_path.display()
        );

        // Clone the repository
        let clone_args = [
            "clone",
            "--depth",
            "1",
            "--branch",
            revision,
            url,
            &temp_path.to_string_lossy(),
        ];

        debug!(
            "Running git command: {} {}",
            self.git_command_path,
            clone_args.join(" ")
        );

        let output = Command::new(&self.git_command_path)
            .args(clone_args)
            .output()
            .map_err(|e| {
                error!("Failed to execute git clone: {}", e);
                DomainError::GitError(format!("Failed to clone repository: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Git clone failed: {}", stderr);

            // Try fallback to direct commit if branch/tag fails
            if stderr.contains("Remote branch") && stderr.contains("not found") {
                info!("Branch not found, trying to clone and checkout specific commit");

                // Clone without branch specification
                let base_clone_args = ["clone", "--no-checkout", url, &temp_path.to_string_lossy()];

                debug!(
                    "Running fallback git command: {} {}",
                    self.git_command_path,
                    base_clone_args.join(" ")
                );

                let base_output = Command::new(&self.git_command_path)
                    .args(base_clone_args)
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute fallback git clone: {}", e);
                        DomainError::GitError(format!("Failed to clone repository: {}", e))
                    })?;

                if !base_output.status.success() {
                    let base_stderr = String::from_utf8_lossy(&base_output.stderr);
                    error!("Fallback git clone failed: {}", base_stderr);
                    return Err(DomainError::GitError(format!(
                        "Git clone command failed: {}",
                        base_stderr
                    )));
                }

                // Try to checkout the specific revision
                let checkout_args = ["checkout", revision];

                debug!(
                    "Running git checkout command: {} {}",
                    self.git_command_path,
                    checkout_args.join(" ")
                );

                let checkout_output = Command::new(&self.git_command_path)
                    .args(checkout_args)
                    .current_dir(temp_path)
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute git checkout: {}", e);
                        DomainError::GitError(format!("Failed to checkout revision: {}", e))
                    })?;

                if !checkout_output.status.success() {
                    let checkout_stderr = String::from_utf8_lossy(&checkout_output.stderr);
                    error!("Git checkout failed: {}", checkout_stderr);
                    return Err(DomainError::GitError(format!(
                        "Git checkout command failed: {}",
                        checkout_stderr
                    )));
                }

                info!(
                    "Successfully cloned and checked out revision {} using fallback method",
                    revision
                );
            } else {
                return Err(DomainError::GitError(format!(
                    "Git clone command failed: {}",
                    stderr
                )));
            }
        } else {
            debug!("Repository cloned successfully");
        }

        Ok(())
    }

    fn extract_paths(
        &self,
        repo_path: &Path,
        patterns: &[String],
        target_path: &Path,
    ) -> Result<(), DomainError> {
        debug!(
            "Extracting paths from {} to {}",
            repo_path.display(),
            target_path.display()
        );
        debug!("Patterns: {:?}", patterns);

        if patterns.is_empty() {
            debug!("No patterns specified, copying everything");
            // If no patterns are specified, copy everything
            let fs_manager = crate::infrastructure::file_system::FileSystemManagerImpl::new();
            return fs_manager.copy_content(repo_path, target_path);
        }

        // Compile all patterns
        debug!("Compiling {} patterns", patterns.len());
        let compiled_patterns: Vec<Pattern> = patterns
            .iter()
            .map(|p| {
                debug!("Compiling pattern: {}", p);
                Pattern::new(p).map_err(|e| {
                    error!("Invalid pattern '{}': {}", p, e);
                    DomainError::PathPatternError(format!("Invalid pattern '{}': {}", p, e))
                })
            })
            .collect::<Result<_, _>>()?;

        // Walk the repository and copy matching files
        let mut copied_any = false;
        let mut matched_count = 0;

        debug!("Walking repository for matching files");
        for entry in WalkDir::new(repo_path).min_depth(1) {
            let entry = entry.map_err(|e| {
                error!("Failed to walk directory {}: {}", repo_path.display(), e);
                DomainError::FileSystemError(format!("Failed to walk directory: {}", e))
            })?;

            // Get the path relative to the repository root
            let relative_path = entry.path().strip_prefix(repo_path).map_err(|e| {
                error!("Failed to strip prefix {}: {}", repo_path.display(), e);
                DomainError::FileSystemError(format!("Failed to strip prefix: {}", e))
            })?;

            // Check if the path matches any pattern
            let relative_path_str = relative_path.to_string_lossy();
            let should_include = compiled_patterns
                .iter()
                .any(|p| p.matches(&relative_path_str));

            if should_include {
                matched_count += 1;
                debug!("Path matched pattern: {}", relative_path_str);
                let target_file_path = target_path.join(relative_path);

                if entry.path().is_dir() {
                    debug!("Creating directory: {}", target_file_path.display());
                    std::fs::create_dir_all(&target_file_path).map_err(|e| {
                        error!(
                            "Failed to create directory {}: {}",
                            target_file_path.display(),
                            e
                        );
                        DomainError::FileSystemError(format!("Failed to create directory: {}", e))
                    })?;
                } else {
                    // Ensure parent directories exist
                    if let Some(parent) = target_file_path.parent() {
                        if !parent.exists() {
                            debug!("Creating parent directory: {}", parent.display());
                            std::fs::create_dir_all(parent).map_err(|e| {
                                error!(
                                    "Failed to create parent directory {}: {}",
                                    parent.display(),
                                    e
                                );
                                DomainError::FileSystemError(format!(
                                    "Failed to create parent directory: {}",
                                    e
                                ))
                            })?;
                        }
                    }

                    debug!(
                        "Copying file: {} to {}",
                        entry.path().display(),
                        target_file_path.display()
                    );
                    std::fs::copy(entry.path(), &target_file_path).map_err(|e| {
                        error!(
                            "Failed to copy file from {} to {}: {}",
                            entry.path().display(),
                            target_file_path.display(),
                            e
                        );
                        DomainError::FileSystemError(format!("Failed to copy file: {}", e))
                    })?;

                    copied_any = true;
                }
            }
        }

        debug!(
            "Matched {} paths, copied files: {}",
            matched_count, copied_any
        );

        if !copied_any && !patterns.is_empty() {
            warn!("No files matched the provided patterns: {:?}", patterns);
            return Err(DomainError::PathPatternError(format!(
                "No files matched the provided patterns: {:?}",
                patterns
            )));
        }

        debug!("Path extraction completed successfully");
        Ok(())
    }
}
