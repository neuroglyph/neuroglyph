// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Core library for gitmind CLI

pub mod commands;
pub mod error;
pub mod filesystem;
pub mod git;
pub mod link;
pub mod result;
pub mod time;

use filesystem::{FileSystem, StdFileSystem};
use git::{GitClient, GitOperations};
use result::{CommandResult, IntoCommandResult};
use std::path::Path;
use time::{Clock, SystemClock};

/// Main application logic
pub struct App<G: GitOperations = GitClient, F: FileSystem = StdFileSystem, C: Clock = SystemClock>
{
    /// Working directory
    pub working_dir: std::path::PathBuf,
    git: G,
    fs: F,
    clock: C,
}

impl App<GitClient, StdFileSystem, SystemClock> {
    /// Create a new App instance with default dependencies
    pub fn new(working_dir: impl AsRef<Path>) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
            git: GitClient,
            fs: StdFileSystem,
            clock: SystemClock,
        }
    }
}

impl<G: GitOperations, F: FileSystem, C: Clock> App<G, F, C> {
    /// Create a new App instance with custom dependencies
    pub fn with_deps(working_dir: impl AsRef<Path>, git: G, fs: F, clock: C) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
            git,
            fs,
            clock,
        }
    }

    /// Initialize gitmind in the current repository
    pub fn init(&self) -> CommandResult<()> {
        let cmd = commands::InitCommand::new(&self.working_dir, self.git.clone(), self.fs.clone());
        cmd.execute().into_command_result()
    }

    /// Create a semantic link between two files
    pub fn link(&self, source: &str, target: &str, link_type: &str) -> CommandResult<String> {
        match commands::LinkCommand::new(
            &self.working_dir,
            self.git.clone(),
            self.fs.clone(),
            self.clock.clone(),
        ) {
            Ok(cmd) => cmd.execute(source, target, link_type).into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }

    /// List all semantic links
    pub fn list(
        &self,
        source: Option<&str>,
        target: Option<&str>,
    ) -> CommandResult<Vec<link::Link>> {
        match commands::ListCommand::new(&self.working_dir, self.fs.clone()) {
            Ok(cmd) => cmd.execute(source, target).into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }

    /// Remove a specific link between two files
    pub fn unlink(
        &self,
        source: &str,
        target: &str,
        link_type: Option<&str>,
    ) -> CommandResult<usize> {
        match commands::UnlinkCommand::new(&self.working_dir, self.git.clone(), self.fs.clone()) {
            Ok(cmd) => cmd.execute(source, target, link_type).into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }

    /// Remove all links from a source file
    pub fn unlink_all_from(&self, source: &str, link_type: Option<&str>) -> CommandResult<usize> {
        match commands::UnlinkCommand::new(&self.working_dir, self.git.clone(), self.fs.clone()) {
            Ok(cmd) => cmd
                .execute_all_from(source, link_type)
                .into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }

    /// Remove all links to a target file
    pub fn unlink_to(&self, target: &str, link_type: Option<&str>) -> CommandResult<usize> {
        match commands::UnlinkCommand::new(&self.working_dir, self.git.clone(), self.fs.clone()) {
            Ok(cmd) => cmd.execute_to(target, link_type).into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }

    /// Check for broken links and optionally fix them
    pub fn check(&self, fix: bool, dry_run: bool) -> CommandResult<Vec<link::Link>> {
        match commands::CheckCommand::new(&self.working_dir, self.fs.clone(), self.git.clone()) {
            Ok(cmd) => cmd.execute(fix, dry_run).into_command_result(),
            Err(e) => Err(e).into_command_result(),
        }
    }
}
