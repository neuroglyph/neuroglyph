// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Error types for gitmind

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not a git repository")]
    NotAGitRepository,

    #[error("gitmind already initialized")]
    AlreadyInitialized,

    #[error("gitmind not initialized")]
    NotInitialized,

    #[error("Source file not found: {0}")]
    SourceNotFound(String),

    #[error("Target file not found: {0}")]
    TargetNotFound(String),

    #[error("Link already exists")]
    LinkAlreadyExists,

    #[error("Git error: {0}")]
    Git(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
