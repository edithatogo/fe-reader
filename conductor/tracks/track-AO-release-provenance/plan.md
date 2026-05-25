# Release Provenance Plan

1. [x] Materialise files for this track.
2. [x] Run `python3 scripts/strict_contract_check.py`.
3. [x] Run `python3 scripts/ci_policy_check.py`.
4. [x] Update `docs/v9-coding-agent-start-here.md` with any missing first PRs.
5. [x] Mark advisory checks as hard only after baselines exist and an ADR approves the promotion.

## Completion Evidence

- Added release evidence scripts for SBOM status, provenance attestation scaffold, signing readiness, release evidence bundling, and release provenance policy checks.
- Wired release provenance commands into `.github/workflows/07-release.yml`, repository CI/CD checks, strict contract checks, and the contract test matrix.
- Extended release evidence and update manifest schemas with provenance, signing readiness, workflow, builder, source, and material metadata.
- Documented Wave 0 release provenance expectations in the release, reproducible build, signing, and CI/CD pipeline docs.
- Verified with `scripts/conductor_phase_gate.sh --phase track-AO-release-provenance --auto-fix`.
