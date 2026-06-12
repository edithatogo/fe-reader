# Executable Wave 0 Contracts plan

- [x] Task: Run `cargo metadata --format-version=1`.
- [x] Task: Run `cargo test --workspace --all-targets`.
- [x] Task: Fix compile/test failures with smallest possible changes.
- [x] Task: Run CLI smoke commands against `fixtures/minimal/minimal.pdf`.
- [x] Task: Update `docs/wave0-contract-acceptance-tests.md` only when acceptance criteria change.

## Completion Evidence

- `cargo metadata --format-version=1` succeeds.
- `cargo fmt --all -- --check` succeeds.
- `cargo test --workspace --all-targets` succeeds.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` succeeds.
- `python3 scripts/strict_contract_check.py` succeeds.
- `python3 scripts/ci_policy_check.py` succeeds.
- `python3 scripts/validate_schemas.py` succeeds.
- `bash scripts/v8_cli_smoke.sh` succeeds.
- `bash scripts/wave0_bootstrap_check.sh` succeeds.
- `bash scripts/wave0_bootstrap_check.sh`, `bash scripts/v8_cli_smoke.sh`, and `python3 scripts/wave0_acceptance_check.py` were re-run on 2026-06-12 and passed.
- No acceptance criteria changes were needed in `docs/wave0-contract-acceptance-tests.md`.
