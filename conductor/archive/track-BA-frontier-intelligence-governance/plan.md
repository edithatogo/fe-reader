# Track BA: Frontier Intelligence Governance Plan

## Phase BA1 - Frontier policy baseline

- [x] Task: Define frontier preview policy.
    - [x] Document the feature gate `frontier_intelligence_preview`.
    - [x] Assign owner, rollback and exit criteria.
    - [x] Keep ML/RAG disabled by default.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BA1 --auto-fix`.

## Phase BA2 - Evaluation and privacy evidence

- [x] Task: Add evaluation guardrails.
    - [x] Define privacy-safe eval fixtures.
    - [x] Define resource and quality budgets.
    - [x] Verify no private data leaves the local machine.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BA2 --auto-fix`.

## Phase BA3 - Preview promotion gate

- [x] Task: Define promotion and rollback checks.
    - [x] Require explicit opt-in UI/CLI controls.
    - [x] Require rollback path and disable switch.
    - [x] Keep this track non-blocking for desktop stable.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BA3 --auto-fix`.
