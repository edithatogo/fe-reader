# Track AG: Config and Policy Engine

## Summary

Unify user settings, feature flags, enterprise policy, automation policy and active-content policy.

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
