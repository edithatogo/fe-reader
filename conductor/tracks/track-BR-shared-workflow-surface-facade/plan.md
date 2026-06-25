# Implementation Plan

## Phase BR1: Contract and Operation IDs

- [ ] Task: Add a shared workflow facade contract outside `fe_reader_core`.
- [ ] Task: Define stable operation IDs for reader, diagnostics, planning and approved apply.
- [ ] Task: Add a checker that rejects missing operation IDs or unsafe mutation shortcuts.
- [ ] Task: Run the new checker and `python3 scripts/platform_parity_matrix_check.py`.

## Phase BR2: Adapter Exposure

- [ ] Task: Wire the operation IDs into CLI JSON and adapter-facing snapshots.
- [ ] Task: Update UniFFI, C ABI and web postMessage contract surfaces compatibly.
- [ ] Task: Add API compatibility notes for CLI, UniFFI, C ABI, MCP, web, Android, iOS, Windows COM, AppleScript and Linux D-Bus.
- [ ] Task: Run `python3 scripts/mobile_smoke_bindings_check.py`, `python3 scripts/web_postmessage_contract_smoke.py` and `python3 scripts/browser_extension_contract_smoke.py`.

## Phase BR3: Conductor Closeout

- [ ] Task: Run `cargo test --workspace --all-targets`.
- [ ] Task: Run `python3 scripts/conductor_lifecycle_check.py --require-git-note`.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
