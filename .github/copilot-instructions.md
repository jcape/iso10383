# ISO 10383 Codebase Guidelines

## Overview

This workspace provides Rust libraries for working with ISO 10383 Market Identifier Codes (MICs). It's a **monorepo workspace** with 4 interdependent crates that form a code generation pipeline from XML data to compile-time constants.

## Architecture & Data Flow

The codebase follows a **build-time code generation** pattern:

1. **`parser`** - Parses `ISO10383_MIC.xml` files from ISO 20022 using `quick-xml` and `serde`
2. **`macros`** - Procedural macro that consumes parser output at compile-time to generate enums and constants
3. **`static`** - Invokes `iso10383_macros::generate!()` to produce a `Code` enum and static records from embedded XML
4. **`types`** - Foundation crate providing `no_std` MIC string types (`Mic`, `&mic`) with validation

### Why this structure?

- **Types** must be `no_std` and dependency-light for embedded/embedded use
- **Parser** needs `std` and `serde` but only runs at build time
- **Macros** bridge compile-time XML parsing with runtime constants
- **Static** provides the actual data consumers use, generated at compile time

## Key Conventions

### Crate Dependencies

- **Never** make `types` depend on `parser` or `static` - it's the foundation
- `static` → `macros` → `parser` → `types` (strict dependency hierarchy)
- All crates share workspace-level versions (`version = "0.3.1"` in root `Cargo.toml`)

### XML Data Handling

- XML files live in `static/src/ISO10383_MIC.xml` (embedded) and `parser/tests/*.xml` (test snapshots)
- Field names in XML use encoded spaces: `MARKET_x0020_NAME` → use `#[serde(alias = "...")]`
- Proc macro reads XML from `$CARGO_MANIFEST_DIR/src/` - path must be relative to `src/`

### Code Generation Pattern

The `macros` crate transforms XML into:

```rust
// Generated in static/src/lib.rs via:
iso10383_macros::generate!(xml = "ISO10383_MIC.xml");

// Produces:
pub enum Code { XNAS, XNYS, /* ... */ }
pub const XNAS: Record = /* ... */;
```

Generated identifiers follow **TitleCase** via `heck::ToTitleCase` for enum variants.

### no_std Support

- `types` and `static` are `#![no_std]` by default
- Use `#[cfg(feature = "alloc")]` for heap-requiring features
- Feature flags: `"alloc"`, `"serde"`

## Critical Workflows

### Running Tests

```fish
cargo test --workspace --all-features
```

Tests in `parser/tests/all.rs` use parameterized tests (`yare` crate) to validate parsing of historical XML snapshots.

### Linting & Formatting

```fish
cargo fmt --all -- --check
cargo clippy --all --all-features -- -D warnings
```

Workspace lints are **extremely strict** (`Cargo.toml` sets `pedantic = "deny"`). All clippy warnings are errors in CI.

### Pre-commit Checks

```fish
pre-commit run --all-files
```

Configured in `.pre-commit-config.yaml`:

- Conventional commits enforced (commit-msg hook)
- `cargo-deny` for license/advisory checks
- `markdownlint-cli2` and `yamllint` for docs

### Building Documentation

```fish
cargo doc --workspace --all-features --no-deps
```

All public items **must** have doc comments (`missing_docs = "deny"`).

### Updating XML Data

1. Download latest from <https://www.iso20022.org/market-identifier-codes>
2. Replace `static/src/ISO10383_MIC.xml`
3. Add snapshot to `parser/tests/YYYY-MM-DD.xml`
4. Add test case to `parser/tests/all.rs` with expected record count
5. Rebuild: `cargo build -p iso10383-static`

## Important Constraints

- **Rust Edition**: 2024 (set in workspace)
- **MSRV**: 1.92.0 (enforced in `Cargo.toml`)
- **No `unsafe` code** allowed (`unsafe_code = "deny"`)
- **No public APIs without docs** (`missing_docs = "deny"`)
- License: Apache-2.0 only (enforced by `deny.toml`)

## Common Pitfalls

1. **Don't add `std` to `types`** - breaks embedded use cases
2. **XML path in proc macro** must be relative to `src/`, not crate root
3. **Serde aliases** are mandatory - XML has escaped field names
4. **Empty XML tags** like `<LEI></LEI>` deserialize as `Some("")` not `None` - handle with custom logic
5. **Workspace versions** - use exact versions (`=0.3.1`) for internal deps to keep lockstep releases

## External Dependencies

- `chrono` - Date parsing (no default features in `types`)
- `quick-xml` - XML parsing with serde support
- `iso3166-static` - Country code validation
- `iso17442-types` - LEI (Legal Entity Identifier) types
- `ref-cast` - Zero-cost newtype conversions for `&mic` slice type

## Release Process

Managed by `release-plz` (see `.github/workflows/release-plz.yml`):

- Shared version across workspace (`shared-version = true`)
- Tag format: `v{{version}}`
- Automated changelog from conventional commits
