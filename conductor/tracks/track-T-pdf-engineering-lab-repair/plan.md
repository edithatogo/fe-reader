# Track T: PDF Engineering Lab & Repair Recovery

## Mission

Build the read-only diagnostics, safe-open and repair-planning layer for malformed, hostile and complex PDFs.

## Dependencies

- A0 document identity, operation intent and patch-plan types.
- P0 corpus manifest.
- O0 security policy.

## Phases

### T0 Lab contracts

- [x] Deliver `contracts/rust/pdf_lab.rs`.
- [x] Deliver `contracts/rust/recovery.rs`.
- [x] Deliver `schemas/pdf-lab-session.schema.json`.
- [x] Deliver `schemas/recovery-report.schema.json`.
- [x] Add CLI command skeletons for `lab` and `repair`.
- [x] Verify with schema validation and the lab smoke checks.

### T1 Object/page graph inspection

- [x] Deliver object tree, trailer, xref, page tree and resource inspection for simple fixtures.
- [x] Emit page-box summaries and parser-backed diagnostics.
- [x] Verify with `scripts/pdf_lab_inspect_smoke.sh`.

### T2 Content/text/font diagnostics

- [x] Deliver content stream disassembly, text/glyph mapping diagnostics and annotation/form inspection.
- [x] Verify the page-level fallback geometry used by the current text-map path.
- [x] Verify with `scripts/pdf_lab_text_map_smoke.sh`.

### T3 Safe-open and repair planning

- [x] Deliver safe-open mode, repair plan generation and repair receipt schema.
- [x] Keep repair copy-only by default.
- [x] Verify with `scripts/pdf_repair_smoke.sh`.

### T4 Timeline and leak scanning

- [x] Deliver incremental update timeline reporting.
- [x] Deliver redaction residual object scanner.
- [x] Add dedicated smoke coverage for the new diagnostics.

## Evidence

- `contracts/rust/pdf_lab.rs`
- `contracts/rust/recovery.rs`
- `schemas/pdf-lab-session.schema.json`
- `schemas/recovery-report.schema.json`
- `scripts/pdf_lab_inspect_smoke.sh`
- `scripts/pdf_lab_text_map_smoke.sh`
- `scripts/pdf_lab_timeline_smoke.sh`
- `scripts/pdf_lab_redaction_scan_smoke.sh`
- `scripts/pdf_repair_smoke.sh`
- `docs/pdf-engineering-lab.md`
- `docs/pdf-repair-recovery.md`

## Review gates

- `python3 scripts/validate_schemas.py`
- `bash scripts/security_policy_check.sh`
- `bash scripts/pdf_lab_inspect_smoke.sh`
- `bash scripts/pdf_lab_text_map_smoke.sh`
- `bash scripts/pdf_repair_smoke.sh`
