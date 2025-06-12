// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Core library for gitmind CLI

pub mod commands;
pub mod error;
pub mod link;

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
        let cmd = commands::InitCommand::new(&self.working_dir);
        cmd.execute()
    }

    /// Create a semantic link between two files
    pub fn link(&self, source: &str, target: &str, link_type: &str) -> Result<String> {
        let cmd = commands::LinkCommand::new(&self.working_dir);
        cmd.execute(source, target, link_type)
    }
}
