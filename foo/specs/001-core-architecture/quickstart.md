# Quickstart: Core Architecture

## Prerequisites

- Rust toolchain (latest stable) — install via `rustup`
- No other tooling required

## Setup

1. Clone the repository:

   ```bash
   git clone <repo-url>
   cd <repo-name>
   ```

2. Build the entire workspace:

   ```bash
   cargo build
   ```

3. Run all tests:

   ```bash
   cargo test
   ```

4. Run linting:

   ```bash
   cargo clippy --all-targets -- -D warnings
   ```

## Workspace Members

| Crate | Path | Purpose |
|-------|------|---------|
| `core` | `core/` | Core domain logic library |
| `cli` | `cli/` | CLI binary — entrypoint |
| `shared` | `shared/` | Shared DTOs across crate boundaries |
| `tests` | `tests/` | Cross-crate integration tests |

## Adding a New Crate

1. Create the crate directory with `Cargo.toml` and `src/lib.rs`
2. Add the crate path to the `[workspace.members]` list in root `Cargo.toml`
3. Run `cargo build` to verify it compiles

## Development Workflow

1. Write a failing test (`#[cfg(test)]` inline or in `tests/`)
2. Implement until tests pass
3. Run `cargo clippy --all-targets -- -D warnings`
4. Run `cargo test`
5. Commit
