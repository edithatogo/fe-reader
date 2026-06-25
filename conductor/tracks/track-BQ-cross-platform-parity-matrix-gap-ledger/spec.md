# Track BQ: Cross-Platform Parity Matrix and Gap Ledger

## Overview

Create the canonical cross-platform parity matrix for Fe Reader across macOS, Windows, Linux, Android, iOS, Web/PWA and browser extension surfaces. The matrix is the implementation ledger for the later BQ-BY tracks and must distinguish current support from future targets.

## Functional Requirements

- Maintain `docs/platform-parity-matrix.json` as the machine-readable source of truth.
- Validate the matrix with `schemas/platform-parity-matrix.schema.json` and `scripts/platform_parity_matrix_check.py`.
- Classify every platform/capability cell as `supported`, `plan-only`, `documented-limitation` or `external-gate`.
- Cover reader, diagnostics, workflow planning, mutation apply and build/release artifact capabilities.
- Bind public parity claims to evidence paths or explicit limitations.

## Non-Functional Requirements

- Do not add platform, UI, renderer, web, MCP, plugin or ML dependencies to `fe_reader_core`.
- Keep signing, notarization, store submission and external registry publication as explicit external gates.
- Keep frontier ML/RAG disabled.

## Acceptance Criteria

- `python3 scripts/platform_parity_matrix_check.py` passes.
- `python3 scripts/validate_schemas.py` passes.
- PR Contracts runs the new checker.
- The matrix is usable as the gap ledger for BQ-BY implementation.

## Out of Scope

- Implementing the actual platform shells.
- Claiming public release parity for external-gated surfaces.
