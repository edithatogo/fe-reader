# v8 coding agent start here

## Goal

Turn the v8 scaffold into the first compiling, testable implementation checkpoint.

## Read first

1. `IMPLEMENTATION_PROMPT.md`
2. `docs/final-v8-improvements.md`
3. `docs/wave0-first-30-prs.md`
4. `crates/fe_reader_core/src/lib.rs`
5. `crates/fe_reader_pdf_model/src/lib.rs`
6. `crates/fe_reader_cli/src/main.rs`

## Commands to run first

```bash
cargo metadata --format-version=1
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo run -p fe_reader_cli -- doctor
cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json
python3 scripts/validate_schemas.py
bash scripts/wave0_bootstrap_check.sh
```

## First implementation rule

If these fail, fix the smallest concrete issue. Do not add product scope.

## Next PRs after compile

1. Add parser adapter smoke tests around `lopdf` without leaking `lopdf` into `fe_reader_core`.
2. Add a real `inspect` implementation that reports page count from a parser adapter.
3. Add a tile-render adapter stub that can be swapped for PDFium.
4. Add transaction-journal JSON persistence.
5. Add policy tests for MCP, COM, AppleScript, D-Bus, intents, web and plugin surfaces.
