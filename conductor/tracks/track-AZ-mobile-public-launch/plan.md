# Track AZ: Mobile Public Launch Plan

## Phase AZ1 - Mobile launch gap audit

- [ ] Task: Audit Android and iOS launch blockers.
    - [ ] Review emulator, simulator, signing and store checklist evidence.
    - [ ] Confirm the feature gate `mobile_public_launch` is documented.
    - [ ] Record rollback and exit criteria.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AZ1 --auto-fix`.

## Phase AZ2 - Mobile release evidence

- [ ] Task: Add mobile release evidence checks.
    - [ ] Validate Android artifact and Play/F-Droid readiness.
    - [ ] Validate iOS/TestFlight/App Store readiness.
    - [ ] Keep mobile automation mutations gated.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AZ2 --auto-fix`.

## Phase AZ3 - Store publication readiness

- [ ] Task: Prepare store-facing docs.
    - [ ] Document privacy declarations.
    - [ ] Document accessibility and power evidence.
    - [ ] Keep this track non-blocking for desktop stable.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AZ3 --auto-fix`.
