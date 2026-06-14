# Track BB: Opt-in Collaboration and Sync Plan

## Phase BB1 - Collaboration contract design

- [ ] Task: Define local-first collaboration contracts.
    - [ ] Document the feature gate `opt_in_collaboration_sync`.
    - [ ] Define packet and cache privacy boundaries.
    - [ ] Record rollback and exit criteria.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BB1 --auto-fix`.

## Phase BB2 - Provider capability discovery

- [ ] Task: Add sync provider discovery plan.
    - [ ] Require explicit opt-in.
    - [ ] Require clear offline and auth failure modes.
    - [ ] Block silent upload and analytics.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BB2 --auto-fix`.

## Phase BB3 - Reversibility and support

- [ ] Task: Validate rollback and support flows.
    - [ ] Confirm local-only mode remains default.
    - [ ] Add support bundle exclusions for sync-private data.
    - [ ] Keep this track non-blocking for desktop stable.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BB3 --auto-fix`.
