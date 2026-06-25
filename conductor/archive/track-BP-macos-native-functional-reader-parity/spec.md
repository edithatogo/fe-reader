# Track BP: macOS Native Functional Reader Parity Specification

## Overview

Make the native macOS shell function as a real local PDF reader rather than a preview-only shell. The implementation must keep `fe_reader_core` pure and route native UI behavior through platform/native adapters and the existing `fe-reader` CLI where core evidence is needed.

## Functional Requirements

- Render opened PDFs in the native canvas using PDFKit `PDFView`.
- Provide page navigation, current page display, zoom in/out, fit width, actual size, and keyboard shortcuts.
- Restore the current reader session for the opened document within the app session.
- Resolve a bundled or locally built `fe-reader` CLI and use it for deterministic Inspect, Metadata, Search, Accessibility, and Validate evidence where available.
- Show CLI/core results in the inspector without leaking document text into support-style status copy.
- Provide a search field that highlights matches in the visible PDF and reports deterministic CLI search status when the CLI is available.
- Keep Redact disabled or plan-only until the mutation pipeline can produce `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Add verification mode for empty-state and fixture-open macOS screenshots under `target/native-preview/` by default.

## Non-Functional Requirements

- Do not add UI, renderer, platform, plugin, MCP, or ML dependencies to `fe_reader_core`.
- Do not enable local ML, RAG, cloud sync, analytics, telemetry, or hidden network behavior.
- Keep the native app local-first and readable with keyboard-only operation.
- Keep signing and notarization as external release gates; do not weaken those checks.

## Acceptance Criteria

- `script/build_and_run.sh --verify` captures an empty-state screenshot under `target/native-preview/`.
- `script/build_and_run.sh --verify --fixture fixtures/corpus/basic/minimal-v1_0.pdf` captures an open-document screenshot under `target/native-preview/`.
- `scripts/native_preview_check.py` validates both screenshots.
- `swiftc` builds the native macOS app without duplicate or placeholder-only action implementations.
- `python3 scripts/conductor_lifecycle_check.py`, `python3 scripts/launch_qa_check.py`, `python3 scripts/stable_reader_readiness_check.py`, `python3 scripts/pdf_parity_registry_check.py`, and `python3 scripts/pdf_baseline_parity_check.py` pass.

## Out of Scope

- Developer ID signing, notarization, App Store submission, or registry publication.
- Direct redaction apply, PDF mutation, or bypassing policy review.
- Local ML, RAG, cloud collaboration, or marketplace publication.
