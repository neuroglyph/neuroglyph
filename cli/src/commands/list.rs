// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! List command implementation

use crate::error::{Error, Result};
use crate::link::Link;
use std::fs;
use std::path::Path;

pub struct ListCommand<'a> {
    working_dir: &'a Path,
}

impl<'a> ListCommand<'a> {
    pub fn new(working_dir: &'a Path) -> Self {
        Self { working_dir }
    }

    pub fn execute(
        &self,
        source_filter: Option<&str>,
        target_filter: Option<&str>,
    ) -> Result<Vec<Link>> {
        // Check if gitmind is initialized
        let gitmind_dir = self.working_dir.join(".gitmind");
        if !gitmind_dir.exists() {
            return Err(Error::NotInitialized);
        }

        let links_dir = gitmind_dir.join("links");
        if !links_dir.exists() {
            // Links directory was removed (e.g., by git rm when empty)
            // This is OK - just return empty list
            return Ok(Vec::new());
        }

        let mut links = Vec::new();

        // Read all link files
        for entry in fs::read_dir(&links_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip if not a .link file
            if path.extension().and_then(|s| s.to_str()) != Some("link") {
                continue;
            }

            // Read and parse link
            let content = fs::read_to_string(&path)?;
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
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse link file {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }

        // Sort by timestamp (oldest first)
        links.sort_by_key(|link| link.timestamp);

        Ok(links)
    }
}
