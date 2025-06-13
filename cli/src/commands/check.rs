// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Check command implementation

use crate::commands::GitMindContext;
use crate::error::Result;
use crate::filesystem::FileSystem;
use crate::git::GitOperations;
use crate::link::Link;
use std::path::Path;

pub struct CheckCommand<F: FileSystem, G: GitOperations> {
    context: GitMindContext,
    fs: F,
    git: G,
}

impl<F: FileSystem, G: GitOperations> CheckCommand<F, G> {
    pub fn new(working_dir: &Path, fs: F, git: G) -> Result<Self> {
        let context = GitMindContext::new(working_dir)?;
        Ok(Self { context, fs, git })
    }

    pub fn execute(&self, fix: bool, dry_run: bool) -> Result<Vec<Link>> {
        let links_dir = self.context.links_dir();
        let mut broken_links = Vec::new();

        // Read all link files
        let entries = self.fs.read_dir(&links_dir)?;
        for path in entries {
            if path.extension().and_then(|s| s.to_str()) == Some("link") {
                let content = self.fs.read_to_string(&path)?;
                if let Ok(link) = Link::from_canonical_string(&content) {
                    // Check if source or target exists
                    let source_exists =
                        self.fs.exists(&self.context.working_dir.join(&link.source));
                    let target_exists =
                        self.fs.exists(&self.context.working_dir.join(&link.target));

                    if !source_exists || !target_exists {
                        broken_links.push(link);
                    }
                }
            }
        }

        // If fix is requested and not dry run, remove the broken links
        if fix && !dry_run && !broken_links.is_empty() {
            let links_to_remove = broken_links.clone();

            // Remove link files using git rm
            for link in &links_to_remove {
                let link_file = links_dir.join(format!("{}.link", link.short_sha()));
                if self.fs.exists(&link_file) {
                    let relative_path = format!(".gitmind/links/{}.link", link.short_sha());
                    self.git.remove(&self.context.working_dir, &relative_path)?;
                }
            }

            // Git commit
            let commit_msg = format!(
                "check(F016): removed {} broken link{}",
                links_to_remove.len(),
                if links_to_remove.len() == 1 { "" } else { "s" }
            );

            self.git.commit(&self.context.working_dir, &commit_msg)?;
        }

        Ok(broken_links)
    }
}
