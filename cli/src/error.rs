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

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
