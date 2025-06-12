// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Link command implementation

use crate::commands::GitMindContext;
use crate::error::{Error, Result};
use crate::link::Link;
use std::path::Path;
use std::process::Command;

pub struct LinkCommand {
    context: GitMindContext,
}

impl LinkCommand {
    pub fn new(working_dir: &Path) -> Result<Self> {
        let context = GitMindContext::new(working_dir)?;
        Ok(Self { context })
    }

    pub fn execute(&self, source: &str, target: &str, link_type: &str) -> Result<String> {
        // Check if source file exists
        let source_path = self.context.working_dir.join(source);
        if !source_path.exists() {
            return Err(Error::SourceNotFound(source.to_string()));
        }

        // Check if target file exists
        let target_path = self.context.working_dir.join(target);
        if !target_path.exists() {
            return Err(Error::TargetNotFound(target.to_string()));
        }

        // Create link
        let timestamp = chrono::Utc::now().timestamp();
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
        if link_file.exists() {
            // Read existing content to check if it's the same link
            let existing = std::fs::read_to_string(&link_file)?;
            if existing.starts_with(&format!("{}: {} -> {}", link_type, source, target)) {
                return Err(Error::LinkAlreadyExists);
            }
        }

        // Write link file
        std::fs::write(&link_file, link.to_canonical_string())?;

        // Git add the link file
        let output = Command::new("git")
            .current_dir(&self.context.working_dir)
            .args(["add", link_file.to_str().unwrap()])
            .output()
            .map_err(|e| Error::Git(format!("Failed to run git add: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Git(format!(
                "git add failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Git commit
        let commit_msg = format!("link(F001): {} -> {}", source, target);
        let output = Command::new("git")
            .current_dir(&self.context.working_dir)
            .args(["commit", "-m", &commit_msg])
            .output()
            .map_err(|e| Error::Git(format!("Failed to run git commit: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Git(format!(
                "git commit failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(link.short_sha())
    }
}
