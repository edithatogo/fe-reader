# Document IR Transformation Evidence

## AA0

- `fe_reader_ir` exposes passive `DocumentIr` and `TransformationGraph` contracts.
- The read-only smoke graph validates against `schemas/document-ir.schema.json` and `schemas/transformation-pass.schema.json`.
- Evidence check: `python3 scripts/document_ir_contract_check.py`

## AA1

- `fe_reader_ir` adds a passive transformation pass registry and compile report.
- `xtask ir-compile-smoke` emits a compile report that stays passive and does not execute passes or create patch plans.
- `PatchPlan` can carry passive transformation graph metadata for mutating workflows without changing default apply behavior.
- Evidence check: `python3 scripts/ir_compile_smoke.py`

## Notes

- The compiler is binding-only. It validates pass metadata and records the selected graph, but it does not mutate document bytes.
- The compile metadata is optional and backward-compatible on the core patch plan and transaction journal surfaces.
