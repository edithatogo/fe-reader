# Track BC: Rendering Performance Promotion Plan

## Phase BC1 - Rendering budget baseline

- [ ] Task: Define rendering and performance budgets.
    - [ ] Document the feature gate `rendering_performance_promotion`.
    - [ ] Map visual, differential and performance evidence.
    - [ ] Record rollback and exit criteria.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BC1 --auto-fix`.

## Phase BC2 - Optional GPU promotion checks

- [ ] Task: Gate optional GPU renderer promotion.
    - [ ] Keep CPU fallback available.
    - [ ] Require platform-specific smoke and visual evidence.
    - [ ] Keep GPU paths disabled when evidence is incomplete.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BC2 --auto-fix`.

## Phase BC3 - Platform performance evidence

- [ ] Task: Validate performance evidence.
    - [ ] Capture startup, memory, render and power budget summaries.
    - [ ] Link evidence from release-quality docs.
    - [ ] Keep this track non-blocking for desktop stable.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase BC3 --auto-fix`.
