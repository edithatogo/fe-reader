# Track K: Security & Quality Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase K1 — cargo deny/audit/vet

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase K1 --auto-fix`.
### Phase K2 — fuzzing

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase K2 --auto-fix`.
### Phase K3 — schema validation

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase K3 --auto-fix`.
### Phase K4 — architecture compliance

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase K4 --auto-fix`.
### Phase K5 — redaction verification

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase K5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

Track K completed after K1-K5 phase gates passed on 2026-06-12.
