//! Error types for the CLI binary.

use std::fmt;

/// Errors originating from the CLI layer.
#[derive(Debug)]
pub enum CliError {
    /// An error from the core domain logic.
    Domain(foo_core::error::CoreError),
    /// Invalid command-line arguments.
    InvalidArgs(String),
    /// An internal error.
    Internal(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Domain(err) => write!(f, "{}", err),
            CliError::InvalidArgs(msg) => write!(f, "invalid arguments: {}", msg),
            CliError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<foo_core::error::CoreError> for CliError {
    fn from(err: foo_core::error::CoreError) -> Self {
        CliError::Domain(err)
    }
}
