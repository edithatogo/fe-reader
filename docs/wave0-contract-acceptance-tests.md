# Wave 0 contract acceptance tests

Wave 0 is accepted when the following are true.

## Rust workspace

- `cargo metadata --format-version=1` succeeds.
- `cargo test --workspace --all-targets` succeeds.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` succeeds or has only documented TODO suppressions.
- No UI/platform/PDFium/MCP/plugin/ML dependency appears in `fe_reader_core`.

## CLI

- `fe-reader doctor` prints core/pdf/security identities.
- `fe-reader inspect fixtures/minimal/minimal.pdf --json` emits `intent`, `plan` and `summary`.
- `fe-reader policy plugin` denies plugin loading by default.
- `fe-reader policy external-tool` denies external tools by default.

## Safety

- Mutating plans are not auto-approved.
- Automation mutation requires review by default.
- Redaction and signing plans must use high-risk policy once implemented.
- Secure redaction must never use incremental append.

## Implementation stop rule

Only add scope when it changes a contract, schema, test, CLI command, acceptance criterion or security boundary.
