# Track D: UI & Native Shell Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase D1 — Tauri shell

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase D1 --auto-fix`.
### Phase D2 — Svelte viewer

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase D2 --auto-fix`.
### Phase D3 — annotation overlay

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase D3 --auto-fix`.
### Phase D4 — mobile native wrappers

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase D4 --auto-fix`.
### Phase D5 — document UX

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase D5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

## Completion Evidence

- Native shell is implemented in `native/macos/FeReaderNativeApp.swift` with a SwiftUI-first shell, AppKit interop for launch and capture behavior, native command surfaces, and adapter-driven document handling.
- Verified on 2026-06-12 with `bash script/build_and_run.sh --verify`, which produced `artifacts/screenshots/fe-reader-native-preview.png` and passed the native preview check.
- Verified with `cargo test -q -p fe_reader_platform`, `bash scripts/platform_recent_smoke.sh`, `bash scripts/wave5_integration_smoke.sh`, `python3 scripts/platform_search_contract_smoke.py`, `python3 scripts/ios_share_shortcuts_smoke.py`, and `python3 scripts/mobile_smoke_bindings_check.py`.
