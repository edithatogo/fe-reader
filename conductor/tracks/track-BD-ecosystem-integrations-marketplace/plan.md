# Track BD: Ecosystem Integrations and Marketplace Plan

## Phase BD1 - API and SDK compatibility

- [x] Task: Map ecosystem API surfaces.
    - [x] Document the feature gate `ecosystem_integrations_marketplace`.
    - [x] Add compatibility snapshot requirements.
    - [x] Record rollback and exit criteria.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BD1 --auto-fix`.

## Phase BD2 - Plugin and workflow-pack publication

- [x] Task: Define publication gates.
    - [x] Keep plugin mutations read-only or plan-only by default.
    - [x] Require provenance and maintainer approval.
    - [x] Document package/marketplace blockers.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BD2 --auto-fix`.

## Phase BD3 - Marketplace readiness

- [x] Task: Prepare marketplace metadata.
    - [x] Link support and security policies.
    - [x] Validate compatibility and automation safety evidence.
    - [x] Keep this track non-blocking for desktop stable.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BD3 --auto-fix`.
