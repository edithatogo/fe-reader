# Track BA: Frontier Intelligence Governance Plan

## Phase BA1 - Frontier policy baseline

- [ ] Task: Define frontier preview policy.
    - [ ] Document the feature gate `frontier_intelligence_preview`.
    - [ ] Assign owner, rollback and exit criteria.
    - [ ] Keep ML/RAG disabled by default.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BA1 --auto-fix`.

## Phase BA2 - Evaluation and privacy evidence

- [ ] Task: Add evaluation guardrails.
    - [ ] Define privacy-safe eval fixtures.
    - [ ] Define resource and quality budgets.
    - [ ] Verify no private data leaves the local machine.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BA2 --auto-fix`.

## Phase BA3 - Preview promotion gate

- [ ] Task: Define promotion and rollback checks.
    - [ ] Require explicit opt-in UI/CLI controls.
    - [ ] Require rollback path and disable switch.
    - [ ] Keep this track non-blocking for desktop stable.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BA3 --auto-fix`.
