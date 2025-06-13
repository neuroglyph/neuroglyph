// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! List command implementation

use crate::commands::GitMindContext;
use crate::error::Result;
use crate::filesystem::FileSystem;
use crate::link::Link;
use std::path::Path;

pub struct ListCommand<F: FileSystem> {
    context: GitMindContext,
    fs: F,
}

impl<F: FileSystem> ListCommand<F> {
    pub fn new(working_dir: &Path, fs: F) -> Result<Self> {
        let context = GitMindContext::new(working_dir)?;
        Ok(Self { context, fs })
    }

    pub fn execute(
        &self,
        source_filter: Option<&str>,
        target_filter: Option<&str>,
    ) -> Result<Vec<Link>> {
        let links_dir = self.context.links_dir();
        if !self.fs.exists(&links_dir) {
            // Links directory was removed (e.g., by git rm when empty)
            // This is OK - just return empty list
            return Ok(Vec::new());
        }

        let mut links = Vec::new();

        // Read all link files
        let entries = self.fs.read_dir(&links_dir)?;
        for path in entries {
            // Skip if not a .link file
            if path.extension().and_then(|s| s.to_str()) != Some("link") {
                continue;
            }

            // Read and parse link
            let content = self.fs.read_to_string(&path)?;
            let content = content.trim();

            match Link::from_canonical_string(content) {
                Ok(link) => {
                    // Apply filters
                    let matches_source = source_filter.is_none_or(|f| link.source == f);
                    let matches_target = target_filter.is_none_or(|f| link.target == f);

                    if matches_source && matches_target {
                        links.push(link);
                    }
                }
                Err(_) => {
                    // Skip malformed link files silently
                    // They will be caught by the check command
                }
            }
        }

        // Sort by timestamp (oldest first)
        links.sort_by_key(|link| link.timestamp);

        Ok(links)
    }
}
