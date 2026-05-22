<!--
  Sync Impact Report
  Version change: 1.0.0 → 2.0.0
  Modified principles:
    - I. Library-First → I. Zero-Cost Abstractions
    - II. CLI Interface → II. Strict Error Handling
    - III. Test-First (NON-NEGOTIABLE) → III. Type Safety by Construction
    - IV. Integration Testing → IV. Modular Crate Architecture
    - V. Observability & Simplicity → V. Test-First (NON-NEGOTIABLE)
  Added sections: None
  Removed sections: None
  Templates requiring updates:
    - .specify/templates/plan-template.md ✅ updated (no changes needed)
    - .specify/templates/spec-template.md ✅ updated (no changes needed)
    - .specify/templates/tasks-template.md ✅ updated (no changes needed)
    - .specify/templates/checklist-template.md ✅ updated (no changes needed)
  Follow-up TODOs: None
-->

# Foo Constitution

## Core Principles

### I. Zero-Cost Abstractions

All abstractions MUST compile down to efficient native code with no runtime
overhead beyond what is explicitly opted into. Favor generics, traits, and
monomorphization over dynamic dispatch. Boxing and trait objects MUST be
justified by measured need. Use `#[inline]` deliberately, not habitually.

**Rationale**: Zero-cost abstractions are Rust's founding promise. Every layer
of indirection that cannot be optimized away is technical debt against
performance.

### II. Strict Error Handling

All fallible operations MUST return proper `Result` or `Option` types. Panic
paths MUST be confined to unrecoverable states (e.g., hardware fault, corrupted
invariant) and documented accordingly. Use domain-error enums with `thiserror`;
reserve `anyhow` for binary/CLI entrypoints only. Raw `.unwrap()` and
`.expect()` are FORBIDDEN outside test code.

**Rationale**: Explicit error types make failure modes visible at the type
level. Hiding errors behind panics or stringly-typed exceptions defeats Rust's
reliability guarantees.

### III. Type Safety by Construction

Leverage Rust's type system to encode domain invariants at compile time. Use
newtypes for all primitives with semantic meaning (e.g., `struct UserId(u64)`
not bare `u64`). Prefer `enum` over stringly-typed values. Use `PhantomData`,
typestate patterns, and const generics to make illegal states unrepresentable.

**Rationale**: Every runtime assertion that could have been a compile-time
error is a preventable defect. Types are the most efficient test — they run at
compile time with zero runtime cost.

### IV. Modular Crate Architecture

The project MUST be organized as a Cargo workspace with clearly separated
crates by concern. Public API surfaces MUST be minimal and explicitly
documented. Enable `#![deny(missing_docs)]` on all public interfaces.
Dependency graphs MUST be acyclic at the crate level. New dependencies MUST be
justified and approved.

**Rationale**: Fine-grained crates enforce module boundaries at the build
level. Acyclic dependency graphs preserve compilation parallelism and prevent
spaghetti coupling.

### V. Test-First (NON-NEGOTIABLE)

TDD is mandatory. Tests MUST be written and approved by the user before any
implementation code. Tests MUST fail before implementation begins. The
Red-Green-Refactor cycle MUST be strictly enforced. Use `#[cfg(test)]` modules
inline with implementation.

**Rationale**: Test-first ensures requirements are understood before
implementation, creating a specification that prevents regression and documents
behavior.

## Additional Constraints

- **Technology Stack**: All code MUST compile on the latest stable Rust
  toolchain via `cargo`. New dependencies MUST be listed in `Cargo.toml` with
  explicit version requirements and justified in the implementation plan.
- **Security**: All user input MUST be validated and sanitized. Secrets MUST
  never be committed to version control. Use `#![forbid(unsafe_code)]` at crate
  level unless explicitly exempted in the implementation plan.
- **Performance**: Features MUST meet the performance goals defined in the
  implementation plan. Regression MUST be detected through automated benchmarks
  (`cargo bench`) where feasible.

## Development Workflow

- All code MUST go through the full pipeline: specification → plan → tasks →
  implementation → checklist.
- Code review is required for all changes. Reviews MUST verify constitution
  compliance.
- Testing gates: All tests MUST pass before merge (`cargo test`). Test coverage
  MUST not decrease.
- Commits SHOULD be atomic and reference the task ID. Run `cargo fmt` and
  `cargo clippy` before every commit.

## Governance

This constitution supersedes all other practices and informal conventions.
Amendments require:
1. A documented proposal describing the change and its rationale.
2. Approval from the project maintainers.
3. A migration plan for any existing code or practices affected by the change.

**Versioning Policy**: This document follows semantic versioning
(MAJOR.MINOR.PATCH).
- MAJOR: Backward incompatible governance/principle removals or redefinitions.
- MINOR: New principle/section added or materially expanded guidance.
- PATCH: Clarifications, wording, typo fixes, non-semantic refinements.

**Compliance Review**: All PRs and reviews MUST verify compliance with this
constitution. Complexity MUST be justified in the implementation plan's
Complexity Tracking section.

**Version**: 2.0.0 | **Ratified**: 2026-05-21 | **Last Amended**: 2026-05-22
