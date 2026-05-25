# Frontier CI Experiments Plan

1. [x] Materialise files for this track.
2. [x] Run `python3 scripts/strict_contract_check.py`.
3. [x] Run `python3 scripts/ci_policy_check.py`.
4. [x] Update `docs/v9-coding-agent-start-here.md` with any missing first PRs.
5. [x] Mark advisory checks as hard only after baselines exist and an ADR approves the promotion.

## Completion Evidence

- Added `scripts/frontier_ci_check.py` and wired it into PR contracts, strict contracts, the Conductor phase gate, and the contract test matrix.
- Expanded frontier nightly to cover Rust beta/nightly check, Miri, sanitizer, fuzz, GPU frontier features, and differential oracle smoke while remaining scheduled/manual and `continue-on-error`.
- Expanded performance nightly to emit `artifacts/perf/manifest.json` and `artifacts/perf/summary.md`, upload advisory performance evidence, and run PGO/BOLT/build-speed tooling discovery.
- Added advisory smoke scripts for Miri, sanitizers, fuzz, GPU frontier features, and toolchain experiments; each writes JSON under `target/frontier-reports`.
- Added promotion metadata in `contracts/ci/contract-test-matrix.yaml` so frontier/performance checks cannot become hard gates without baselines and ADR approval.
- Verified with `scripts/conductor_phase_gate.sh --phase track-AN-frontier-ci-experiments --auto-fix`.
