// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use tempfile::TempDir;

use crate::domain::DomainError;
use crate::domain::repositories::FileSystemManager;

pub struct FileSystemManagerImpl {
    temp_dirs: std::sync::Mutex<Vec<TempDir>>,
}

impl FileSystemManagerImpl {
    pub fn new() -> Self {
        Self {
            temp_dirs: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl FileSystemManager for FileSystemManagerImpl {
    fn clean_directory(&self, path: &Path) -> Result<(), DomainError> {
        if !path.exists() {
            return Ok(());
        }
        
        if !path.is_dir() {
            return Err(DomainError::FileSystemError(
                format!("Path is not a directory: {}", path.display())
            ));
        }
        
        // Remove all contents of the directory
        for entry in fs::read_dir(path).map_err(|e| DomainError::FileSystemError(
            format!("Failed to read directory: {}", e)
        ))? {
            let entry = entry.map_err(|e| DomainError::FileSystemError(
                format!("Failed to read directory entry: {}", e)
            ))?;
            
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                fs::remove_dir_all(&entry_path).map_err(|e| DomainError::FileSystemError(
                    format!("Failed to remove directory: {}", e)
                ))?;
            } else {
                fs::remove_file(&entry_path).map_err(|e| DomainError::FileSystemError(
                    format!("Failed to remove file: {}", e)
                ))?;
            }
        }
        
        Ok(())
    }
    
    fn copy_content(&self, source: &Path, destination: &Path) -> Result<(), DomainError> {
        if !source.exists() {
            return Err(DomainError::FileSystemError(
                format!("Source path does not exist: {}", source.display())
            ));
        }
        
        if !source.is_dir() {
            return Err(DomainError::FileSystemError(
                format!("Source path is not a directory: {}", source.display())
            ));
        }
        
        // Create the destination directory if it doesn't exist
        if !destination.exists() {
            fs::create_dir_all(destination).map_err(|e| DomainError::FileSystemError(
                format!("Failed to create destination directory: {}", e)
            ))?;
        }
        
        // Copy all contents from source to destination
        for entry in WalkDir::new(source).min_depth(1) {
            let entry = entry.map_err(|e| DomainError::FileSystemError(
                format!("Failed to walk directory: {}", e)
            ))?;
            
            let relative_path = entry.path().strip_prefix(source).map_err(|e| DomainError::FileSystemError(
                format!("Failed to strip prefix: {}", e)
            ))?;
            
            let target_path = destination.join(relative_path);
            
            if entry.path().is_dir() {
                fs::create_dir_all(&target_path).map_err(|e| DomainError::FileSystemError(
                    format!("Failed to create directory: {}", e)
                ))?;
            } else {
                // Create parent directories if they don't exist
                if let Some(parent) = target_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| DomainError::FileSystemError(
                            format!("Failed to create parent directory: {}", e)
                        ))?;
                    }
                }
                
                fs::copy(entry.path(), &target_path).map_err(|e| DomainError::FileSystemError(
                    format!("Failed to copy file: {}", e)
                ))?;
            }
        }
        
        Ok(())
    }
    
    fn create_temp_directory(&self) -> Result<PathBuf, DomainError> {
        let temp_dir = tempfile::tempdir().map_err(|e| DomainError::FileSystemError(
            format!("Failed to create temporary directory: {}", e)
        ))?;
        
        let path = temp_dir.path().to_path_buf();
        
        // Store the TempDir so it doesn't get dropped and deleted
        let mut temp_dirs = self.temp_dirs.lock().unwrap();
        temp_dirs.push(temp_dir);
        
        Ok(path)
    }
    
    fn remove_temp_directory(&self, path: &Path) -> Result<(), DomainError> {
        let mut temp_dirs = self.temp_dirs.lock().unwrap();
        
        // Find the TempDir that corresponds to the path
        let index = temp_dirs.iter().position(|td| td.path() == path);
        
        if let Some(idx) = index {
            // Remove the TempDir from the list (this will drop it and delete the directory)
            temp_dirs.remove(idx);
            Ok(())
        } else {
            // If the path isn't in our list, try to remove it manually
            if path.exists() {
                fs::remove_dir_all(path).map_err(|e| DomainError::FileSystemError(
                    format!("Failed to remove temporary directory: {}", e)
                ))?;
            }
            
            Ok(())
        }
    }
}
