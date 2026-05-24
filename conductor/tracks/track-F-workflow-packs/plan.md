# Track F: Workflow Packs Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase F1 — workflow engine

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase F1 --auto-fix`.
### Phase F2 — signature templates

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase F2 --auto-fix`.
### Phase F3 — healthcare de-id

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase F3 --auto-fix`.
### Phase F4 — government disclosure

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase F4 --auto-fix`.
### Phase F5 — research/publishing/education packs

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase F5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
