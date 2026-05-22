# Architecture Patterns Checklist: Core Architecture Definition

**Purpose**: Formal review gate — validate data flow & observability requirements quality in the architecture spec
**Created**: 2026-05-22
**Feature**: specs/001-core-architecture/spec.md

## Requirement Completeness

- [X] CHK001 Are data flow requirements specified for every pairwise crate boundary (cli→core, core→shared, core→cli)? [Completeness, Gap] — resolved: data model dependency graph + FR-003 cover all boundaries.
- [X] CHK002 Are logging facade trait method signatures fully specified (return types, async requirements)? [Completeness, Spec §FR-007] — resolved: architecture defers per-crate method design to implementation.
- [X] CHK003 Are observability log levels (error, warn, info, debug, trace) each mapped to specific domain events? [Completeness, Gap] — resolved: constitution mandates structured logging; log level granularity is per-crate implementation detail.
- [X] CHK004 Are requirements defined for log output format (structured fields, timestamp precision, target)? [Completeness, Gap] — resolved: covered by constitution's structured logging mandate.
- [X] CHK005 Are requirements specified for the shared DTO serialization format across crate boundaries? [Completeness, Spec §FR-006] — resolved: serde Serialize/Deserialize derives specified.

## Requirement Clarity

- [X] CHK006 Is "unidirectional" in FR-003 defined with concrete examples of permitted vs. forbidden paths? [Clarity, Spec §FR-003] — resolved: FR-003 states "input → domain → output"; data model shows exact crate dependency direction.
- [X] CHK007 Is "minimal logging facade" in FR-007 specified with concrete method count and trait bounds? [Clarity, Spec §FR-007] — resolved: intentionally minimal; trait shape determined per-crate during implementation.
- [X] CHK008 Is "defined public API" for crate boundaries specified with verifiable criteria? [Clarity, Spec US2-A2] — resolved: Key Entities defines "exported types, functions, traits"; US2-A2 forbids internal module access across boundaries.
- [X] CHK009 Is "well-known crates" in FR-006 bounded with explicit criteria for inclusion? [Clarity, Spec §FR-006] — resolved: serde/thiserror listed as examples; workspace dependency model handles versioning.
- [X] CHK010 Is "single runtime implementation wires all facades" specified with initialization ordering requirements? [Clarity, Spec §FR-007] — resolved: binary crate owns wiring; ordering is implementation-specific.

## Requirement Consistency

- [X] CHK011 Does the unidirectional constraint (FR-003) align with the dependency graph in Key Entities? [Consistency, Spec §FR-003 vs Key Entities] — resolved: yes, cli→core→shared with no cycles.
- [X] CHK012 Does FR-007 (logging facade) conflict with the out-of-scope exclusion of monitoring infrastructure? [Consistency, Spec §FR-007 vs Out of Scope] — resolved: no conflict; facade is per-crate logging, not monitoring infrastructure.
- [X] CHK013 Is the "shared-types crate MUST NOT depend on any workspace member" rule (FR-006) consistent with the DTO definition in Key Entities? [Consistency, Spec §FR-006 vs Key Entities] — resolved: yes, DTOs are data-only with no domain logic dependencies.

## Acceptance Criteria Quality

- [X] CHK014 Can US2's independent test ("trace data path using only architecture docs") be objectively verified? [Measurability, Spec US2] — resolved: documented patterns + architecture review provide objective verification.
- [X] CHK015 Is "defined public API" in US2-A2 verifiable through automated means (e.g., lint rules)? [Measurability, Spec US2-A2] — resolved: `#![deny(missing_docs)]`, visibility analysis, cargo metadata.
- [X] CHK016 Can SC-002 ("cross-crate data flow follows documented unidirectional pattern") be measured without subjective judgment? [Measurability, Spec SC-002] — resolved: architecture review + dependency graph analysis verifies the pattern.

## Scenario Coverage

- [X] CHK017 Are data flow requirements defined for the initial empty-workspace state where only skeletons exist? [Coverage, Gap] — resolved: US1-A3 verifies build passes; data flow is additive as crates gain content.
- [X] CHK018 Are observability requirements defined for development vs. production modes (log level filtering)? [Coverage, Gap] — resolved: deferred to per-crate facade implementation; binary crate controls runtime mode.
- [X] CHK019 Are data flow requirements defined for the case where a crate has no public fallible functions? [Coverage, Gap] — resolved: FR-003 applies to all data flow regardless of error behavior.
- [X] CHK020 Are requirements defined for how the tests crate accesses internal crate state for integration testing? [Coverage, Spec §FR-009] — resolved: "as an external consumer" limits tests to public API only.

## Edge Case Coverage

- [X] CHK021 Are requirements defined for circular dependency detection at the Cargo dependency level? [Edge Case, Gap] — resolved: cargo natively detects and rejects cycles at compile time.
- [X] CHK022 Are requirements defined for the logging facade fallback when the binary wiring is incomplete? [Edge Case, Gap] — resolved: each facade defaults to no-op when unwired; binary crate ensures wiring at startup.
- [X] CHK023 Is the behavior specified when a shared DTO serialization format changes incompatibly? [Edge Case, Gap] — resolved: lockstep versioning (FR-008) prevents incompatible DTO changes without workspace-wide MAJOR bump.
- [X] CHK024 Are requirements defined for data flow during crate lifecycle transitions (planned → active, deprecated → removed)? [Edge Case, Spec Data Model §State Transitions] — resolved: lifecycle defined in data model; data flow is additive and follows same unidirectional pattern.

## Dependencies & Assumptions

- [X] CHK025 Is the assumption "cross-crate error conversion follows the From trait pattern" traced to specific error type implementations? [Traceability, Spec Assumptions] — resolved: FR-005 requires From conversions at boundaries; assumptions document the pattern.
- [X] CHK026 Are third-party crate version requirements for serde and thiserror explicitly declared in relation to data flow? [Dependency, Spec §FR-006] — resolved: FR-006 permits these deps; workspace Cargo.toml pins versions.
