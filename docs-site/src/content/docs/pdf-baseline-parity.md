---
title: PDF Baseline Parity
description: Evidence boundary for post-launch PDF baseline capability claims.
---

Fe Reader tracks post-launch PDF baseline parity through the `advanced_pdf_baseline` feature gate and the exhaustive registry in `docs/pdf-parity-registry.md`. This work does not block desktop stable launch.

The canonical matrix is maintained in the repository:

- `docs/pdf-parity-registry.md`
- `docs/pdf-parity-registry.json`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/pdf-baseline-parity-matrix.json`

Every public claim must point to a fixture, contract smoke, visual regression report, differential oracle, or documented limitation. Write-capable features must remain on:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Validate the claim boundary with:

```bash
python3 scripts/pdf_parity_registry_check.py
python3 scripts/pdf_baseline_parity_check.py
```

Current post-launch limitation categories include production renderer parity, editable annotation preservation, XFA and JavaScript forms, external converter fidelity, signed documents, attachments/portfolios, and OCR/searchable-PDF workflows. See `docs/pdf-parity-registry.md`.
