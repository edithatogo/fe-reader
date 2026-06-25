# Implementation Plan

## Phase BW1: Shared QA Manifest

- [ ] Task: Define the shared fixture and scenario manifest for platform parity.
- [ ] Task: Add report shape for launch/open/search/accessibility/performance results.
- [ ] Task: Add checker for missing required platform reports.
- [ ] Task: Run the checker locally.

## Phase BW2: Platform Smoke Coverage

- [ ] Task: Add or harden macOS, Windows, Linux, Android, iOS, Web/PWA and browser smoke scripts.
- [ ] Task: Keep optional real-device or heavy traces out of required PR gates.
- [ ] Task: Add CI jobs with minimal permissions, `timeout-minutes` and concurrency.
- [ ] Task: Run safe platform checks locally where supported.

## Phase BW3: Evidence Integration and Closeout

- [ ] Task: Update `docs/platform-parity-matrix.json` with QA evidence.
- [ ] Task: Run platform matrix, baseline parity and accessibility checks.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
