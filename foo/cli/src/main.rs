#![allow(dead_code)]

//! CLI entrypoint.
//!
//! This binary crate wires all facades and starts the application.
//! Data flows: CLI input → core domain logic → CLI output.
//!
//! Error handling:
//! - All errors are rendered as user-facing messages to stderr.
//! - Exit code non-zero on failure.

mod error;
mod log;

use std::process;
use log::CliLogger;
use foo_core::log::Logger;

fn main() {
    let logger = CliLogger;
    logger.info("application starting");

    let version = env!("CARGO_PKG_VERSION");
    println!("foo v{}", version);

    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(exit_code(&err));
    }

    logger.info("application finished");
}

/// Application entry point that can return an error.
fn run() -> Result<(), error::CliError> {
    // Architecture placeholder — feature commands will be added here.
    Ok(())
}

/// Map a CLI error to a POSIX exit code.
fn exit_code(err: &error::CliError) -> i32 {
    match err {
        error::CliError::Domain(_) => 1,
        error::CliError::InvalidArgs(_) => 2,
        error::CliError::Internal(_) => 1,
    }
}
