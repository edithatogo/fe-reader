# Track B: Rendering & Hardware Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase B1 — RenderBackend trait

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase B1 --auto-fix`.
### Phase B2 — PDFium tile renderer

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase B2 --auto-fix`.
### Phase B3 — tile cache

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase B3 --auto-fix`.
### Phase B4 — GPU compositor option

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase B4 --auto-fix`.
### Phase B5 — hardware acceleration flags

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase B5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

## Completion Evidence

- `fe_reader_render` exposes validated tile requests, acceleration preferences, stable cache keys, thumbnail dispatch, and a bounded in-memory tile cache.
- `fe_reader_render_pdfium` implements the render backend boundary with explicit unavailable-runtime behavior until PDFium library discovery is governed by platform policy.
- `fe_reader_render_gpu` exposes conservative hardware acceleration flags and GPU compositor policy resolution without enabling GPU rendering by default.
- `contracts/rust/render_backend.rs` matches the adapter-facing render contract shape.
- Verified focused package tests with `cargo test -p fe_reader_render --all-targets`, `cargo test -p fe_reader_render_pdfium --all-targets`, and `cargo test -p fe_reader_render_gpu --all-targets`.
