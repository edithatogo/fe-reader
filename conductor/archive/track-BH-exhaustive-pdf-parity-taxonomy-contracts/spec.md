# Track BH: Exhaustive PDF Parity Taxonomy and Contracts

## Overview

Define PDF parity exhaustively enough that Fe Reader can make mature claims without relying on vague feature lists. This track turns parity into a machine-readable contract registry covering supported, preview, plan-only, blocked and documented-limitation states.

## Functional Requirements

- Define a PDF parity taxonomy covering reading, rendering, navigation, search, text extraction, images, fonts, color, annotations, comments, forms, JavaScript/XFA warnings, signatures, encryption, permissions, attachments, portfolios, layers, multimedia, tagged PDFs, accessibility, PDF/A, PDF/UA, PDF/X, PDF 2.0, redaction, page organization, optimization, linearization, repair, incremental updates, OCR/searchable PDFs, conversion and printing/export.
- Create schemas for parity claims, evidence classes, support levels, risk levels, oracle requirements, fixtures and public claim text.
- Map every claim to required contracts, corpus fixtures, oracle expectations and release gates.
- Preserve mutation pipeline requirements for every write-capable claim.

## Non-Functional Requirements

- Do not claim support for any PDF class without evidence or an explicit limitation.
- Keep feature gates and rollback paths attached to every parity family.
- Use neutral competitive-baseline language without cloning or naming any vendor as the product target.

## Acceptance Criteria

- A machine-readable exhaustive parity registry exists and validates.
- Public docs derive from or link to the parity registry.
- CI fails if marketing or release docs contain unregistered PDF capability claims.
- Existing `advanced_pdf_baseline` matrix is either superseded or nested under the exhaustive registry.

## Out of Scope

- Implementing every parity feature in this track.
- Adding unsafe JavaScript or XFA execution.
