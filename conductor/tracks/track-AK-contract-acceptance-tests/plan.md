# Contract Acceptance Tests plan

- [x] Task: Run `cargo metadata --format-version=1`.
- [x] Task: Run `cargo test --workspace --all-targets`.
- [x] Task: Fix compile/test failures with smallest possible changes.
- [x] Task: Run CLI smoke commands against `fixtures/minimal/minimal.pdf`.
- [x] Task: Update `docs/wave0-contract-acceptance-tests.md` only when acceptance criteria change.

## Completion Evidence

- `python3 scripts/wave0_acceptance_check.py` validates `inspect --json` output, parser summary schema, non-mutating plans, and the Wave 0 policy matrix.
- `scripts/conductor_phase_gate.sh` now runs strict contracts, CI policy, schema validation, static v8 contracts, CLI smoke, and Wave 0 acceptance as hard gates.
- `scripts/wave0_bootstrap_check.sh` no longer masks schema, formatting, or test failures.
- `scripts/v8_cli_smoke.sh` asserts doctor identities, inspect envelope shape, read-only plan details, parser diagnostics, and policy decisions.
- `scripts/security_policy_check.sh` asserts default security policy, MCP destructive-tool requirements, and platform approval-token patterns.
- `cargo test -p fe_reader_security --lib` covers the default policy matrix.
- `scripts/conductor_phase_gate.sh --phase track-AK-contract-acceptance-tests --auto-fix` succeeds.
- `bash scripts/wave0_bootstrap_check.sh`, `bash scripts/v8_cli_smoke.sh`, and `python3 scripts/wave0_acceptance_check.py` were re-run on 2026-06-12 and passed.
