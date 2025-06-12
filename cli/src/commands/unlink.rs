// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Unlink command implementation

use crate::error::{Error, Result};
use crate::link::Link;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Command to remove links between files
pub struct UnlinkCommand {
    working_dir: PathBuf,
}

impl UnlinkCommand {
    /// Create a new UnlinkCommand
    pub fn new(working_dir: &Path) -> Self {
        Self {
            working_dir: working_dir.to_path_buf(),
        }
    }

    /// Remove a specific link between two files
    pub fn execute(&self, source: &str, target: &str, link_type: Option<&str>) -> Result<usize> {
        self.ensure_gitmind_initialized()?;

        let links = self.find_links(Some(source), Some(target), link_type)?;
        let count = links.len();

        if count == 0 {
            return Ok(0);
        }

        self.remove_links(&links)?;
        self.commit_removal(source, target, count)?;

        Ok(count)
    }

    /// Remove all links from a source file
    pub fn execute_all_from(&self, source: &str, link_type: Option<&str>) -> Result<usize> {
        self.ensure_gitmind_initialized()?;

        let links = self.find_links(Some(source), None, link_type)?;
        let count = links.len();

        if count == 0 {
            return Ok(0);
        }

        self.remove_links(&links)?;
        self.commit_removal_all_from(source, count)?;

        Ok(count)
    }

    /// Remove all links to a target file
    pub fn execute_to(&self, target: &str, link_type: Option<&str>) -> Result<usize> {
        self.ensure_gitmind_initialized()?;

        let links = self.find_links(None, Some(target), link_type)?;
        let count = links.len();

        if count == 0 {
            return Ok(0);
        }

        self.remove_links(&links)?;
        self.commit_removal_to(target, count)?;

        Ok(count)
    }

    /// Ensure gitmind is initialized
    fn ensure_gitmind_initialized(&self) -> Result<()> {
        let gitmind_dir = self.working_dir.join(".gitmind");
        if !gitmind_dir.exists() {
            return Err(Error::NotInitialized);
        }
        Ok(())
    }

    /// Find links matching the given criteria
    fn find_links(
        &self,
        source: Option<&str>,
        target: Option<&str>,
        link_type: Option<&str>,
    ) -> Result<Vec<(Link, PathBuf)>> {
        let links_dir = self.working_dir.join(".gitmind/links");
        let mut matching_links = Vec::new();

        // If links directory doesn't exist, there are no links to remove
        if !links_dir.exists() {
            return Ok(matching_links);
        }

        for entry in fs::read_dir(links_dir)
            .map_err(|e| Error::IoError(format!("Failed to read links directory: {}", e)))?
        {
            let entry = entry
                .map_err(|e| Error::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("link") {
                let content = fs::read_to_string(&path).map_err(|e| {
                    Error::IoError(format!(
                        "Failed to read link file {}: {}",
                        path.display(),
                        e
                    ))
                })?;

                let link = Link::from_canonical_string(&content)?;

                let matches = match (source, target) {
                    (Some(s), Some(t)) => link.source == s && link.target == t,
                    (Some(s), None) => link.source == s,
                    (None, Some(t)) => link.target == t,
                    (None, None) => true,
                };

                let type_matches = link_type.map_or(true, |t| link.link_type == t);

                if matches && type_matches {
                    matching_links.push((link, path));
                }
            }
        }

        Ok(matching_links)
    }

    /// Remove the link files
    fn remove_links(&self, links: &[(Link, PathBuf)]) -> Result<()> {
        for (_link, path) in links {
            // Stage the removal with git rm
            let output = Command::new("git")
                .current_dir(&self.working_dir)
                .args(["rm", &path.to_string_lossy()])
                .output()
                .map_err(|e| Error::GitError(format!("Failed to run git rm: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(Error::GitError(format!("git rm failed: {}", stderr)));
            }
        }
        Ok(())
    }

    /// Commit the removal of a specific link
    fn commit_removal(&self, source: &str, target: &str, count: usize) -> Result<()> {
        let message = format!(
            "unlink(F016): {} -/-> {} ({} link{})",
            source,
            target,
            count,
            if count == 1 { "" } else { "s" }
        );
        self.git_commit(&message)
    }

    /// Commit the removal of all links from a source
    fn commit_removal_all_from(&self, source: &str, count: usize) -> Result<()> {
        let message = format!(
            "unlink(F016): all from {} ({} link{})",
            source,
            count,
            if count == 1 { "" } else { "s" }
        );
        self.git_commit(&message)
    }

    /// Commit the removal of all links to a target
    fn commit_removal_to(&self, target: &str, count: usize) -> Result<()> {
        let message = format!(
            "unlink(F016): all to {} ({} link{})",
            target,
            count,
            if count == 1 { "" } else { "s" }
        );
        self.git_commit(&message)
    }

    /// Create a git commit
    fn git_commit(&self, message: &str) -> Result<()> {
        let output = Command::new("git")
            .current_dir(&self.working_dir)
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| Error::GitError(format!("Failed to run git commit: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::GitError(format!("git commit failed: {}", stderr)));
        }

        Ok(())
    }
}
