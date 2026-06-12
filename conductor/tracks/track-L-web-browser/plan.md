# Track L: Web & Browser Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase L1 — Web local/PWA

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase L1 --auto-fix`.
### Phase L2 — WASM core subset

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase L2 --auto-fix`.
### Phase L3 — PDF.js rendering

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase L3 --auto-fix`.
### Phase L4 — File System Access fallback

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase L4 --auto-fix`.
### Phase L5 — browser extension

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase L5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

Track L completed after L1-L5 phase gates passed on 2026-06-12.
