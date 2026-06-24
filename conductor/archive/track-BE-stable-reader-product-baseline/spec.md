# Track BE: Stable Reader Product Baseline

## Overview

Make Fe Reader genuinely usable as a stable desktop PDF reader before broad marketing. This track converts the current evidence-heavy preview into a reader-first, professional-workflow-capable product baseline.

## Functional Requirements

- Open local PDFs from file picker, drag-and-drop, command line and recent files.
- Provide page navigation, thumbnails or page list, zoom, fit modes, rotation view, document outline where available and session restore.
- Provide deterministic search, text extraction diagnostics, metadata view, safe-open diagnostics and visible limitations for malformed or unsupported documents.
- Provide reader-adjacent professional workflows: annotation planning/viewing, AcroForm inspection/fill planning, metadata scrub/preserve planning, redaction planning/verification, export/conversion capability discovery and audit receipts.
- Keep all writes on `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Provide CLI parity for every launch-critical reader and workflow claim.

## Non-Functional Requirements

- `fe_reader_core` remains free of UI, platform, renderer, plugin and ML dependencies.
- The product must remain local-first with no telemetry, upload, sync or analytics by default.
- Accessibility must cover keyboard paths, labels and error states for all stable reader workflows.
- Performance budgets must cover startup, open, navigation, search and render smoke paths.

## Acceptance Criteria

- A stable-reader readiness check fails until all baseline reader claims have working UI, CLI or documented limitation evidence.
- Launch docs distinguish supported stable reader features from preview, plan-only and documented-limitation features.
- Cross-platform smoke tests exercise open, inspect, navigate/search where UI automation exists.
- No broad marketing claim exceeds the evidence in the stable-reader readiness report.

## Out of Scope

- ML/RAG features.
- Cloud collaboration.
- Marketplace/plugin publication.
- Full mobile store launch.
