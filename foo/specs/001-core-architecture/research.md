# Research: Core Architecture Definition

## Architecture Decisions

### Decision 1: Crate Workspace Layout

- **Decision**: Three initial crates — `core` (library), `cli` (binary), `shared` (library DTOs) — plus a `tests/` integration test crate.
- **Rationale**: Clean separation of domain logic from CLI concerns; shared DTO crate prevents circular dependencies between library crates.
- **Alternatives considered**:
  - Monolithic single crate — rejected due to lack of boundary enforcement
  - Two-crate layout (lib + bin only) — rejected because shared types would need to live in one of them, creating unwanted coupling

### Decision 2: Data Flow Pattern

- **Decision**: Unidirectional flow — input enters at `cli`, flows through `core` domain logic, outputs via `cli`.
- **Rationale**: Simplifies reasoning about data paths; prevents circular coupling between sibling crates.
- **Alternatives considered**:
  - Bidirectional — rejected as it creates implicit coupling and makes testing harder

### Decision 3: Error Handling Convention

- **Decision**: Per-crate `thiserror` enums with `From` conversion at boundaries.
- **Rationale**: Each crate owns its error domain; boundary conversion preserves context without leaking internal error types.
- **Alternatives considered**:
  - Single global error type — rejected as it couples all crates to a shared error definition
  - `Box<dyn Error>` — rejected per constitution principle II (Strict Error Handling)

### Decision 4: Observability Pattern

- **Decision**: Per-crate logging facade (trait/thin wrapper); binary crate wires all facades at startup.
- **Rationale**: Library crates remain testable without a runtime logger; the binary is the single wiring point.
- **Alternatives considered**:
  - Shared tracing crate — rejected in favor of lighter per-crate facades
  - Boundary-only instrumentation — rejected as too limiting for debugging

### Decision 5: Versioning

- **Decision**: Lockstep versioning — all workspace crates share one version.
- **Rationale**: Simpler release management; since all crates evolve together for this project, independent versioning adds overhead without benefit.
- **Alternatives considered**:
  - Independent semver — rejected; adds complexity without need

### Decision 6: Test Organization

- **Decision**: External `tests/` crate for integration tests; inline `#[cfg(test)]` for unit tests.
- **Rationale**: Integration tests exercise crates together as an external consumer would; unit tests stay colocated with code.
- **Alternatives considered**:
  - Per-crate integration tests — rejected; wouldn't test cross-crate integration as an external consumer
