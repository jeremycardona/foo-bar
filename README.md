# foo

IF youre able to read: this all happpens in the foo folder

A modular Rust project organized as a Cargo workspace with separated crates for
core domain logic, a CLI interface, and shared types.

## Architecture

- **`core/`** — Library crate for core domain logic
- **`shared/`** — Library crate for cross-crate DTOs and shared types
- **`cli/`** — Binary crate for the CLI entrypoint
- **`tests/`** — Integration test crate exercising all crates as an external consumer

Data flows unidirectionally: CLI input → core domain logic → CLI output.

## Prerequisites

- Rust toolchain (latest stable) — install via `rustup`

## Quickstart

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

## Principles

- Zero-cost abstractions — static dispatch across crate boundaries
- Strict error handling — domain error enums with `From` conversion at boundaries
- Type safety — compile-time type agreement via shared types crate
- Test-first — integration tests from day one

## License

i dont even know. but here is some foundations to build.
