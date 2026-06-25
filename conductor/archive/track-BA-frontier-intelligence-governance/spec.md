# Track BA: Frontier Intelligence Governance

## Overview

Define governance for optional local ML, RAG and intelligent assistance work after deterministic features mature. This track does not block desktop stable launch.

## Scope

- Feature-gated local ML/RAG experiments.
- Evaluation harnesses and failure-mode documentation.
- Privacy, resource and rollback controls.
- User-visible opt-in and disable flows.

## Functional Requirements

- ML/RAG remains disabled by default.
- Frontier dependencies require owner, feature gate, rollback plan and exit criteria.
- No frontier path may bypass document privacy, policy review or mutation safety.

## Non-Functional Requirements

- Feature gate: `frontier_intelligence_preview`.
- Rollback: disable frontier flags and fall back to deterministic extraction/search/workflows.
- Exit criteria: privacy, security, evaluation and resource-budget evidence justify any preview promotion.

## Acceptance Criteria

- Frontier preview policy exists.
- Evaluation datasets avoid private document leakage.
- Runtime defaults keep frontier features off.

## Out of Scope

- Enabling ML/RAG in early launch waves.
- Remote telemetry.
- Cloud-only assistance.
