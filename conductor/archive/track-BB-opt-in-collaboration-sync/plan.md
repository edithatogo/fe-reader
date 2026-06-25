# Track BB: Opt-in Collaboration and Sync Plan

## Phase BB1 - Collaboration contract design

- [x] Task: Define local-first collaboration contracts.
    - [x] Document the feature gate `opt_in_collaboration_sync`.
    - [x] Define packet and cache privacy boundaries.
    - [x] Record rollback and exit criteria.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BB1 --auto-fix`.

## Phase BB2 - Provider capability discovery

- [x] Task: Add sync provider discovery plan.
    - [x] Require explicit opt-in.
    - [x] Require clear offline and auth failure modes.
    - [x] Block silent upload and analytics.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BB2 --auto-fix`.

## Phase BB3 - Reversibility and support

- [x] Task: Validate rollback and support flows.
    - [x] Confirm local-only mode remains default.
    - [x] Add support bundle exclusions for sync-private data.
    - [x] Keep this track non-blocking for desktop stable.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BB3 --auto-fix`.
