// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Git operations trait for dependency injection

use crate::error::{Error, Result};
use std::path::Path;

/// Trait for Git operations to enable dependency injection and testing
pub trait GitOperations: Send + Sync + Clone {
    /// Check if the current directory is a git repository
    fn is_repository(&self, path: &Path) -> Result<bool>;

    /// Add a file to the git index
    fn add(&self, working_dir: &Path, file_path: &str) -> Result<()>;

    /// Commit changes with the given message
    fn commit(&self, working_dir: &Path, message: &str) -> Result<()>;

    /// Remove a file from git tracking
    fn remove(&self, working_dir: &Path, file_path: &str) -> Result<()>;
}

/// Default implementation using gitoxide
#[derive(Clone)]
pub struct GitClient;

impl GitOperations for GitClient {
    fn is_repository(&self, path: &Path) -> Result<bool> {
        match gix::discover(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn add(&self, working_dir: &Path, file_path: &str) -> Result<()> {
        // For now, use git command directly until we figure out the gitoxide API
        let output = std::process::Command::new("git")
            .current_dir(working_dir)
            .args(["add", file_path])
            .output()
            .map_err(|e| Error::Git(format!("Failed to run git add: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Git(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    fn commit(&self, working_dir: &Path, message: &str) -> Result<()> {
        // For now, use git command directly until we figure out the gitoxide API
        let output = std::process::Command::new("git")
            .current_dir(working_dir)
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| Error::Git(format!("Failed to run git commit: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Git(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    fn remove(&self, working_dir: &Path, file_path: &str) -> Result<()> {
        // For now, use git command directly until we figure out the gitoxide API
        let output = std::process::Command::new("git")
            .current_dir(working_dir)
            .args(["rm", file_path])
            .output()
            .map_err(|e| Error::Git(format!("Failed to run git rm: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Git(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod test_doubles {
    use super::*;

    /// Simulates index.lock contention - the most common Git edge case
    #[derive(Clone)]
    pub struct IndexLockedGit;

    impl GitOperations for IndexLockedGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: Unable to create '.git/index.lock': File exists.\n\n\
                Another git process seems to be running in this repository, e.g.\n\
                an editor opened by 'git commit'. Please make sure all processes\n\
                are terminated then try again."
                    .to_string(),
            ))
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Ok(())
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates a repository with no commits (no HEAD)
    #[derive(Clone)]
    pub struct NoHeadGit;

    impl GitOperations for NoHeadGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: reference 'refs/heads/main' not found".to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates corrupted Git repository
    #[derive(Clone)]
    pub struct CorruptedRepoGit;

    impl GitOperations for CorruptedRepoGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(false) // Corruption makes it unrecognizable as a repo
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: not a git repository (or any of the parent directories): .git".to_string(),
            ))
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: not a git repository (or any of the parent directories): .git".to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: not a git repository (or any of the parent directories): .git".to_string(),
            ))
        }
    }

    /// Simulates disk full during Git operations
    #[derive(Clone)]
    pub struct DiskFullGit;

    impl GitOperations for DiskFullGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: Unable to write new index file\n\
                error: No space left on device"
                    .to_string(),
            ))
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: failed to write commit object\n\
                error: No space left on device"
                    .to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: Unable to write new index file\n\
                error: No space left on device"
                    .to_string(),
            ))
        }
    }

    /// Simulates a bare repository (no working tree)
    #[derive(Clone)]
    pub struct BareRepositoryGit;

    impl GitOperations for BareRepositoryGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true) // It IS a repository, just bare
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: this operation must be run in a work tree".to_string(),
            ))
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: this operation must be run in a work tree".to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Err(Error::Git(
                "fatal: this operation must be run in a work tree".to_string(),
            ))
        }
    }

    /// Simulates detached HEAD state
    #[derive(Clone)]
    pub struct DetachedHeadGit;

    impl GitOperations for DetachedHeadGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(()) // Add works in detached HEAD
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            // Commits work but with a warning (we'll simulate success but the warning would be shown)
            Ok(())
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates pre-commit hook failure
    #[derive(Clone)]
    pub struct HookFailureGit;

    impl GitOperations for HookFailureGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "hint: The pre-commit hook was failed.\n\
                hint: The 'pre-commit' hook was ignored because it's not set as executable.\n\
                error: cannot run .git/hooks/pre-commit: Permission denied"
                    .to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates missing Git configuration
    #[derive(Clone)]
    pub struct NoConfigGit;

    impl GitOperations for NoConfigGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Err(Error::Git(
                "Author identity unknown\n\n\
                *** Please tell me who you are.\n\n\
                Run\n\n\
                  git config --global user.email \"you@example.com\"\n\
                  git config --global user.name \"Your Name\"\n\n\
                to set your account's default identity."
                    .to_string(),
            ))
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates operations in a Git worktree (not the main working directory)
    #[derive(Clone)]
    pub struct WorktreeGit;

    impl GitOperations for WorktreeGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            // Worktrees can add files normally
            Ok(())
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            // Commits in worktrees work but go to the shared repository
            Ok(())
        }

        fn remove(&self, _working_dir: &Path, _file_path: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Simulates operations across submodule boundaries
    #[derive(Clone)]
    pub struct SubmoduleGit;

    impl GitOperations for SubmoduleGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, file_path: &str) -> Result<()> {
            // Simulate trying to add a file that's in a submodule
            if file_path.contains("submodule/") {
                Err(Error::Git(
                    "fatal: Pathspec 'submodule/file.txt' is in submodule 'submodule'".to_string(),
                ))
            } else {
                Ok(())
            }
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Ok(())
        }

        fn remove(&self, _working_dir: &Path, file_path: &str) -> Result<()> {
            if file_path.contains("submodule/") {
                Err(Error::Git(
                    "fatal: Pathspec 'submodule/file.txt' is in submodule 'submodule'".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    /// Simulates non-UTF8 path encoding issues
    #[derive(Clone)]
    pub struct InvalidEncodingGit;

    impl GitOperations for InvalidEncodingGit {
        fn is_repository(&self, _path: &Path) -> Result<bool> {
            Ok(true)
        }

        fn add(&self, _working_dir: &Path, file_path: &str) -> Result<()> {
            // Simulate invalid UTF-8 in path
            if file_path.contains('\u{FFFD}') || file_path.contains("�") {
                Err(Error::Git(
                    "fatal: invalid path '�����.txt'\n\
                    error: unable to process path with invalid encoding"
                        .to_string(),
                ))
            } else {
                Ok(())
            }
        }

        fn commit(&self, _working_dir: &Path, _message: &str) -> Result<()> {
            Ok(())
        }

        fn remove(&self, _working_dir: &Path, file_path: &str) -> Result<()> {
            if file_path.contains('\u{FFFD}') || file_path.contains("�") {
                Err(Error::Git(
                    "fatal: invalid path '�����.txt'\n\
                    error: unable to process path with invalid encoding"
                        .to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }
}
