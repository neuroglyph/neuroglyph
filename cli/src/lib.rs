// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Core library for gitmind CLI

pub mod error;

use error::Result;
use std::path::Path;

/// Main application logic
pub struct App {
    /// Working directory
    pub working_dir: std::path::PathBuf,
}

impl App {
    /// Create a new App instance
    pub fn new(working_dir: impl AsRef<Path>) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
        }
    }

    /// Initialize gitmind in the current repository
    pub fn init(&self) -> Result<()> {
        // Check if we're in a git repository
        let git_dir = self.working_dir.join(".git");
        if !git_dir.exists() {
            return Err(error::Error::NotAGitRepository);
        }

        // Check if already initialized
        let gitmind_dir = self.working_dir.join(".gitmind");
        if gitmind_dir.exists() {
            return Err(error::Error::AlreadyInitialized);
        }

        // Create .gitmind/links/ directory
        let links_dir = gitmind_dir.join("links");
        std::fs::create_dir_all(links_dir)?;

        Ok(())
    }
}
