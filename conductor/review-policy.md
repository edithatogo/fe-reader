# Review Policy

## Always run

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python3 scripts/validate_schemas.py
python3 scripts/architecture_compliance_check.py --workspace-root .
```

## Security/supply-chain checks

```bash
cargo deny check
cargo audit
cargo vet
```

## Optional smoke fuzz

```bash
cargo +nightly fuzz run fuzz_parse_document -- -max_total_time=60
```

## Auto-fix policy

Allowed:

- formatting;
- machine-applicable lint fixes;
- generated schema formatting;
- minor import cleanup.

Disallowed auto-proceed:

- architecture compliance failure;
- redaction verification failure;
- secure redaction using incremental append;
- automation bypass;
- MCP public HTTP exposure;
- plugin sandbox policy failure;
- distribution signing/notarisation failure;
- platform permission model failure;
- tests still failing after one auto-fix attempt.
