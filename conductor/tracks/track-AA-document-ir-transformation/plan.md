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

## Completion Evidence

- `fe_reader_core::PatchPlan` now carries passive transformation graph metadata without changing write approval or write mode.
- `fe_reader_core::TransactionJournal` now carries the transformation graph id from a planned patch.
- `fe_reader_uniffi::FePatchPlan` exposes the same passive transformation metadata for bindings.
- `schemas/patch-plan.schema.json`, `contracts/rust/core_types.rs`, and `scripts/patch_plan_contract_check.py` now keep the contract visible in schema and token checks.
- `docs/document-ir-transformation-evidence.md` records the phase evidence and `docs/engine-ir-and-transformation-pipeline.md` now documents the patch-plan binding behavior.
- Verified with `cargo fmt --all`, `git diff --check`, `cargo test -q -p fe_reader_core`, `cargo test -q -p fe_reader_uniffi`, `cargo clippy -p fe_reader_core -p fe_reader_uniffi --all-targets --all-features -- -D warnings`, and `bash scripts/conductor_phase_gate.sh --phase AA1 --auto-fix`.

## Exit criteria

- Review skill passes.
- No security or architecture failure is auto-fixed silently.
- Evidence file exists for any capability claim.
