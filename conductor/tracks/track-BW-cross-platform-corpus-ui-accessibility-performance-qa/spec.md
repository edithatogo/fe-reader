# Track BW: Cross-Platform Corpus, UI, Accessibility and Performance QA

## Overview

Create a shared QA harness that runs comparable corpus, UI, accessibility and performance smoke scenarios across desktop, mobile, web and browser surfaces where feasible.

## Functional Requirements

- Define a shared fixture set for platform parity smoke.
- Add UI smoke coverage for open, search, inspect, validate and plan-only workflow actions.
- Add accessibility and keyboard/touch path evidence per platform.
- Add startup/open/search performance budgets and reports per platform.
- Feed evidence back into `docs/platform-parity-matrix.json`.

## Non-Functional Requirements

- Tests must remain deterministic and CI-safe.
- Heavy device or real-hardware traces may be recorded as optional external evidence rather than hard PR gates.

## Acceptance Criteria

- QA reports are generated under `target/platform-reports/`.
- Platform Tests or a new explicit workflow runs the safe subset.
- Platform parity matrix and claim governance use the QA reports as evidence.

## Out of Scope

- Exhaustive corpus certification.
- Real-device lab requirements as mandatory PR gates.
