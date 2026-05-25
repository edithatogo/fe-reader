# Repository CI/CD Plan

1. [x] Materialise files for this track.
2. [x] Run `python3 scripts/strict_contract_check.py`.
3. [x] Run `python3 scripts/ci_policy_check.py`.
4. [x] Update `docs/v9-coding-agent-start-here.md` with any missing first PRs.
5. [x] Mark advisory checks as hard only after baselines exist and an ADR approves the promotion.

## Completion Evidence

- Added `scripts/repository_ci_cd_check.py` and wired it into PR contracts, strict contracts, the Conductor phase gate, and the contract test matrix.
- Expanded CODEOWNERS domain coverage for repository policy, contracts, schemas, scripts, core, platform automation, and packaging.
- Split dependency automation ownership: Renovate owns Cargo updates; Dependabot owns GitHub Actions updates.
- Hardened CI policy checks for action pin/bootstrap markers, read-only permissions, stable triggers, required commands, concurrency, timeouts, and release evidence upload failure behavior.
- Added structured release evidence, release matrix, and release readiness JSON outputs under `target/release-evidence/`.
- Verified with `scripts/conductor_phase_gate.sh --phase track-AM-repository-ci-cd --auto-fix`.
