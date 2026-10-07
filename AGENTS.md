# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Rust workspace of crates for ISO 10383 Market Identifier Codes (MICs). All four crates share one version (`[workspace.package]` in the root `Cargo.toml`) and are released together via `release-plz` with `v{{version}}` tags; internal deps are pinned with exact versions (`=x.y.z`). Commit messages must follow Conventional Commits (enforced by a `commit-msg` hook) since changelogs are generated from them.

## Commands

```bash
cargo build --workspace --all-features
cargo test --workspace --all-features
cargo test -p iso10383-parser --test all all_historical::sep2026   # single parameterized case
cargo clippy --all --all-features -- -D warnings
cargo fmt --all -- --check
cargo doc --workspace --all-features --no-deps
cargo deny --all-features check
prek run --all-files        # git hooks (prek.toml); replaces the old .pre-commit-config.yaml
```

CI (`.github/workflows/ci.yml`) additionally builds every crate with `--no-default-features` and each of `alloc`, `serde`, `serde,alloc` on MSRV (1.88.0), stable, and beta, so feature-gated code must compile in all combinations. `RUSTFLAGS`/`RUSTDOCFLAGS` are `-D warnings` in CI.

## Architecture

The crates form a compile-time code generation pipeline from the ISO-published `ISO10383_MIC.xml` to static Rust data:

- **`types`** (`iso10383-types`) — `#![no_std]` foundation: the owned `Mic` / borrowed `mic` (unsized, `ref-cast`) code types with `const` validation, plus the `Kind`, `Category`, `Status` enums. Optional `alloc`, `serde`, `zerocopy` features. Must never depend on the other crates or `std`.
- **`parser`** (`iso10383-parser`) — `std` + `serde` model of the XML (`MicList`, `MicRecord`). XML field names contain encoded spaces (e.g. `MARKET_x0020_NAME`), handled with `#[serde(alias = ...)]`.
- **`macros`** (`iso10383-macros`) — proc macro `generate!(xml = "...", zerocopy = ...)`. Resolves `xml` relative to the *calling crate's* `src/` dir (`CARGO_MANIFEST_DIR/src`), parses it with `parser` via `quick-xml`, collects columns in `xml/recordset.rs` (`RecordSet`), and emits the `Code` enum, per-MIC record constants, and an `Error` type. `zerocopy` takes a bool or the feature name to gate zerocopy derives on.
- **`static`** (`iso10383-static`) — `#![no_std]`; its entire `lib.rs` is a single `generate!` invocation over the embedded `static/src/ISO10383_MIC.xml`, plus hand-written serde impls in `_serde.rs`.

Dependency direction: `static` → `macros` → `parser` → `types`.

### Updating MIC data

1. Replace `static/src/ISO10383_MIC.xml` with the new release.
2. Add a copy as `parser/tests/YYYY-MM-DD.xml`.
3. Add a case to the `yare::parameterized` list in `parser/tests/all.rs` with the expected record count.

These XML files are excluded from the whitespace/format hooks; don't hand-edit them.

## Lints and style

Workspace lints (root `Cargo.toml`) are very strict — clippy `pedantic` and many restriction lints are `deny`. Notable consequences:

- Every item, including private ones, needs a doc comment (`missing_docs`, `missing_docs_in_private_items`); doc paragraphs must end with punctuation.
- Public functions need `#[inline]` (`missing_inline_in_public_items`); every type needs `Debug`.
- `allow_attributes` is denied: use `#[expect(...)]` instead of `#[allow(...)]`. `unsafe_code` is denied workspace-wide; the rare exception uses `#[expect(unsafe_code)]` with a `// SAFETY:` comment.
- No `unwrap()`, `todo!`, `print!`/`eprint!`, `dbg!`, absolute paths (`std::foo::bar()` inline), or `mod.rs` files (`mod_module_files`).
- Module-local feature modules are named with a trailing underscore (`alloc_.rs`, `serde_.rs`) or leading underscore (`_serde.rs`) to avoid clashing with crate names.

`CONTRIBUTING.md` adds project-specific conventions:

- Order top-of-file items: `extern crate`, `pub use`, `pub mod`, `mod`, then `use`.
- Re-export external types that appear in public APIs (or wrap them in a newtype); export types at the crate root; group related free functions in a `pub mod`.
- No `use *` outside tests; prefer `{}` scopes over `drop()` for RAII guards.

## Known inconsistencies

- `.github/copilot-instructions.md` is partly stale (it cites version 0.3.1, MSRV 1.92, and `.pre-commit-config.yaml`); trust `Cargo.toml` and `prek.toml`.
- The `cargo-nono` hook in `prek.toml` and the `CONTRIBUTING.md` title reference `iso17442` (copied from a sibling project) rather than `iso10383-types`.
