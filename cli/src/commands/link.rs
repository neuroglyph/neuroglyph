// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Link command implementation

use crate::commands::GitMindContext;
use crate::error::{Error, Result};
use crate::filesystem::FileSystem;
use crate::git::GitOperations;
use crate::link::Link;
use crate::time::Clock;
use std::path::Path;

pub struct LinkCommand<G: GitOperations, F: FileSystem, C: Clock> {
    context: GitMindContext,
    git: G,
    fs: F,
    clock: C,
}

impl<G: GitOperations, F: FileSystem, C: Clock> LinkCommand<G, F, C> {
    pub fn new(working_dir: &Path, git: G, fs: F, clock: C) -> Result<Self> {
        let context = GitMindContext::new(working_dir)?;
        Ok(Self {
            context,
            git,
            fs,
            clock,
        })
    }

    pub fn execute(&self, source: &str, target: &str, link_type: &str) -> Result<String> {
        // Check if source file exists
        let source_path = self.context.working_dir.join(source);
        if !self.fs.exists(&source_path) {
            return Err(Error::SourceNotFound(source.to_string()));
        }

        // Check if target file exists
        let target_path = self.context.working_dir.join(target);
        if !self.fs.exists(&target_path) {
            return Err(Error::TargetNotFound(target.to_string()));
        }

        // Create link
        let timestamp = self.clock.now();
        let link = Link::new(
            link_type.to_string(),
            source.to_string(),
            target.to_string(),
            timestamp,
        );

        // Create link file path
        let link_file = self
            .context
            .links_dir()
            .join(format!("{}.link", link.short_sha()));

        // Check if link already exists
        if self.fs.exists(&link_file) {
            // Read existing content to check if it's the same link
            let existing = self.fs.read_to_string(&link_file)?;
            if existing.starts_with(&format!("{}: {} -> {}", link_type, source, target)) {
                return Err(Error::LinkAlreadyExists);
            }
        }

        // Write link file
        self.fs.write(&link_file, &link.to_canonical_string())?;

        // Git add the link file
        let link_file_str = link_file.to_str().ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8 in path",
            ))
        })?;

        self.git.add(&self.context.working_dir, link_file_str)?;

        // Git commit
        let commit_msg = format!("link(F001): {} -> {}", source, target);
        self.git.commit(&self.context.working_dir, &commit_msg)?;

        Ok(link.short_sha())
    }
}
