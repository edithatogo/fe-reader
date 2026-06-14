# Track AY: Post-launch PDF Baseline Parity

## Overview

Build post-launch evidence for industry baseline PDF capabilities without weakening the stable desktop launch gates. This track does not block desktop stable launch.

## Scope

- Reading and malformed-PDF recovery evidence.
- Rendering and visual regression evidence.
- Page organization, annotations, forms, metadata and redaction verification.
- Conversion and source pipeline limitations.
- User-facing limitation notes where parity is incomplete.

## Functional Requirements

- Map each baseline PDF capability to a fixture, contract smoke, differential oracle or documented limitation.
- Keep all write-capable features on the `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt` pipeline.
- Ensure `fe_reader_core` remains free of UI, renderer, platform, plugin and ML dependencies.

## Non-Functional Requirements

- Feature gate: `advanced_pdf_baseline`.
- Rollback: remove claims from release docs and disable incomplete workflow packs.
- Exit criteria: every public claim has evidence or a documented limitation.

## Acceptance Criteria

- Baseline parity matrix exists and is linked from docs.
- All supported claims have tests or evidence artifacts.
- Unsupported claims remain explicit limitations.

## Out of Scope

- ML/RAG features.
- Cloud collaboration.
- Mobile store launch.
