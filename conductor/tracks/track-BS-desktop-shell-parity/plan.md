# Implementation Plan

## Phase BS1: Desktop Shell Baseline

- [ ] Task: Identify the macOS reference behavior and record it in the desktop shell parity notes.
- [ ] Task: Add or harden the Tauri v2 desktop shell scaffold for Windows and Linux.
- [ ] Task: Wire open, recents, document state and safe failure states through shared adapter commands.
- [ ] Task: Run desktop compile or smoke validation available on the host.

## Phase BS2: Reader and Workflow Parity

- [ ] Task: Wire navigation, zoom, search, inspect, validate and accessibility-report actions.
- [ ] Task: Add plan-only annotations, forms, page organization, redaction and conversion action surfaces.
- [ ] Task: Ensure write-capable actions produce plans and never direct mutations.
- [ ] Task: Update `docs/platform-parity-matrix.json` statuses and evidence.

## Phase BS3: Desktop QA and Closeout

- [ ] Task: Add macOS, Windows and Linux desktop shell smoke checks.
- [ ] Task: Run `python3 scripts/platform_parity_matrix_check.py` and desktop-focused CI checks.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
