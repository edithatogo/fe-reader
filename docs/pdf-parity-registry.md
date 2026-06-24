# Exhaustive PDF Parity Registry

This registry is the canonical source for Fe Reader PDF capability claims.
It nests the post-launch `advanced_pdf_baseline` matrix and expands it into a
machine-readable taxonomy for reading, rendering, search, forms, redaction,
repair, conversion, accessibility, standards and other PDF families.

The registry is defined in [`pdf-parity-registry.json`](pdf-parity-registry.json).
Validate it with:

```bash
python3 scripts/pdf_parity_registry_check.py
```

The legacy baseline matrix remains available for Track AY as a nested subset:

- [`pdf-baseline-parity-matrix.md`](pdf-baseline-parity-matrix.md)
- [`pdf-baseline-parity-matrix.json`](pdf-baseline-parity-matrix.json)

Write-capable families still must follow:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Any public doc, release note or homepage claim about PDF capability should link
back to this registry or use the exact claim ids defined here.
The corpus/oracle evidence factory contract is documented in `docs/corpus-oracle-evidence-factory-contract.md`.
