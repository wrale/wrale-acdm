// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::Path;
use std::process::Command;
use glob::Pattern;
use walkdir::WalkDir;

use crate::domain::DomainError;
use crate::domain::repositories::{RepositoryFetcher, GitOperations, FileSystemManager};

pub struct GitOperationsImpl;

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
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .current_dir(path)
            .output()
            .map_err(|e| DomainError::GitError(format!("Failed to check if path is a git repository: {}", e)))?;
            
        Ok(output.status.success())
    }
}

pub struct GitRepositoryFetcher {
    git_command_path: String,
}

impl GitRepositoryFetcher {
    pub fn new() -> Self {
        Self {
            git_command_path: "git".to_string(),
        }
    }
    
    pub fn with_git_path(git_path: &str) -> Self {
        Self {
            git_command_path: git_path.to_string(),
        }
    }
}

impl RepositoryFetcher for GitRepositoryFetcher {
    fn fetch(&self, url: &str, revision: &str, temp_path: &Path) -> Result<(), DomainError> {
        // Clone the repository
        let output = Command::new(&self.git_command_path)
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("--branch")
            .arg(revision)
            .arg(url)
            .arg(temp_path)
            .output()
            .map_err(|e| DomainError::GitError(format!("Failed to clone repository: {}", e)))?;
            
        if !output.status.success() {
            return Err(DomainError::GitError(format!(
                "Git clone command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        
        Ok(())
    }
    
    fn extract_paths(&self, repo_path: &Path, patterns: &[String], target_path: &Path) -> Result<(), DomainError> {
        if patterns.is_empty() {
            // If no patterns are specified, copy everything
            let fs_manager = crate::infrastructure::file_system::FileSystemManagerImpl::new();
            return fs_manager.copy_content(repo_path, target_path);
        }
        
        // Compile all patterns
        let compiled_patterns: Vec<Pattern> = patterns
            .iter()
            .map(|p| Pattern::new(p).map_err(|e| DomainError::PathPatternError(
                format!("Invalid pattern '{}': {}", p, e)
            )))
            .collect::<Result<_, _>>()?;
            
        // Walk the repository and copy matching files
        let mut copied_any = false;
        
        for entry in WalkDir::new(repo_path).min_depth(1) {
            let entry = entry.map_err(|e| DomainError::FileSystemError(
                format!("Failed to walk directory: {}", e)
            ))?;
            
            // Get the path relative to the repository root
            let relative_path = entry.path().strip_prefix(repo_path).map_err(|e| DomainError::FileSystemError(
                format!("Failed to strip prefix: {}", e)
            ))?;
            
            // Check if the path matches any pattern
            let relative_path_str = relative_path.to_string_lossy();
            let should_include = compiled_patterns.iter().any(|p| p.matches(&relative_path_str));
            
            if should_include {
                let target_file_path = target_path.join(relative_path);
                
                if entry.path().is_dir() {
                    std::fs::create_dir_all(&target_file_path).map_err(|e| DomainError::FileSystemError(
                        format!("Failed to create directory: {}", e)
                    ))?;
                } else {
                    // Ensure parent directories exist
                    if let Some(parent) = target_file_path.parent() {
                        std::fs::create_dir_all(parent).map_err(|e| DomainError::FileSystemError(
                            format!("Failed to create parent directory: {}", e)
                        ))?;
                    }
                    
                    std::fs::copy(entry.path(), &target_file_path).map_err(|e| DomainError::FileSystemError(
                        format!("Failed to copy file: {}", e)
                    ))?;
                    
                    copied_any = true;
                }
            }
        }
        
        if !copied_any && !patterns.is_empty() {
            return Err(DomainError::PathPatternError(
                format!("No files matched the provided patterns: {:?}", patterns)
            ));
        }
        
        Ok(())
    }
}
