# Track AY: Post-launch PDF Baseline Parity Plan

## Phase AY1 - Capability evidence matrix

- [ ] Task: Create baseline PDF capability matrix.
    - [ ] Map reading, rendering, editing, forms, metadata, redaction and conversion claims.
    - [ ] Add evidence path or limitation for every claim.
    - [ ] Confirm the feature gate `advanced_pdf_baseline` is documented.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AY1 --auto-fix`.

## Phase AY2 - Fixture and oracle expansion

- [ ] Task: Expand evidence for high-value baseline gaps.
    - [ ] Add corpus fixtures for malformed and common production PDFs.
    - [ ] Add differential oracle or visual regression checks where needed.
    - [ ] Add rollback notes for unsupported claims.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AY2 --auto-fix`.

## Phase AY3 - Public claim alignment

- [ ] Task: Align README/docs with evidence.
    - [ ] Remove or qualify unsupported claims.
    - [ ] Link exit criteria evidence from release docs.
    - [ ] Keep this track non-blocking unless a launch-critical defect is found.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AY3 --auto-fix`.
