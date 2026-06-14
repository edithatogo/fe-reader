# Track AX: Advanced Roadmap Continuation Plan

## Phase AX1 - Post-launch backlog decomposition

- [x] Task: Decompose advanced roadmap into granular tracks.
    - [x] Identify advanced PDF capabilities.
    - [x] Identify mobile launch follow-up.
    - [x] Identify optional frontier intelligence and GPU lanes.
    - [x] Identify cloud/collaboration follow-up only as separately approved.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AX1 --auto-fix`.

## Phase AX2 - Governance for advanced tracks

- [x] Task: Add governance metadata to advanced tracks.
    - [x] Require feature gates.
    - [x] Require owners, rollback plans and exit criteria for frontier dependencies.
    - [x] Mark launch-blocking versus non-blocking dependencies.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AX2 --auto-fix`.

## Phase AX3 - Implementation sequencing

- [x] Task: Produce post-launch implementation order.
    - [x] Prioritize launch-critical defect fixes.
    - [x] Sequence mobile and cloud work after desktop stable readiness.
    - [x] Keep optional frontier lanes advisory.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AX3 --auto-fix`.

## Exit Criteria

- Advanced roadmap continuation is represented as granular Conductor tracks and does not block desktop stable launch.

## Completion Evidence

- Added post-launch tracks AY, AZ, BA, BB, BC and BD with feature gates, owners, rollback plans, exit criteria and non-blocking launch metadata.
- Added `docs/post-launch-advanced-roadmap.md` to sequence post-launch work after desktop stable readiness and keep ML/RAG plus cloud collaboration opt-in.
- Added `scripts/advanced_roadmap_check.py` and verified it passes.
- Ran `scripts/conductor_phase_gate.sh --phase AX1 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AX2 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AX3 --auto-fix`.
