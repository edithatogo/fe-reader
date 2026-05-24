# Track U: Differential Testing & Interoperability Oracles

## Mission

Create a repeatable oracle harness comparing Fe Reader outputs to external tools and engines.

## Oracles

- qpdf for syntax/structure.
- veraPDF for PDF/A and PDF/UA.
- PDFium for rendering baseline.
- Poppler, MuPDF and Ghostscript when available.
- Pandoc and LibreOffice for conversion provider comparisons.

## Phases

### U0 Report schema and runner

Deliver `contracts/rust/differential_oracle.rs`, `schemas/differential-test-report.schema.json` and `scripts/differential_oracle_smoke.sh`.

### U1 Rendering oracle

Compare one-page visual output against available renderer oracles and record acceptable deltas.

### U2 Text/metadata oracle

Compare extraction and metadata snapshots against available tools.

### U3 Repair/redaction oracle

Compare repaired and redacted outputs using syntax, extraction and leak-scan checks.

### U4 Standards/conversion oracle

Run PDF/A, PDF/UA, PDF/X and conversion checks where tools are installed.

## Policy

Unavailable oracles are advisory in local development, but release CI must record which oracles were available and which were skipped.
