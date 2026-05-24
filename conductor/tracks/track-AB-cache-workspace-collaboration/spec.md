# Track AB: Cache, Workspace & Offline Collaboration

## Theme

Workspace

## Purpose

Content-addressed cache, workspace catalogue, review packets and offline collaboration.

## Non-negotiables

- Add or update contracts before implementation.
- Keep `fe_reader_core` free of UI, OS, network, database, renderer and converter dependencies.
- Provide CLI or smoke-test coverage for new behaviours.
- Add corpus fixtures or synthetic examples when claims depend on PDF behaviour.
- Respect OperationIntent -> PatchPlan -> Policy/Review -> Apply -> Verify -> Receipt for document mutation.

## Main waves

0-8

## Deliverables

- Track-specific contract updates.
- Track-specific schema updates.
- Smoke tests and phase-gate integration.
- Documentation updates.
- Release evidence notes when the track affects public behaviour.
