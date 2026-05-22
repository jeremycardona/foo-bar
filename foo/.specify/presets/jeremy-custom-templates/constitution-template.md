# Gas Smash Constitution

## Core Principles

### I. Pure Rust (Zero JS)

No node_modules. No package.json. If it doesn't compile via cargo, it doesn't exist. Use wasm-bindgen only where browser APIs are required.

### II. Unified Type-Space

The Backend and Frontend MUST share the same dto (Data Transfer Object) crate. A change in the Bitcoin data model must break the frontend build immediately to ensure type safety.

### III. Demo-First Performance

Latency from Alchemy to the UI must be minimized. Use tokio for async data fetching.

## Governance

- **The Timer**: With < 3 hours left, YAGNI (You Ain't Gonna Need It) is the law. No database; keep state in memory.
    

**Version**: 1.0.0 | **Ratified**: [Current Time]

---