# Track AT: Stable Release Evidence Gates Plan

## Phase AT1 - Evidence schema hardening

- [ ] Task: Tighten stable release evidence schema and checks.
    - [ ] Require stable-channel SBOM/provenance/checksum/signing evidence.
    - [ ] Reject preview placeholders for stable, beta and enterprise channels.
    - [ ] Add tests for missing evidence classes.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AT1 --auto-fix`.

## Phase AT2 - Report gates

- [ ] Task: Gate stable release on required reports.
    - [ ] Validate compatibility corpus report.
    - [ ] Validate performance budget report.
    - [ ] Validate accessibility report.
    - [ ] Validate visual regression report.
    - [ ] Validate security/dependency report.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AT2 --auto-fix`.

## Phase AT3 - Waivers and rollback

- [ ] Task: Add stable-release waiver validation.
    - [ ] Require owner, expiry, rationale and rollback path.
    - [ ] Fail expired or ownerless waivers.
    - [ ] Include waivers in release evidence output.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AT3 --auto-fix`.

## Phase AT4 - CI release evidence

- [ ] Task: Wire stable evidence gates into release CI.
    - [ ] Ensure workflow permissions remain minimal.
    - [ ] Upload complete evidence bundles.
    - [ ] Document local and CI execution.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AT4 --auto-fix`.

## Exit Criteria

- Stable release evidence gate is strict, reproducible and CI-backed.
- Missing evidence produces actionable failures.

