# Implementation Plan

## Phase BU1: Web Local Reader

- [ ] Task: Add web-local/PWA scaffold using the existing web postMessage contract.
- [ ] Task: Implement user-granted PDF open, inspect, search and validate paths.
- [ ] Task: Add plan-only workflow action surfaces.
- [ ] Task: Run `python3 scripts/web_postmessage_contract_smoke.py`.

## Phase BU2: Browser Extension Handoff

- [ ] Task: Add browser-extension scaffold for embedded PDF/link detection and handoff.
- [ ] Task: Support inspect and plan-workflow messages only.
- [ ] Task: Ensure no direct local mutation, hidden upload or unapproved persistence.
- [ ] Task: Run `python3 scripts/browser_extension_contract_smoke.py`.

## Phase BU3: Web QA and Closeout

- [ ] Task: Add Playwright or equivalent smoke for web-local/PWA fixture open and search.
- [ ] Task: Update `docs/platform-parity-matrix.json` web and browser evidence.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
