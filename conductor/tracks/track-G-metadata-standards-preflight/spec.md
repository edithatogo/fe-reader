# Track G: Metadata, Standards & Preflight

## Theme

Professional documents

## Scope

- XMP
- scrub/preserve
- PDF 2.0 features
- PDF/A/UA/X adapters
- accessibility inspection

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
