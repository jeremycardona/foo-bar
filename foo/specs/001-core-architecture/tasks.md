---

description: "Task list for Core Architecture Definition"
---
# Tasks: Core Architecture Definition

## Phase 1: Setup (Shared Infrastructure)

- [X] T001 Create root `Cargo.toml` with `[workspace]` definition, `workspace.package` version, and member list at `Cargo.toml`
- [X] T002 [P] Create `core/` crate directory with `Cargo.toml` (lib) and `src/lib.rs`
- [X] T003 [P] Create `shared/` crate directory with `Cargo.toml` (lib) and `src/lib.rs`
- [X] T004 [P] Create `cli/` crate directory with `Cargo.toml` (bin) and `src/main.rs`
- [X] T005 [P] Create `tests/` crate directory with `Cargo.toml` (lib) and `tests/smoke.rs`

## Phase 2: Foundational

- [X] T006 Configure workspace-level dependency declarations for `serde` (with `derive` feature) and `thiserror` in root `Cargo.toml`
- [X] T007 Add `shared` crate deps (`serde`, `thiserror`) inheriting from workspace in `shared/Cargo.toml`

## Phase 3: User Story 1 — Crate Workspace Layout (P1)

- [X] T008 [US1] Configure `core/Cargo.toml` with `foo-shared` dependency (path dep) and `version.workspace = true`
- [X] T009 [US1] Configure `cli/Cargo.toml` with `foo-core` and `foo-shared` path deps, `version.workspace = true`
- [X] T010 [US1] Configure `tests/Cargo.toml` with workspace crate path deps, `version.workspace = true`
- [X] T011 [US1] Add `#![deny(unsafe_code)]` and `#![deny(missing_docs)]` to all library `lib.rs` files
- [X] T012 [US1] Verify `cargo metadata` lists all 4 workspace members; verify `cargo build` passes

## Phase 4: User Story 2 — Data Flow Patterns (P1)

- [X] T013 [US2] Define `Logger` trait in `core/src/log.rs` with info/error/warn methods
- [X] T014 [US2] [P] Create `core/src/lib.rs` public API skeleton documenting unidirectional flow (input → domain → output)
- [X] T015 [US2] Implement `CliLogger` in `cli/src/log.rs` wiring all facades at startup in `cli/src/main.rs`

## Phase 5: User Story 3 — Error Boundary Conventions (P2)

- [X] T016 [US3] Define `CoreError` enum in `core/src/error.rs` with domain variants
- [X] T017 [US3] Define `CliError` enum in `cli/src/error.rs` with domain variants
- [X] T018 [US3] Implement `From<CoreError> for CliError` conversion in `cli/src/error.rs`
- [X] T019 [US3] Implement CLI error rendering with user-friendly messages and exit codes in `cli/src/main.rs`

## Phase 6: Polish & Cross-Cutting

- [X] T020 Configure root `clippy.toml` with shared lint config (if needed)
- [X] T021 Verify `cargo clippy --all-targets -- -D warnings` passes with zero warnings
- [X] T022 Verify `cargo test` passes on all crates and `tests/smoke.rs` runs

---

## Execution Notes

- Dependency order: Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6
- Parallel opportunities: T002-T005 (Phase 1), T008-T011 (Phase 3), T013-T014 (Phase 4), T021-T022 (Phase 6)
- Suggested MVP: Phase 1–3 (workspace compiling with all crates)
- Independent test per story:
  - US1: `cargo metadata` shows 4 members; `cargo build` compiles cleanly
  - US2: Data flow documented in `core/src/lib.rs`; logger trait defined
  - US3: Error enums defined with thiserror; `cargo build` compiles cleanly
