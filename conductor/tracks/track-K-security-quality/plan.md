# Track K: Security & Quality Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase K1 — cargo deny/audit/vet

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase K1 --auto-fix`.
### Phase K2 — fuzzing

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase K2 --auto-fix`.
### Phase K3 — schema validation

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase K3 --auto-fix`.
### Phase K4 — architecture compliance

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase K4 --auto-fix`.
### Phase K5 — redaction verification

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase K5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
