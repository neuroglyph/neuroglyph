// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! File system operations trait for dependency injection

use crate::error::Result;
use std::path::Path;

/// Trait for file system operations to enable dependency injection and testing
pub trait FileSystem: Send + Sync + Clone {
    /// Check if a path exists
    fn exists(&self, path: &Path) -> bool;

    /// Read a file to string
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Write string content to a file
    fn write(&self, path: &Path, content: &str) -> Result<()>;

    /// Create a directory and all parent directories
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Remove a file
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// Read directory entries
    fn read_dir(&self, path: &Path) -> Result<Vec<std::path::PathBuf>>;
}

/// Default implementation using std::fs
#[derive(Clone)]
pub struct StdFileSystem;

impl FileSystem for StdFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(Into::into)
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content).map_err(Into::into)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path).map_err(Into::into)
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        std::fs::remove_file(path).map_err(Into::into)
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<std::path::PathBuf>> {
        let entries = std::fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();
        Ok(entries)
    }
}

#[cfg(test)]
pub mod test_doubles {
    use super::*;
    use std::collections::HashMap;
    use std::io::{Error as IoError, ErrorKind};

    /// Test double that simulates various filesystem errors
    #[derive(Clone)]
    pub struct FailingFileSystem {
        /// Paths that should fail with permission denied
        permission_denied_paths: Vec<std::path::PathBuf>,
        /// Paths that should fail with disk full
        disk_full_paths: Vec<std::path::PathBuf>,
        /// In-memory file storage for successful operations
        files: HashMap<std::path::PathBuf, String>,
    }

    impl FailingFileSystem {
        pub fn new() -> Self {
            Self {
                permission_denied_paths: Vec::new(),
                disk_full_paths: Vec::new(),
                files: HashMap::new(),
            }
        }

        pub fn fail_with_permission_denied(mut self, path: impl Into<std::path::PathBuf>) -> Self {
            self.permission_denied_paths.push(path.into());
            self
        }

        pub fn fail_with_disk_full(mut self, path: impl Into<std::path::PathBuf>) -> Self {
            self.disk_full_paths.push(path.into());
            self
        }

        pub fn with_file(
            mut self,
            path: impl Into<std::path::PathBuf>,
            content: impl Into<String>,
        ) -> Self {
            self.files.insert(path.into(), content.into());
            self
        }
    }

    impl FileSystem for FailingFileSystem {
        fn exists(&self, path: &Path) -> bool {
            self.files.contains_key(path)
                || self.permission_denied_paths.contains(&path.to_path_buf())
                || self.disk_full_paths.contains(&path.to_path_buf())
        }

        fn read_to_string(&self, path: &Path) -> Result<String> {
            if self.permission_denied_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::PermissionDenied, "Permission denied").into());
            }

            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| IoError::new(ErrorKind::NotFound, "File not found").into())
        }

        fn write(&self, path: &Path, _content: &str) -> Result<()> {
            if self.permission_denied_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::PermissionDenied, "Permission denied").into());
            }

            if self.disk_full_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::Other, "No space left on device").into());
            }

            Ok(())
        }

        fn create_dir_all(&self, path: &Path) -> Result<()> {
            if self.permission_denied_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::PermissionDenied, "Permission denied").into());
            }

            Ok(())
        }

        fn remove_file(&self, path: &Path) -> Result<()> {
            if self.permission_denied_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::PermissionDenied, "Permission denied").into());
            }

            if !self.files.contains_key(path) {
                return Err(IoError::new(ErrorKind::NotFound, "File not found").into());
            }

            Ok(())
        }

        fn read_dir(&self, path: &Path) -> Result<Vec<std::path::PathBuf>> {
            if self.permission_denied_paths.contains(&path.to_path_buf()) {
                return Err(IoError::new(ErrorKind::PermissionDenied, "Permission denied").into());
            }

            // Return files that are in this directory
            let entries: Vec<_> = self
                .files
                .keys()
                .filter(|p| p.parent() == Some(path))
                .cloned()
                .collect();

            Ok(entries)
        }
    }
}
