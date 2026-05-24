# Track M: Frontier Intelligence Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase M1 — optional local NER

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase M1 --auto-fix`.
### Phase M2 — embeddings

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase M2 --auto-fix`.
### Phase M3 — grounded Q&A

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase M3 --auto-fix`.
### Phase M4 — model provenance

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase M4 --auto-fix`.
### Phase M5 — no automatic mutation

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase M5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
