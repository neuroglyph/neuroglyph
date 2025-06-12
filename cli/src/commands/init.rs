// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Initialize command implementation

use crate::commands::GitMindContext;
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

        // Create context (unchecked since we're initializing)
        let context = GitMindContext::new_unchecked(self.working_dir);

        // Check if already initialized
        if context.gitmind_dir.exists() {
            return Err(crate::error::Error::AlreadyInitialized);
        }

        // Create .gitmind/links/ directory
        std::fs::create_dir_all(context.links_dir())?;

        // Create .gitkeep file to prevent directory removal when empty
        let gitkeep_path = context.gitmind_dir.join(".gitkeep");
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
