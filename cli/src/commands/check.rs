// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Check command implementation

use crate::commands::GitMindContext;
use crate::error::{Error, Result};
use crate::link::Link;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct CheckCommand {
    context: GitMindContext,
}

impl CheckCommand {
    pub fn new(working_dir: &Path) -> Result<Self> {
        let context = GitMindContext::new(working_dir)?;
        Ok(Self { context })
    }

    pub fn execute(&self, fix: bool, dry_run: bool) -> Result<Vec<Link>> {
        let links_dir = self.context.links_dir();
        let mut broken_links = Vec::new();

        // Read all link files
        for entry in fs::read_dir(&links_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("link") {
                let content = fs::read_to_string(&path)?;
                if let Ok(link) = Link::from_canonical_string(&content) {
                    // Check if source or target exists
                    let source_exists = self.context.working_dir.join(&link.source).exists();
                    let target_exists = self.context.working_dir.join(&link.target).exists();

                    if !source_exists || !target_exists {
                        broken_links.push(link);
                    }
                }
            }
        }

        // If fix is requested and not dry run, remove the broken links
        if fix && !dry_run && !broken_links.is_empty() {
            let links_to_remove = broken_links.clone();

            // Remove link files
            for link in &links_to_remove {
                let link_file = links_dir.join(format!("{}.link", link.short_sha()));
                if link_file.exists() {
                    fs::remove_file(&link_file)?;
                }
            }

            // Git add the removed files
            let output = Command::new("git")
                .current_dir(&self.context.working_dir)
                .args(["add", "-u", ".gitmind/links/"])
                .output()
                .map_err(|e| Error::Git(format!("Failed to run git add: {}", e)))?;

            if !output.status.success() {
                return Err(Error::Git(format!(
                    "git add failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }

            // Git commit
            let commit_msg = format!(
                "check(F016): removed {} broken link{}",
                links_to_remove.len(),
                if links_to_remove.len() == 1 { "" } else { "s" }
            );

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
        }

        Ok(broken_links)
    }
}
