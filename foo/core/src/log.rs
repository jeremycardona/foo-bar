//! Logging facade for the core domain.
//!
//! Each crate defines its own minimal logging trait.
//! The binary crate wires all facades together at startup.

/// Minimal logging facade for domain events.
pub trait Logger {
    /// Log an informational message.
    fn info(&self, message: &str);
    /// Log a warning message.
    fn warn(&self, message: &str);
    /// Log an error message.
    fn error(&self, message: &str);
    /// Log a debug message.
    fn debug(&self, message: &str);
}
