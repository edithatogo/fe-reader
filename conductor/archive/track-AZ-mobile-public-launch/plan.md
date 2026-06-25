# Track AZ: Mobile Public Launch Plan

## Phase AZ1 - Mobile launch gap audit

- [x] Task: Audit Android and iOS launch blockers.
    - [x] Review emulator, simulator, signing and store checklist evidence.
    - [x] Confirm the feature gate `mobile_public_launch` is documented.
    - [x] Record rollback and exit criteria.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AZ1 --auto-fix`.

## Phase AZ2 - Mobile release evidence

- [x] Task: Add mobile release evidence checks.
    - [x] Validate Android artifact and Play/F-Droid readiness.
    - [x] Validate iOS/TestFlight/App Store readiness.
    - [x] Keep mobile automation mutations gated.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AZ2 --auto-fix`.

## Phase AZ3 - Store publication readiness

- [x] Task: Prepare store-facing docs.
    - [x] Document privacy declarations.
    - [x] Document accessibility and power evidence.
    - [x] Keep this track non-blocking for desktop stable.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AZ3 --auto-fix`.
