# Fe Reader v7 Coding Agent Start Here

Start here, not in the broad strategy documents.

## Objective

Make Wave 0 executable. The first goal is not feature richness. The first goal is a compiling, testable, policy-gated skeleton that proves the architecture boundaries are real.

## Read in order

1. `IMPLEMENTATION_PROMPT.md`
2. `AGENTS.md`
3. `docs/final-v7-improvements.md`
4. `docs/wave0-first-30-prs.md`
5. `Cargo.toml`
6. `crates/fe_reader_core/src/lib.rs`
7. `crates/fe_reader_cli/src/main.rs`
8. `scripts/wave0_bootstrap_check.sh`
9. `conductor/waves.yaml`
10. `conductor/tracks.md`

## First command set

```bash
bash scripts/wave0_bootstrap_check.sh
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo run -p fe_reader_cli -- doctor
cargo run -p fe_reader_cli -- inspect fixtures/empty-placeholder.pdf --json
```

If the local Rust toolchain is not available, do not invent code around it. Record the missing toolchain and continue with contract/schema work.

## Hard boundaries

- `fe_reader_core` must not depend on rendering, Tauri, platform APIs, MCP, UniFFI, plugin runtimes or ML crates.
- Any PDF mutation must go through intent -> patch plan -> transaction journal -> apply -> verify -> receipt.
- Secure redaction must use sanitising rewrite, not incremental append.
- Automation surfaces are read-only by default.
- No local LLM/RAG/NER work before deterministic extraction, search, redaction verification and performance baselines are implemented.

## First success definition

A successful Wave 0 implementation produces a boring but reliable repo:

- all crates compile,
- CLI has `doctor`, `inspect`, and schema-validation hooks,
- schemas validate,
- phase gate script passes,
- CI samples are wired,
- transaction journal and error taxonomy contracts are present,
- no platform or renderer dependencies leak into the core.
