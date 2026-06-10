# Track AA: Document IR and Transformation Compiler Plan

## Phase 0

- [x] Read the relevant v6 documentation.
- [x] Materialise contract stubs.
- [x] Add schema validation fixtures.
- [x] Add architecture compliance checks for boundary violations.

Phase AA0 implementation note:

- Hardened the passive `fe_reader_ir` Document IR with annotation, image, form-field, optional-content, text-font and Unicode-confidence fields.
- Tightened Document IR and transformation graph schemas, including pass inputs and outputs.
- Added a public API snapshot and `scripts/document_ir_contract_check.py` to keep AA0 contract drift visible in the phase gate.

## Phase 1

- [x] Implement minimal compile-ready crate or module.
- [x] Add CLI/xtask smoke exposure.
- [x] Add at least one unit/property/golden test.

Phase AA1 implementation note:

- Added a passive transformation pass registry and graph compiler in `fe_reader_ir`.
- Added `xtask ir-compile-smoke` and `scripts/ir_compile_smoke.py` to emit and validate compile reports.
- Added a compile-report schema and extended the IR public API snapshot/checker so AA1 drift is gate-visible.

## Phase 2

- Integrate with OperationIntent/PatchPlan/TransactionJournal if mutating.
- Add release/evidence documentation.
- Run review skill and resolve allowed auto-fixes only.

## Exit criteria

- Review skill passes.
- No security or architecture failure is auto-fixed silently.
- Evidence file exists for any capability claim.
