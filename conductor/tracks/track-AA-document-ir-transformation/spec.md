# Track AA: Document IR and Transformation Compiler

## Summary

Define and implement the typed intermediate representation, transformation graph, pass registry and pass-to-patch compilation flow.

## Non-negotiables

- Keep core boundaries intact.
- Add schemas and CLI exposure before UI polish.
- Add tests or smoke checks for every public contract.
- Do not make optional frontier features default without accepted evidence.

## Deliverables

- Rust contract file in `contracts/rust/`.
- JSON Schema in `schemas/` where relevant.
- CLI or xtask smoke command where feasible.
- Conductor phase-gate entry.
- Documentation in `docs/`.
