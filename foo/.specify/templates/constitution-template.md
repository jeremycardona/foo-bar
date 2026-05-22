# Foo Constitution

## Core Principles

### I. Library-First

Every feature MUST start as a standalone library. Libraries MUST be self-contained,
independently testable, and documented. A clear purpose is required — no
organizational-only libraries are permitted.

**Rationale**: Library-first architecture maximizes reusability, testability, and
separation of concerns. Organizational-only libraries create coupling without
value.

### II. CLI Interface

Every library MUST expose its functionality via a CLI. All I/O MUST follow the
text in/out protocol: stdin/args → stdout, errors → stderr. Both JSON and
human-readable output formats MUST be supported.

**Rationale**: CLI interfaces enable composability, scripting, and simple
debugging. Text protocols are universally portable.

### III. Test-First (NON-NEGOTIABLE)

TDD is mandatory. Tests MUST be written and approved by the user before any
implementation code. Tests MUST fail before implementation begins. The
Red-Green-Refactor cycle MUST be strictly enforced.

**Rationale**: Test-first ensures requirements are understood before
implementation, creating a specification that prevents regression and documents
behavior.

### IV. Integration Testing

Integration tests are REQUIRED for: new library contract tests, contract changes,
inter-service communication, and shared schema changes. Unit tests alone are
insufficient for verifying system boundaries.

**Rationale**: Integration tests catch interface mismatches and contract
violations that unit tests cannot detect.

### V. Observability & Simplicity

All systems MUST use structured logging. Text I/O ensures debuggability.
Complexity MUST be justified — apply YAGNI principles. Start simple and add
complexity only when proven necessary.

**Rationale**: Observability enables debugging in production. Simplicity reduces
maintenance burden and defect surface area.

## Additional Constraints

- **Technology Stack**: MUST be compatible with the project's established
  language and framework choices. New dependencies MUST be justified and
  approved.
- **Security**: All user input MUST be validated and sanitized. Secrets MUST
  never be committed to version control.
- **Performance**: Features MUST meet the performance goals defined in the
  implementation plan. Regression MUST be detected through automated benchmarks
  where feasible.

## Development Workflow

- All code MUST go through the full pipeline: specification → plan → tasks →
  implementation → checklist.
- Code review is required for all changes. Reviews MUST verify constitution
  compliance.
- Testing gates: All tests MUST pass before merge. Test coverage MUST not
  decrease.
- Commits SHOULD be atomic and reference the task ID.

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

**Version**: [VERSION] | **Ratified**: [RATIFICATION_DATE] | **Last Amended**: [LAST_AMENDED_DATE]

---