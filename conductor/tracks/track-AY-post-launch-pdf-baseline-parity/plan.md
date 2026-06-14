# Track AY: Post-launch PDF Baseline Parity Plan

## Phase AY1 - Capability evidence matrix

- [x] Task: Create baseline PDF capability matrix.
    - [x] Map reading, rendering, editing, forms, metadata, redaction and conversion claims.
    - [x] Add evidence path or limitation for every claim.
    - [x] Confirm the feature gate `advanced_pdf_baseline` is documented.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AY1 --auto-fix`.

## Phase AY2 - Fixture and oracle expansion

- [x] Task: Expand evidence for high-value baseline gaps.
    - [x] Add corpus fixtures for malformed and common production PDFs.
    - [x] Add differential oracle or visual regression checks where needed.
    - [x] Add rollback notes for unsupported claims.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AY2 --auto-fix`.

## Phase AY3 - Public claim alignment

- [x] Task: Align README/docs with evidence.
    - [x] Remove or qualify unsupported claims.
    - [x] Link exit criteria evidence from release docs.
    - [x] Keep this track non-blocking unless a launch-critical defect is found.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AY3 --auto-fix`.

## Completion Evidence

- Added `docs/pdf-baseline-parity-matrix.json` and `docs/pdf-baseline-parity-matrix.md`.
- Added `scripts/pdf_baseline_parity_check.py` to enforce required families, evidence paths, limitations and mutation-pipeline coverage.
- Added placeholder fixture classes for attachments/portfolios and scanned OCR limitations.
- Linked the matrix from README, launch limitations, stable desktop release docs and the Astro/Starlight docs site.
- Ran `python3 scripts/pdf_baseline_parity_check.py`.
- Ran `python3 scripts/corpus_manifest_validate.py`.
- Ran `python3 scripts/compatibility_corpus_report.py`.
- Ran `python3 scripts/ci_policy_check.py`.
- Ran `npm --prefix docs-site run build`.
- Ran `scripts/conductor_phase_gate.sh --phase AY1 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AY2 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AY3 --auto-fix`.
