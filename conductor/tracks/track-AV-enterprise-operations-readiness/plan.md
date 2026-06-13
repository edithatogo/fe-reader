# Track AV: Enterprise Operations Readiness Plan

## Phase AV1 - Offline install and install modes

- [ ] Task: Validate enterprise install paths.
    - [ ] Check macOS offline install path.
    - [ ] Check Windows offline install path.
    - [ ] Check Linux offline install path.
    - [ ] Validate per-user and global install documentation.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AV1 --auto-fix`.

## Phase AV2 - Managed policy templates

- [ ] Task: Validate enterprise policy templates.
    - [ ] Check policy schema.
    - [ ] Check policy precedence.
    - [ ] Check automation and update policy controls.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AV2 --auto-fix`.

## Phase AV3 - Update and rollback

- [ ] Task: Validate signed update and rollback workflows.
    - [ ] Validate stable update manifest requirements.
    - [ ] Validate rollback manifest requirements.
    - [ ] Fail unsigned or mismatched artifacts.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AV3 --auto-fix`.

## Phase AV4 - Support bundle and diagnostics

- [ ] Task: Add support bundle smoke validation.
    - [ ] Ensure document contents are excluded by default.
    - [ ] Ensure secrets are excluded.
    - [ ] Document support review workflow.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AV4 --auto-fix`.

## Exit Criteria

- Enterprise-ready desktop operations have runnable evidence and documentation.

