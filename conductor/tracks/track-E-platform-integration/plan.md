# Track E: Platform Integration Plan

## Parallelisation

This track may proceed when its wave dependencies in `conductor/waves.yaml` are satisfied. Coordinate with dependent tracks through contracts rather than ad-hoc shared code.

### Phase E1 — Windows recents/COM

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase E1 --auto-fix`.
### Phase E2 — macOS Open Recent/AppleScript/App Intents

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase E2 --auto-fix`.
### Phase E3 — Linux portals/D-Bus

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase E3 --auto-fix`.
### Phase E4 — Android SAF/AppSearch

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase E4 --auto-fix`.
### Phase E5 — iOS document browser/PencilKit

- Implement contract skeleton.
- Add unit/smoke test.
- Add CLI or adapter path where applicable.
- Run `scripts/conductor_phase_gate.sh --phase E5 --auto-fix`.

## Exit criteria

- Track-specific contracts implemented or stubbed.
- Tests pass.
- Review skill passes.
- Any blocked/forked dependency is recorded in `third_party/fork-policy.yaml`.

## Completion Evidence

- Platform integration contracts are implemented in `crates/fe_reader_platform/src/lib.rs` as read-only/default-deny stubs for Windows, macOS, Linux, Android, and iOS surfaces.
- Contract files exist for Windows COM, macOS AppleScript, Linux D-Bus, Android intents, and iOS App Intents under `contracts/platform/`.
- Verified with `cargo test -q -p fe_reader_platform`, `bash scripts/platform_recent_smoke.sh`, `bash scripts/wave5_integration_smoke.sh`, `python3 scripts/platform_search_contract_smoke.py`, `python3 scripts/ios_share_shortcuts_smoke.py`, and `python3 scripts/mobile_smoke_bindings_check.py`.
