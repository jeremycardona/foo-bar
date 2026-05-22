# Feature Specification: Core Architecture Definition

**Feature Branch**: feature/001-core-architecture
**Created**: 2026-05-22
**Status**: Draft
**Input**: define core architecture

## User Scenarios & Testing

### User Story 1 — Crate Workspace Layout (Priority: P1)

As a developer, I want a clearly defined Cargo workspace with separated crates
so that I know where to place new code and how modules relate to each other.

**Why this priority**: The workspace layout is the foundation — all subsequent
development depends on crate boundaries being established first.

**Independent Test**: A developer can list all workspace members via
`cargo metadata` and describe each crate's purpose from its documentation.

**Acceptance Scenarios**:

1. **Given** the workspace is initialized, **When** a developer runs
   `cargo metadata`, **Then** the output lists all workspace members with
   their source paths.
2. **Given** a workspace member crate, **When** inspected, **Then** its
   `Cargo.toml` declares the crate type (lib/bin) and a one-line description.
3. **Given** the workspace, **When** a developer clones the repo fresh and runs
   the build, **Then** all crates compile without errors.
4. **Given** an empty or minimal crate, **When** added to the workspace,
   **Then** the existing build remains unaffected — no breaking changes
   introduced by workspace membership alone.

---

### User Story 2 — Data Flow Patterns (Priority: P1)

As a developer, I want documented data flow patterns so that I know how data
moves between crate boundaries consistently.

**Why this priority**: Consistent data flow prevents architectural drift and
makes cross-crate changes predictable.

**Independent Test**: A reviewer can trace a feature's data path from input to
output across crate boundaries using only the architecture docs.

**Acceptance Scenarios**:

1. **Given** a feature that spans multiple crates, **When** the data flow is
   traced, **Then** the path follows the documented pattern (input → domain
   logic → output).
2. **Given** two adjacent crates, **When** one calls into the other,
   **Then** the call goes through a defined public API — no internal modules
   are accessed across crate boundaries.
3. **Given** the architecture documentation, **When** a new developer reads
   it, **Then** they can describe the data flow direction for any feature
   without reading source code.

---

### User Story 3 — Error Boundary Conventions (Priority: P2)

As a developer, I want consistent error handling conventions across crate
boundaries so that errors propagate correctly and are surfaced appropriately.

**Why this priority**: Errors that cross crate boundaries silently or lose
context undermine reliability.

**Independent Test**: A static analysis pass can verify that every public
function returns a well-defined error type rather than a generic catch-all.

**Acceptance Scenarios**:

1. **Given** a public function in a library crate, **When** it can fail,
   **Then** its return type includes a domain-specific error enum (not
   `Box<dyn Error>` or raw `String`).
2. **Given** an error that crosses a crate boundary, **When** propagated,
   **Then** it preserves the original context via error wrapping.
3. **Given** the CLI entrypoint, **When** an error reaches the top level,
   **Then** it is rendered as a user-facing message to stderr with exit code
   non-zero.

## Requirements

### Functional Requirements

- **FR-001**: The project MUST be organized as a Cargo workspace with at least
  three crate types: a library crate for core domain logic, a binary crate for
  the CLI interface, and a shared-types crate for cross-crate DTOs.
- **FR-002**: Each workspace member MUST declare its dependencies explicitly in
  its own `Cargo.toml` — no workspace-level dependency inheritance for public
  API deps.
- **FR-003**: Inter-crate data flow MUST follow a unidirectional pattern:
  input enters at the boundary, flows through domain logic, and exits as
  output. Bidirectional coupling between sibling crates is FORBIDDEN.
- **FR-004**: Every public fallible function in a library crate MUST return a
  custom error enum (deriving `thiserror::Error`) specific to that crate's
  domain.
- **FR-005**: Error types MUST implement conversion into the adjacent crate's
  error type at the boundary, not earlier.
- **FR-006**: The shared-types crate MUST NOT depend on any other workspace
  member. It MAY depend on std and well-known crates (e.g., `serde`,
  `thiserror`).
- **FR-007**: Each library crate MUST define a minimal logging facade
  (trait or thin wrapper) for its own domain events. A single runtime
  implementation in the binary crate wires all facades together at startup.
- **FR-008**: All workspace member crates MUST share a single version number.
  The workspace version is defined at the workspace root and inherited by all
  members.
- **FR-009**: The workspace MUST include a dedicated integration test crate
  (`tests/`) that tests all library crates together as an external consumer
  would. Individual crates MAY also contain inline `#[cfg(test)]` unit tests.

### Key Entities

- **Workspace Member (Crate)**: A Cargo package within the workspace. May be
  `lib` (library) or `bin` (binary). Each has a defined public API surface and
  a single responsibility.
- **Crate Boundary**: The public API surface of a crate — its exported types,
  functions, and traits. All cross-crate interaction MUST go through this
  surface.
- **Domain Error**: A `thiserror`-derived enum representing all failure modes
  within a crate's domain. Maps to an exit code and user-facing message at the
  CLI boundary.
- **Shared DTO**: A type in the shared-types crate that crosses crate
  boundaries. MUST be `#[derive(Debug, Clone, PartialEq)]` and
  `#[derive(Serialize, Deserialize)]` if serialization is needed.

## Clarifications

### Session 2026-05-22

- Q: What should be explicitly excluded from the architecture definition? → A: Core patterns scope — crate layout, data flow, error handling, and testing conventions only.
- Q: How should observability signals flow across crate boundaries? → A: Per-crate facade — each crate defines its own minimal logging abstraction; a common runtime wires them together at the binary entrypoint.
- Q: What build/compile time targets should the architecture optimize for? → A: Correctness first — crate boundaries are domain-driven; no build time targets imposed.
- Q: How should internal crate versioning work within the workspace? → A: Lockstep — all workspace crates share a single version number; released together.
- Q: How should tests be organized across workspace crates? → A: External test crate — a dedicated `tests/` workspace member tests all crates together as an external consumer would.

## Out of Scope

The following concerns are explicitly excluded from this architecture
definition and will be addressed in separate features:
- Deployment topology and hosting infrastructure
- CI/CD pipeline configuration
- Monitoring and alerting infrastructure
- Release process and package distribution

## Assumptions

- The project build system uses a standard Rust toolchain; no custom build
  scripts or non-Cargo tooling is required for the workspace to compile.
- Developers are familiar with basic Rust workspace and module conventions.
- The workspace will grow incrementally — initial crates may be minimal and
  expanded as features are added.
- Cross-crate error conversion follows the `From` trait pattern rather than
  manual mapping.
- Crate boundaries are driven by domain logic, not build time optimization.
  Build performance tuning is out of scope for this architecture definition.

## Success Criteria

- **SC-001**: A new developer can identify which crate to modify for a given
  feature in under 5 minutes by reading the workspace layout.
- **SC-002**: Cross-crate data flow follows the documented unidirectional
  pattern for all features — verified by architecture review.
- **SC-003**: Every public fallible function returns a domain-specific error
  type — verified by documentation inspection and/or automated lint rules.
- **SC-004**: The full workspace compiles with zero warnings — verified by the
  build system with a zero-warnings policy.
- **SC-005**: Error messages at the CLI boundary are user-friendly and include
  actionable information (not raw debug output or panic traces).
