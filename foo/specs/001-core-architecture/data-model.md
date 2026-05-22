# Data Model: Core Architecture Definition

## Entities

### Workspace Member (Crate)

```
struct Crate {
    name: String,                   // package name (e.g., "foo-core")
    path: PathBuf,                  // relative workspace path (e.g., "core/")
    kind: CrateKind,                // Lib | Bin
    description: String,            // one-line purpose from Cargo.toml
    public_api: Vec<ExportedItem>,  // pub items exposed at crate boundary
}
```

**Validation rules**:
- `name` MUST be unique across all workspace members
- `kind` determines whether Cargo.toml has `[lib]` or `[[bin]]` section
- `public_api` MUST be explicitly documented; no `pub(crate)` items visible externally

**Relationships**:
- A workspace contains 1..N crates
- `shared` crate depends on 0 external crates (std only + serde/thiserror)
- `core` crate depends on `shared` (for DTOs)
- `cli` crate depends on `core` and `shared`
- `tests` crate depends on `core`, `shared`, `cli`

### Crate Boundary

```
struct Boundary {
    source: Crate,
    exported_types: Vec<TypeDef>,
    exported_traits: Vec<TraitDef>,
    exported_functions: Vec<FnDef>,
    error_enum: Option<ErrorEnum>,  // domain error for fallible operations
}
```

**Validation rules**:
- All cross-crate access MUST go through `exported_*` items
- No `pub(crate)` items from one crate accessible from another
- `error_enum` MUST exist if any fallible function is exported

### Domain Error

```
struct DomainError {
    crate_origin: String,           // which crate defined this error
    variants: Vec<ErrorVariant>,    // domain-specific failure cases
    display_message: String,        // user-facing message at CLI boundary
    exit_code: u8,                  // non-zero exit code for CLI
}
```

**Validation rules**:
- MUST derive `thiserror::Error` and `std::fmt::Display`
- MUST implement `From<T>` for adjacent crate error types
- MUST NOT contain raw `String` error messages — use typed variants

### Shared DTO

```
struct SharedDto {
    name: String,
    fields: Vec<Field>,
    derives: Vec<String>,  // e.g., Debug, Clone, PartialEq, Serialize, Deserialize
}
```

**Validation rules**:
- MUST derive `Debug`, `Clone`, `PartialEq`
- MUST derive `Serialize` + `Deserialize` if serialization crosses crate boundary
- MUST NOT depend on any workspace member crate
- MUST be defined in the `shared` crate only

## State Transitions

### Crate Lifecycle

```
Planned → Active → Deprecated → Removed
```

- **Planned**: Declared in workspace `Cargo.toml` members list but minimal
  `src/lib.rs` only (single `// TODO` comment permitted)
- **Active**: Full implementation with tests and documentation
- **Deprecated**: Marked with `#[deprecated]` on all public API items; no new
  features added
- **Removed**: Removed from workspace members; any remaining references broken
  in a MAJOR version bump
