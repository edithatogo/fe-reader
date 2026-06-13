# Track Q Plan

## Q0 Wave 1

- [x] Implement in-memory literal search over extracted spans.
- [x] Add page+bounding-box search result contract.
- Evidence: `crates/fe_reader_search/src/lib.rs`, `scripts/search_index_smoke.sh`, `contracts/snapshots/rust-public-api/fe_reader_search.search.preview.json`.

## Q1 Wave 2

- [x] Add phrase, regex and metadata search.
- [x] Add text diagnostics contract.
- [x] Add CJK/RTL test fixtures.
- Evidence: `crates/fe_reader_text/src/lib.rs`, `docs/text-font-i18n-plan.md`, `fixtures/corpus/rtl-cjk-complex-text/`.

## Q2 Wave 5

- [x] Add optional Tantivy provider behind `search_tantivy` feature.
- [x] Add workspace index purge and privacy controls.
- Evidence: `crates/fe_reader_search/Cargo.toml`, `docs/privacy-diagnostics-observability.md`, `docs/content-addressed-cache-workspace.md`.

## Q3 Wave 7

- [x] Add accessibility text report and release search compatibility report.
- Evidence: `scripts/accessibility_audit_smoke.py`, `scripts/search_compatibility_report.py`, `target/search-compatibility-report.json`, `target/search-compatibility-report.md`.
