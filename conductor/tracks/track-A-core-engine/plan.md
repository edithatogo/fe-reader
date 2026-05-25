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

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase A2 --auto-fix`.

Phase A2 implementation note:

- Added focused `PatchPlan` tests for no-write plans and risk raising from mutating operations.
- Updated the patch plan JSON schema to match the serialized core contract used by CLI inspect.
- Added a rust public API snapshot and `scripts/patch_plan_contract_check.py` as a hard contract check for CLI plan JSON.
### Phase A3 — PDF model

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase A3 --auto-fix`.

Phase A3 implementation note:

- Tightened the PDF summary schema to match CLI serialization, including string document IDs.
- Added a malformed PDF fixture and contract check proving parser failures are reported as non-fatal diagnostics.
- Added a PDF model public API snapshot and wired `scripts/pdf_model_contract_check.py` into the phase gate.
### Phase A4 — Page ops

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase A4 --auto-fix`.

Phase A4 implementation note:

- Added typed page operation descriptors for delete, rotate and move inside the patch-plan contract.
- Updated the patch plan JSON schema and public API snapshot for page-operation plan shapes.
- Added `scripts/page_ops_contract_check.py` and wired it into the hard phase gate.
### Phase A5 — Write modes

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase A5 --auto-fix`.

Phase A5 implementation note:

- Added operation-derived write-mode policy so patch plans cannot choose weaker write modes by default.
- Escalated redaction and metadata scrubbing to `sanitizing_rewrite`, while stamps can use `incremental_append`.
- Added a write-mode public API snapshot and `scripts/write_modes_contract_check.py` as a hard phase-gate check.
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
