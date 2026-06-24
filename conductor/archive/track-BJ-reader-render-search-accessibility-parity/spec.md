# Track BJ: Reader, Render, Search and Accessibility Parity

## Overview

Implement the user-visible reading features that make Fe Reader a usable, modern and advanced PDF reader.

## Functional Requirements

- Provide production rendering for common PDFs with visual regression and oracle evidence.
- Implement navigation, zoom, thumbnails, outlines, page labels, rotation view and session restore.
- Implement deterministic search, text extraction diagnostics, missing ToUnicode warnings, CJK/RTL handling and OCR/searchable-PDF provider gating.
- Provide tagged-PDF and accessibility report workflows.
- Keep OCR and external providers opt-in with capability discovery and clear failure modes.

## Non-Functional Requirements

- Rendering must have CPU fallback and must not require experimental GPU paths.
- Accessibility must be keyboard and screen-reader testable.
- Performance budgets must cover startup, page open, scroll/navigation, search and OCR planning.

## Acceptance Criteria

- Stable reader claims pass visual, text, accessibility and performance evidence gates.
- Unsupported text/OCR/accessibility cases appear as documented limitations, not silent failures.
- UI and CLI expose consistent diagnostics.

## Out of Scope

- Local ML/RAG.
- Cloud OCR services by default.
