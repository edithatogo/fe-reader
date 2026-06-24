# Track BI: PDF Corpus and Oracle Factory

## Overview

Create the fixture generation, corpus governance and oracle pipeline required to prove PDF parity claims across real PDF families.

## Functional Requirements

- Generate or import redistributable fixtures for every parity family.
- Track fixture provenance, license, privacy status, expected behavior and oracle requirements.
- Run qpdf, veraPDF, PDFium, Poppler, MuPDF, Ghostscript, ImageMagick, OCR and conversion oracles where available.
- Record skipped or unavailable oracles explicitly.
- Produce release compatibility, visual, extraction, structure and mutation-safety reports.

## Non-Functional Requirements

- No private PDFs may be committed.
- Malformed and hostile PDFs are expected input and must use safe-open contracts.
- Oracle availability is advisory locally but must be recorded in release evidence.

## Acceptance Criteria

- Corpus manifest covers every exhaustive parity family.
- Oracle reports link to parity claims and fixture IDs.
- Release evidence contains a compatibility report with pass/fail/skip status by parity family.

## Out of Scope

- Paid proprietary fixture redistribution unless licensing permits it.
- Automatic repair of hostile PDFs without review.
