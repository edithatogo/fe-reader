# Track G: Metadata, Standards & Preflight Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase G1 — XMP

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase G1 --auto-fix`.
- Evidence: `scripts/metadata_wave2_smoke.sh`, `scripts/conductor_phase_gate.sh --phase G1 --auto-fix`.
### Phase G2 — scrub/preserve

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase G2 --auto-fix`.
- Evidence: `scripts/metadata_wave2_smoke.sh`, `scripts/conductor_phase_gate.sh --phase G2 --auto-fix`.
### Phase G3 — PDF 2.0 features

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase G3 --auto-fix`.
- Evidence: `crates/fe_reader_metadata/src/lib.rs`, `scripts/conductor_phase_gate.sh --phase G3 --auto-fix`.
### Phase G4 — PDF/A/UA/X adapters

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase G4 --auto-fix`.
- Evidence: `crates/fe_reader_prepress/src/lib.rs`, `scripts/prepress_smoke.sh`.
### Phase G5 — accessibility inspection

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase G5 --auto-fix`.
- Evidence: `crates/fe_reader_accessibility/src/lib.rs`, `scripts/accessibility_audit_smoke.py`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
