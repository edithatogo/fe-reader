# Track A: Core Engine Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase A1 — Operation intent contracts

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase A1 --auto-fix`.

Phase A1 implementation note:

- Tightened `OperationIntent` construction with an explicit risk-classified constructor and a high-risk helper.
- Updated the operation intent JSON schema to match the serialized core contract used by CLI inspect.
- Added `scripts/operation_intent_contract_check.py` as a hard contract check for CLI intent JSON and schema alignment.
### Phase A2 — Patch plans

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase A2 --auto-fix`.
### Phase A3 — PDF model

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase A3 --auto-fix`.
### Phase A4 — Page ops

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase A4 --auto-fix`.
### Phase A5 — Write modes

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase A5 --auto-fix`.
### Phase A6 — Audit receipts

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase A6 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
