//! Domain error types for the core crate.

use std::fmt;

/// Errors originating from core domain logic.
#[derive(Debug, Clone)]
pub enum CoreError {
    /// The requested resource was not found.
    NotFound(String),
    /// The input provided was invalid.
    InvalidInput(String),
    /// An internal error occurred.
    Internal(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::NotFound(msg) => write!(f, "not found: {}", msg),
            CoreError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            CoreError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for CoreError {}
