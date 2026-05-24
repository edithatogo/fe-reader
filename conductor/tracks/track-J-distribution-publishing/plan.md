# Track J: Distribution & Publishing Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase J1 — Windows winget/scoop/choco/msix/nsis

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase J1 --auto-fix`.
### Phase J2 — NuGet

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase J2 --auto-fix`.
### Phase J3 — Homebrew/MAS

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase J3 --auto-fix`.
### Phase J4 — Flatpak/Snap/AUR/deb/rpm/AppImage

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase J4 --auto-fix`.
### Phase J5 — Google Play/App Store

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase J5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
