# Track AT: Stable Release Evidence Gates Plan

## Phase AT1 - Evidence schema hardening

- [x] Task: Tighten stable release evidence schema and checks.
    - [x] Require stable-channel SBOM/provenance/checksum/signing evidence.
    - [x] Reject preview placeholders for stable, beta and enterprise channels.
    - [x] Add tests for missing evidence classes.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AT1 --auto-fix`.

## Phase AT2 - Report gates

- [x] Task: Gate stable release on required reports.
    - [x] Validate compatibility corpus report.
    - [x] Validate performance budget report.
    - [x] Validate accessibility report.
    - [x] Validate visual regression report.
    - [x] Validate security/dependency report.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AT2 --auto-fix`.

## Phase AT3 - Waivers and rollback

- [x] Task: Add stable-release waiver validation.
    - [x] Require owner, expiry, rationale and rollback path.
    - [x] Fail expired or ownerless waivers.
    - [x] Include waivers in release evidence output.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AT3 --auto-fix`.

## Phase AT4 - CI release evidence

- [x] Task: Wire stable evidence gates into release CI.
    - [x] Ensure workflow permissions remain minimal.
    - [x] Upload complete evidence bundles.
    - [x] Document local and CI execution.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AT4 --auto-fix`.

## Exit Criteria

- Stable release evidence gate is strict, reproducible and CI-backed.
- Missing evidence produces actionable failures.

## Completion Evidence

- `scripts/stable_release_evidence_check.py` validates stable-only evidence classes and waiver metadata.
- `packaging/release-waivers.yaml` defines the explicit stable waiver file.
- `scripts/release_readiness_check.sh` includes the stable release evidence check in release readiness output.
- `.github/workflows/07-release.yml` runs the stable evidence check before final release readiness.
- `docs/release-operations-updates.md` documents stable waiver requirements.
