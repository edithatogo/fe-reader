# Track K: Security & Quality

## Theme

Foundation

## Scope

- cargo deny/audit/vet
- fuzzing
- schema validation
- architecture compliance
- redaction verification

## Contract files

- `contracts/README.md`
- Relevant `contracts/rust/*.rs`
- Relevant platform/application/web/MCP contracts
- Relevant schemas in `schemas/`

## Hard rules

- Map high-risk actions to `FeOperationIntent`.
- Return patch plans for destructive or high-risk mutations.
- Update CLI tests for core operations.
- Run phase gate after every phase.
- Do not introduce ML/RAG dependency unless this is Track M in Wave 6.

## Deliverables

- Compileable crate/module skeletons.
- Unit tests and at least one golden/smoke test.
- Documentation update.
- Contract/schema update where public shape changes.
