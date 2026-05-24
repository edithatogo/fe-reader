# CLI First Integration plan

- [x] Task: Run `cargo metadata --format-version=1`.
- [x] Task: Run `cargo test --workspace --all-targets`.
- [x] Task: Fix compile/test failures with smallest possible changes.
- [x] Task: Run CLI smoke commands against `fixtures/minimal/minimal.pdf`.
- [x] Task: Update `docs/wave0-contract-acceptance-tests.md` only when acceptance criteria change.

## Completion Evidence

- `fe_reader_pdf_model` uses `lopdf` outside `fe_reader_core` to provide parser-backed inspection diagnostics.
- `fe-reader inspect fixtures/minimal/minimal.pdf --json` emits `summary.parser.page_count = 1`.
- The inspect plan remains read-only with `write_mode = "no_write"`.
- `cargo metadata --format-version=1` succeeds.
- `cargo fmt --all -- --check` succeeds.
- `cargo test --workspace --all-targets` succeeds.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` succeeds.
- `python3 scripts/strict_contract_check.py` succeeds.
- `python3 scripts/validate_schemas.py` succeeds.
- `bash scripts/v8_cli_smoke.sh` succeeds.
- `bash scripts/wave0_bootstrap_check.sh` succeeds.
