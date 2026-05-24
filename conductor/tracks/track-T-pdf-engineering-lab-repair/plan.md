# Track T: PDF Engineering Lab & Repair Recovery

## Mission

Build the read-only diagnostics, safe-open and repair-planning layer for malformed, hostile and complex PDFs.

## Dependencies

- A0 document identity, operation intent and patch-plan types.
- P0 corpus manifest.
- O0 security policy.

## Phases

### T0 Lab contracts

Deliver:

- `contracts/rust/pdf_lab.rs`
- `contracts/rust/recovery.rs`
- `schemas/pdf-lab-session.schema.json`
- `schemas/recovery-report.schema.json`
- CLI command skeletons for `lab` and `repair`.

### T1 Object/page graph inspection

Deliver object tree, trailer, xref, page tree and resource inspection for simple fixtures.

### T2 Content/text/font diagnostics

Deliver content stream disassembly, text/glyph mapping diagnostics and annotation/form inspection.

### T3 Safe-open and repair planning

Deliver safe-open mode, repair plan generation and repair receipt schema. Repair must save copy only.

### T4 Timeline and leak scanning

Deliver incremental update timeline and redaction residual object scanner.

## Review gates

Run schema validation, security policy check, repair smoke and corpus manifest validation.
