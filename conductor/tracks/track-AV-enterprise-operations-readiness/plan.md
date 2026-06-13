# Track AV: Enterprise Operations Readiness Plan

## Phase AV1 - Offline install and install modes

- [x] Task: Validate enterprise install paths.
    - [x] Check macOS offline install path.
    - [x] Check Windows offline install path.
    - [x] Check Linux offline install path.
    - [x] Validate per-user and global install documentation.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AV1 --auto-fix`.

## Phase AV2 - Managed policy templates

- [x] Task: Validate enterprise policy templates.
    - [x] Check policy schema.
    - [x] Check policy precedence.
    - [x] Check automation and update policy controls.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AV2 --auto-fix`.

## Phase AV3 - Update and rollback

- [x] Task: Validate signed update and rollback workflows.
    - [x] Validate stable update manifest requirements.
    - [x] Validate rollback manifest requirements.
    - [x] Fail unsigned or mismatched artifacts.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AV3 --auto-fix`.

## Phase AV4 - Support bundle and diagnostics

- [x] Task: Add support bundle smoke validation.
    - [x] Ensure document contents are excluded by default.
    - [x] Ensure secrets are excluded.
    - [x] Document support review workflow.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AV4 --auto-fix`.

## Exit Criteria

- Enterprise-ready desktop operations have runnable evidence and documentation.

## Completion Evidence

- Added enterprise install-mode, managed policy, update/rollback and support-bundle fixtures under `packaging/enterprise/`.
- Added `scripts/enterprise_operations_readiness_check.py` and wired it into PR/release workflows, release readiness and provenance checks.
- Added `docs/enterprise-operations-readiness.md` and release-operations notes for the enterprise operations gate.
- Passed focused checks: `python3 scripts/enterprise_operations_readiness_check.py`, `bash scripts/release_readiness_check.sh`, `python3 scripts/release_provenance_check.py`, and `python3 scripts/ci_policy_check.py`.
- Passed phase gates: `scripts/conductor_phase_gate.sh --phase AV1 --auto-fix`, `AV2`, `AV3`, and `AV4`.
