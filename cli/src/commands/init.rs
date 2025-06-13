// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Initialize command implementation

use crate::commands::GitMindContext;
use crate::error::{Error, Result};
use crate::filesystem::FileSystem;
use crate::git::GitOperations;
use std::path::Path;

pub struct InitCommand<'a, G: GitOperations, F: FileSystem> {
    working_dir: &'a Path,
    git: G,
    fs: F,
}

impl<'a, G: GitOperations, F: FileSystem> InitCommand<'a, G, F> {
    pub fn new(working_dir: &'a Path, git: G, fs: F) -> Self {
        Self {
            working_dir,
            git,
            fs,
        }
    }

    pub fn execute(&self) -> Result<()> {
        // Check if we're in a git repository
        if !self.git.is_repository(self.working_dir)? {
            return Err(Error::NotAGitRepository);
        }

        // Create context (unchecked since we're initializing)
        let context = GitMindContext::new_unchecked(self.working_dir);

        // Check if already initialized
        if self.fs.exists(&context.gitmind_dir) {
            return Err(Error::AlreadyInitialized);
        }

        // Create .gitmind/links/ directory
        self.fs.create_dir_all(&context.links_dir())?;

        // Create .gitkeep file to prevent directory removal when empty
        let gitkeep_path = context.gitmind_dir.join(".gitkeep");
        self.fs.write(&gitkeep_path, "")?;

        // Add .gitkeep to git to track it
        let gitkeep_str = gitkeep_path.to_str().ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8 in path",
            ))
        })?;

        self.git.add(self.working_dir, gitkeep_str)?;

        Ok(())
    }
}
