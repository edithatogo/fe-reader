# Implementation Plan

## Phase BT1: Mobile Shell Scaffolds

- [ ] Task: Add Android wrapper scaffold for document open/share and reader state.
- [ ] Task: Add iOS wrapper scaffold for document browser/open-in and reader state.
- [ ] Task: Wire both wrappers to UniFFI read-only and plan-only facade operations.
- [ ] Task: Run `python3 scripts/mobile_smoke_bindings_check.py`.

## Phase BT2: Reader and Workflow Surfaces

- [ ] Task: Add search, inspect, validate and accessibility UI paths.
- [ ] Task: Add plan-only workflow actions for annotations, forms, page organization, redaction and conversion.
- [ ] Task: Keep write-capable flows gated behind policy and approval.
- [ ] Task: Update `docs/platform-parity-matrix.json` mobile evidence and limitations.

## Phase BT3: Mobile QA and Closeout

- [ ] Task: Run `bash scripts/android_emulator_smoke.sh` where local SDK support exists.
- [ ] Task: Run iOS simulator or target compile smoke through `.github/workflows/09-platform-tests.yml`.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
