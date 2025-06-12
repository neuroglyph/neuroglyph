// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Initialize command implementation

use crate::error::Result;
use std::path::Path;

pub struct InitCommand<'a> {
    working_dir: &'a Path,
}

impl<'a> InitCommand<'a> {
    pub fn new(working_dir: &'a Path) -> Self {
        Self { working_dir }
    }

    pub fn execute(&self) -> Result<()> {
        // Check if we're in a git repository
        let git_dir = self.working_dir.join(".git");
        if !git_dir.exists() {
            return Err(crate::error::Error::NotAGitRepository);
        }

        // Check if already initialized
        let gitmind_dir = self.working_dir.join(".gitmind");
        if gitmind_dir.exists() {
            return Err(crate::error::Error::AlreadyInitialized);
        }

        // Create .gitmind/links/ directory
        let links_dir = gitmind_dir.join("links");
        std::fs::create_dir_all(links_dir)?;

        // Create .gitkeep file to prevent directory removal when empty
        let gitkeep_path = gitmind_dir.join(".gitkeep");
        std::fs::write(&gitkeep_path, "")?;

        // Add .gitkeep to git to track it
        let output = std::process::Command::new("git")
            .current_dir(self.working_dir)
            .args(["add", &gitkeep_path.to_string_lossy()])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::Error::Git(format!(
                "Failed to add .gitkeep: {}",
                stderr
            )));
        }

        Ok(())
    }
}
