# Track AA: Document IR and Transformation Compiler Plan

## Phase 0

- Read the relevant v6 documentation.
- Materialise contract stubs.
- Add schema validation fixtures.
- Add architecture compliance checks for boundary violations.

## Phase 1

- Implement minimal compile-ready crate or module.
- Add CLI/xtask smoke exposure.
- Add at least one unit/property/golden test.

## Phase 2

- Integrate with OperationIntent/PatchPlan/TransactionJournal if mutating.
- Add release/evidence documentation.
- Run review skill and resolve allowed auto-fixes only.

## Exit criteria

- Review skill passes.
- No security or architecture failure is auto-fixed silently.
- Evidence file exists for any capability claim.
