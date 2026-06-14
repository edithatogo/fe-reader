# Track BC: Rendering Performance Promotion Plan

## Phase BC1 - Rendering budget baseline

- [x] Task: Define rendering and performance budgets.
    - [x] Document the feature gate `rendering_performance_promotion`.
    - [x] Map visual, differential and performance evidence.
    - [x] Record rollback and exit criteria.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BC1 --auto-fix`.

## Phase BC2 - Optional GPU promotion checks

- [x] Task: Gate optional GPU renderer promotion.
    - [x] Keep CPU fallback available.
    - [x] Require platform-specific smoke and visual evidence.
    - [x] Keep GPU paths disabled when evidence is incomplete.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BC2 --auto-fix`.

## Phase BC3 - Platform performance evidence

- [x] Task: Validate performance evidence.
    - [x] Capture startup, memory, render and power budget summaries.
    - [x] Link evidence from release-quality docs.
    - [x] Keep this track non-blocking for desktop stable.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase BC3 --auto-fix`.
