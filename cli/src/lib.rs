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

    /// List all semantic links
    pub fn list(&self, source: Option<&str>, target: Option<&str>) -> Result<Vec<link::Link>> {
        let cmd = commands::ListCommand::new(&self.working_dir);
        cmd.execute(source, target)
    }

    /// Remove a specific link between two files
    pub fn unlink(&self, source: &str, target: &str, link_type: Option<&str>) -> Result<usize> {
        let cmd = commands::UnlinkCommand::new(&self.working_dir);
        cmd.execute(source, target, link_type)
    }

    /// Remove all links from a source file
    pub fn unlink_all_from(&self, source: &str, link_type: Option<&str>) -> Result<usize> {
        let cmd = commands::UnlinkCommand::new(&self.working_dir);
        cmd.execute_all_from(source, link_type)
    }

    /// Remove all links to a target file
    pub fn unlink_to(&self, target: &str, link_type: Option<&str>) -> Result<usize> {
        let cmd = commands::UnlinkCommand::new(&self.working_dir);
        cmd.execute_to(target, link_type)
    }
}
