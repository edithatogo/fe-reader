# Track J: Distribution & Publishing Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase J1 — Windows winget/scoop/choco/msix/nsis

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase J1 --auto-fix`.
### Phase J2 — NuGet

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase J2 --auto-fix`.
### Phase J3 — Homebrew/MAS

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase J3 --auto-fix`.
### Phase J4 — Flatpak/Snap/AUR/deb/rpm/AppImage

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase J4 --auto-fix`.
### Phase J5 — Google Play/App Store

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase J5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

## Completion Evidence

- Distribution and publishing evidence is recorded through `scripts/release_matrix_check.py`, `scripts/release_readiness_check.sh`, `scripts/release_evidence_check.sh`, `scripts/sbom_audit.sh`, `scripts/signing_readiness_check.sh`, and `scripts/generate_provenance_attestation.sh`.
- The public docs site now surfaces the release pipeline evidence contract in `docs-site/src/content/docs/release-pipeline.md`.
- Verified with `python3 scripts/release_matrix_check.py`, `bash scripts/release_readiness_check.sh`, `bash scripts/release_evidence_check.sh`, `bash scripts/sbom_audit.sh`, and `bash scripts/signing_readiness_check.sh`.
