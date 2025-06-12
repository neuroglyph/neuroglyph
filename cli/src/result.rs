// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Command execution result type

use crate::error::Error;

/// Result of executing a command
#[derive(Debug)]
pub struct CommandResult<T> {
    /// Exit code (0 = success, 1+ = error)
    pub code: i32,
    /// Optional return value
    pub value: Option<T>,
}

impl<T> CommandResult<T> {
    /// Create a successful result with a value
    pub fn success(value: T) -> Self {
        Self {
            code: 0,
            value: Some(value),
        }
    }

    /// Create a successful result without a value
    pub fn success_empty() -> Self {
        Self {
            code: 0,
            value: None,
        }
    }

    /// Create an error result
    pub fn error(code: i32) -> Self {
        Self { code, value: None }
    }

    /// Check if the command succeeded
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// Convert to a Result for easier handling
    pub fn to_result(self) -> Result<Option<T>, i32> {
        if self.is_success() {
            Ok(self.value)
        } else {
            Err(self.code)
        }
    }
}

/// Extension trait to convert our Error type to CommandResult
pub trait IntoCommandResult<T> {
    fn into_command_result(self) -> CommandResult<T>;
}

impl<T> IntoCommandResult<T> for Result<T, Error> {
    fn into_command_result(self) -> CommandResult<T> {
        match self {
            Ok(value) => CommandResult::success(value),
            Err(e) => {
                // Map different error types to exit codes
                let code = match e {
                    Error::NotAGitRepository => 2,
                    Error::NotInitialized => 3,
                    Error::AlreadyInitialized => 4,
                    Error::SourceNotFound(_) => 5,
                    Error::TargetNotFound(_) => 6,
                    Error::LinkAlreadyExists => 7,
                    Error::Git(_) => 8,
                    Error::GitError(_) => 8,
                    Error::Io(_) => 9,
                    Error::IoError(_) => 9,
                    Error::ParseError(_) => 10,
                };
                CommandResult::error(code)
            }
        }
    }
}
