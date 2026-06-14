# Track BD: Ecosystem Integrations and Marketplace Plan

## Phase BD1 - API and SDK compatibility

- [ ] Task: Map ecosystem API surfaces.
    - [ ] Document the feature gate `ecosystem_integrations_marketplace`.
    - [ ] Add compatibility snapshot requirements.
    - [ ] Record rollback and exit criteria.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BD1 --auto-fix`.

## Phase BD2 - Plugin and workflow-pack publication

- [ ] Task: Define publication gates.
    - [ ] Keep plugin mutations read-only or plan-only by default.
    - [ ] Require provenance and maintainer approval.
    - [ ] Document package/marketplace blockers.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BD2 --auto-fix`.

## Phase BD3 - Marketplace readiness

- [ ] Task: Prepare marketplace metadata.
    - [ ] Link support and security policies.
    - [ ] Validate compatibility and automation safety evidence.
    - [ ] Keep this track non-blocking for desktop stable.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BD3 --auto-fix`.
