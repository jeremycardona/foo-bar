//! CLI logging facade implementation.
//!
//! Wires the core Logger trait at startup.

use foo_core::log::Logger;

/// Standard output logger for the CLI binary.
pub struct CliLogger;

impl Logger for CliLogger {
    fn info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    fn warn(&self, message: &str) {
        eprintln!("[WARN] {}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    fn debug(&self, message: &str) {
        println!("[DEBUG] {}", message);
    }
}
