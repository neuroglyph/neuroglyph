// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Shared context for commands that require an initialized gitmind repository

use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

/// Context for commands that operate on an initialized gitmind repository
pub struct GitMindContext {
    /// The working directory path
    pub working_dir: PathBuf,
    /// The .gitmind directory path
    pub gitmind_dir: PathBuf,
}

impl GitMindContext {
    /// Create a new context, validating that the gitmind repository is initialized
    pub fn new(working_dir: &Path) -> Result<Self> {
        let gitmind_dir = working_dir.join(".gitmind");

        if !gitmind_dir.exists() {
            return Err(Error::NotInitialized);
        }

        Ok(Self {
            working_dir: working_dir.to_path_buf(),
            gitmind_dir,
        })
    }

    /// Create a new context without validation (for init command)
    pub fn new_unchecked(working_dir: &Path) -> Self {
        Self {
            working_dir: working_dir.to_path_buf(),
            gitmind_dir: working_dir.join(".gitmind"),
        }
    }

    /// Get the links directory path
    pub fn links_dir(&self) -> PathBuf {
        self.gitmind_dir.join("links")
    }
}
