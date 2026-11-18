# Specification: Serde and YAML boundary qualification (18 November 2026)

**Status:** proposed; source changes not authorised by this planning-only track.  
**Owner:** Fe Reader Conductor integration owner.  
**Scope:** Rust `crates/xtask`, not the PDF parser, renderers or Python/TypeScript tooling.

## Verified source inventory
- Root `Cargo.toml` already declares `serde` with derive and `serde_json` for typed metadata, JSON contracts and receipts. This is appropriate; Serde is not a replacement for `lopdf`, XML, JSON Schema or PDF semantic validation.
- `crates/xtask/Cargo.toml` declares `serde_yaml = "0.9"`. `crates/xtask/src/perf.rs` uses `serde_yaml::from_slice` to read performance-budget YAML into a typed `BudgetFile`.
- Native metadata types such as `crates/fe_reader_metadata/src/lib.rs` already derive Serde. Dynamic `serde_json::Value` in extension/plugin/document operations may be appropriate; do not blindly turn all extension payloads into closed structs.

## Decision and bounds
Evaluate a maintained Serde-compatible YAML implementation for the xtask-only budget importer. Compare `serde-saphyr` and `yaml_serde` by independently verified upstream maintenance, licensing, advisories, Rust-version compatibility, input semantics and dependency footprint. Do not change normal PDF/runtime features or widen the core dependency graph. Migrate only if compatibility and evidence are stronger than retaining the current parser; an explicit no-go is permitted.

## Safety and compatibility
Reject malformed, oversize and duplicate-key budgets according to the declared contract; test alias bombs, tags, anchors, numeric/scalar ambiguity, UTF-8 and error diagnostics. Preserve existing baseline budgets and performance thresholds exactly. A Serde derive is not semantic or JSON Schema validation.

## Ownership and gate
Separate dependency PR, no edits to concurrent active implementation lanes; require independent review and exact-head `cargo test`, `cargo clippy`, `cargo tree` and the relevant performance-budget xtask tests. Documentation alone does not satisfy acceptance.
