#![deny(unsafe_code)]
#![deny(missing_docs)]

//! Core domain logic crate.
//!
//! # Data Flow
//!
//! This crate follows a unidirectional data flow pattern:
//!
//! 1. **Input** — Data enters through the CLI boundary (`cli` crate).
//! 2. **Domain** — Core domain logic processes the input here.
//! 3. **Output** — Results are returned to the CLI for presentation.
//!
//! ## Crate Dependencies
//!
//! - `foo-shared` — Shared DTOs for cross-crate data exchange.
//!
//! ## Public API
//!
//! Everything exported via `pub` is part of the crate boundary.
//! Internal modules use `pub(crate)` to prevent cross-crate access.

pub mod error;
pub mod log;
