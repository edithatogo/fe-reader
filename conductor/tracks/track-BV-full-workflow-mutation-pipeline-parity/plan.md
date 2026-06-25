# Implementation Plan

## Phase BV1: Apply Contract

- [ ] Task: Define metadata update as the first approved low-risk apply workflow.
- [ ] Task: Add or harden intent, patch plan, policy, apply, verify and audit receipt DTOs.
- [ ] Task: Add a fail-closed checker for required mutation guards.
- [ ] Task: Run focused Rust tests for core, security, metadata and CLI crates.

## Phase BV2: Adapter Exposure

- [ ] Task: Expose approved metadata update through CLI JSON and shared facade.
- [ ] Task: Add plan/apply status to native/web/mobile contracts without enabling high-risk shortcuts.
- [ ] Task: Ensure redaction remains plan/verify-only.
- [ ] Task: Update API compatibility notes and platform parity matrix evidence.

## Phase BV3: Mutation QA and Closeout

- [ ] Task: Add golden smoke proving output artifact and audit receipt verification.
- [ ] Task: Run `python3 scripts/platform_parity_matrix_check.py` and mutation contract checks.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
