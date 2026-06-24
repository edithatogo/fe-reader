# Exhaustive PDF Parity Taxonomy and Contracts

Fe Reader meets the exhaustive PDF parity taxonomy and contracts only when the canonical registry and nested baseline matrix are present, internally consistent, and linked from the release documentation.

## Contract

- `docs/pdf-parity-registry.md` is the canonical PDF capability claim registry.
- `docs/pdf-parity-registry.json` is the machine-readable taxonomy source of truth.
- `docs/pdf-baseline-parity-matrix.md` remains the nested post-launch baseline boundary.
- `docs/pdf-baseline-parity-matrix.json` remains the machine-readable nested matrix.
- Every public PDF capability claim points back to the registry or uses one of the exact claim ids defined there.
- Mutating families remain bound to the `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt` pipeline.
- Stable desktop launch stays non-blocking on post-launch PDF parity claims.

## Required evidence

- `docs/pdf-parity-registry.md`
- `docs/pdf-parity-registry.json`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/pdf-baseline-parity-matrix.json`
- `scripts/pdf_parity_registry_check.py`
- `scripts/pdf_baseline_parity_check.py`

## Taxonomy scope

The exhaustive registry covers reading, search, rendering, page organization, annotations, forms, metadata, redaction, conversion, signatures, attachments/portfolios, OCR, accessibility, standards, repair and other PDF families.

This contract does not claim that every family is fully supported. It requires the exhaustive registry, the nested baseline matrix and the documented limitation boundary to stay synchronized.
