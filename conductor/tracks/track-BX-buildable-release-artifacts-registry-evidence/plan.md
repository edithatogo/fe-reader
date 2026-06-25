# Implementation Plan

## Phase BX1: Artifact Build Matrix

- [ ] Task: Define artifact targets for desktop, mobile, web and browser extension dev builds.
- [ ] Task: Add local build scripts that emit artifacts under `target/release-artifacts/`.
- [ ] Task: Generate checksums and release evidence manifest.
- [ ] Task: Run artifact smoke validation.

## Phase BX2: Registry Evidence

- [ ] Task: Update packaging and registry status to distinguish dev artifacts from external publication gates.
- [ ] Task: Keep signed/notarized/store-published states blocked until real evidence exists.
- [ ] Task: Add checker coverage for artifact manifests and registry honesty.
- [ ] Task: Run release and registry checks.

## Phase BX3: Release QA and Closeout

- [ ] Task: Update docs and release notes with evidence-backed install paths only.
- [ ] Task: Run `python3 scripts/platform_parity_matrix_check.py` and release readiness checks.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
