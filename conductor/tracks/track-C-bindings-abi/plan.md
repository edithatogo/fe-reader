# Track C: Bindings & ABI Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase C1 — UniFFI Swift/Kotlin/Python/Ruby

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase C1 --auto-fix`.

Phase C1 implementation note:

- Added a narrow `fe_reader_uniffi` proc-macro facade over core `OperationIntent` and `PatchPlan` contracts.
- Exposes owned binding DTOs for Swift/Kotlin/Python/Ruby smoke generation without exposing PDF parsing, rendering, apply, platform, plugin, or ML surfaces.
- Records the preview API compatibility decision in `contracts/snapshots/uniffi/fe_reader_uniffi.facade.json`.
- Verified Swift, Kotlin, Python and Ruby binding generation into `target/uniffi-smoke/`.
### Phase C2 — C# ABI fallback

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase C2 --auto-fix`.

Phase C2 implementation note:

- Added `fe_reader_c_abi` as a separate preview C ABI fallback for C# P/Invoke wrappers.
- Exposes static identity/capability exports and a no-write plan contract probe only; no apply path is exposed.
- Records the preview ABI compatibility decision in `contracts/snapshots/c-abi/fe_reader_c_abi.facade.json`.
- Adds `scripts/c_abi_snapshot_check.sh` to verify exported C symbols against the snapshot.
### Phase C3 — NuGet wrapper

- [x] Implement contract skeleton.
- [x] Add unit/smoke test.
- [x] Add CLI or adapter path where applicable.
- [x] Run `scripts/conductor_phase_gate.sh --phase C3 --auto-fix`.

Phase C3 implementation note:

- Added `FeReader.Native` as an SDK-style preview NuGet wrapper over the C ABI fallback.
- Added a managed smoke app that verifies wrapper metadata, P/Invoke entry points and the C-compatible plan contract layout without requiring native runtime assets.
- Records the preview .NET wrapper compatibility decision in `contracts/snapshots/dotnet/FeReader.Native.facade.json`.
- Adds `scripts/nuget_wrapper_check.sh` as an advisory local build, smoke and pack check.
### Phase C4 — mobile smoke bindings

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase C4 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.
