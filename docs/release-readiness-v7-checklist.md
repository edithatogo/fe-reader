# Release Readiness v7 Checklist

This checklist records the Wave 0 handoff baseline for feature-building agents.
It is not a public release checklist; public preview and stable release gates are
tracked by `scripts/launch_qa_check.py`, `scripts/release_readiness_check.sh`,
and the desktop/mobile distribution docs.

## Current Status

- [x] `cargo metadata --format-version=1` succeeds.
  Evidence: local command and GitHub Actions `Rust Stable`.
- [x] `cargo test --workspace --all-targets` succeeds.
  Evidence: local command and GitHub Actions `Rust Stable`.
- [x] `fe-reader doctor` works.
  Evidence: `cargo run -q -p fe_reader_cli -- doctor`.
- [x] `fe-reader inspect --json` emits a non-mutating read-only plan envelope.
  Evidence: `cargo run -q -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json` and `scripts/v8_cli_smoke.sh`.
- [x] Schema validation passes.
  Evidence: `python3 scripts/validate_schemas.py`.
- [x] Architecture compliance confirms the core dependency firewall.
  Evidence: `python3 scripts/architecture_compliance_check.py`.
- [x] Error taxonomy and operation transaction contracts are present.
  Evidence: `scripts/strict_contract_check.py`, `contracts/rust/error_taxonomy.rs`, `contracts/rust/operation_transaction.rs`, and `schemas/operation-transaction.schema.json`.
- [x] First compatibility fixture manifest exists.
  Evidence: `fixtures/corpus/manifest.json` and `python3 scripts/corpus_manifest_validate.py`.
- [x] First 30 PRs are represented as implementation tasks.
  Evidence: `docs/wave0-first-30-prs.md` and `python3 scripts/wave0_first_30_prs_check.py`.
- [x] No local AI/ML/RAG dependency is added to Wave 0.
  Evidence: `docs/roadmap-waves.md`, `docs/frontier-intelligence-governance.md`, and `python3 scripts/frontier_intelligence_governance_check.py`.
- [x] Automation surfaces remain read-only or plan-only by default.
  Evidence: `bash scripts/security_policy_check.sh`, `python3 scripts/strict_mutation_contract_check.py`, and `bash scripts/platform_recent_smoke.sh`.

## Guard

Run this checklist guard before editing the handoff baseline:

```bash
python3 scripts/release_readiness_v7_check.py
```
