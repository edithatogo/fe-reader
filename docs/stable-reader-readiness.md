# Stable Reader Readiness

Fe Reader is a local-first PDF workflow platform with a reader-first stable baseline that is evidence-gated rather than marketing-gated.

## Reader baseline

- Open local PDFs from CLI and preview entry points.
- Inspect metadata, search deterministically and surface safe-open diagnostics.
- Expose keyboard and accessibility evidence for the reader baseline.

## Professional workflow boundary

- Annotations, forms, redaction, conversion and related operations remain policy-gated.
- Mutating workflows use `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Unsupported or incomplete workflows are documented as limitations rather than hidden.

## Evidence

- `scripts/stable_reader_readiness_check.py`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/stable-release-evidence.json`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/launch-limitations-support.md`

## Marketing boundary

The stable-reader baseline does not by itself make the product ready for broad marketing. Stable publication still requires signed artifacts, checksums, release evidence and registry approval.
