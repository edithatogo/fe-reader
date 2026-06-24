# Stable Reader Readiness

Fe Reader is a local-first PDF workflow platform with a reader-first stable baseline that is evidence-gated rather than marketing-gated.
This baseline is the contract that makes the product a usable stable bleeding-edge PDF reader.

## Reader baseline

- Open local PDFs from CLI and preview entry points.
- Inspect metadata, search deterministically and surface safe-open diagnostics.
- Navigate pages, zoom, fit, rotate, inspect thumbnails and restore reader session state.
- Expose keyboard and accessibility evidence for the reader baseline.
- Keep OCR-backed searchable-PDF work provider-gated and opt-in.

## Evidence map

- `scripts/stable_reader_readiness_check.py`
- `scripts/wave1_render_smoke.sh`
- `scripts/perf_smoke.sh`
- `scripts/accessibility_audit_smoke.py`
- `scripts/search_index_smoke.sh`
- `scripts/ocr_searchable_pdf_contract_smoke.py`

## Professional workflow boundary

- Annotations, forms, redaction, conversion and related operations remain policy-gated.
- Mutating workflows use `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Unsupported or incomplete workflows are documented as limitations rather than hidden.

## Evidence

- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/stable-release-evidence.json`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/launch-limitations-support.md`
- `docs/usable-stable-bleeding-edge-pdf-reader-contract.md`
- `docs/reader-render-search-accessibility-parity-contract.md`

## Marketing boundary

The stable-reader baseline does not by itself make the product ready for broad marketing. Stable publication still requires signed artifacts, checksums, release evidence and registry approval.
