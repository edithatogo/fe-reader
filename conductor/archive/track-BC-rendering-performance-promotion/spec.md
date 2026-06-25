# Track BC: Rendering Performance Promotion

## Overview

Promote rendering and performance work after CPU baseline evidence is stable and optional GPU paths have governance. This track does not block desktop stable launch.

## Scope

- CPU rendering quality and budget evidence.
- Optional GPU renderer promotion criteria.
- Platform startup, memory, power and thermal budgets.
- Visual regression and differential oracle alignment.

## Functional Requirements

- GPU and other frontier renderer paths remain feature-gated.
- CPU-safe fallback must always exist.
- Performance claims require reproducible budgets and platform evidence.

## Non-Functional Requirements

- Feature gate: `rendering_performance_promotion`.
- Rollback: disable optional GPU/frontier renderer paths and use CPU/pdfium-safe rendering.
- Exit criteria: quality, memory, startup and platform budgets pass with documented fallback behavior.

## Acceptance Criteria

- Rendering promotion checklist exists.
- Visual and differential evidence supports supported claims.
- Performance budgets are stable across target desktop platforms.

## Out of Scope

- Making GPU acceleration mandatory.
- ML rendering.
- Launch-blocking desktop signing work.
