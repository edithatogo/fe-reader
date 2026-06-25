# Implementation Plan

## Phase BQ1: Matrix Contract

- [ ] Task: Review `docs/platform-parity-matrix.json`, `schemas/platform-parity-matrix.schema.json` and `scripts/platform_parity_matrix_check.py`.
- [ ] Task: Ensure the matrix covers all target platforms and the required full-workflow capability families.
- [ ] Task: Add missing evidence paths or limitations without inflating support claims.
- [ ] Task: Run `python3 scripts/platform_parity_matrix_check.py`.

## Phase BQ2: Claim Boundary

- [ ] Task: Align matrix statuses with `docs/pdf-baseline-parity-matrix.json`, `docs/platform-integration-plan.md` and `packaging/registry-status.yaml`.
- [ ] Task: Ensure external release gates are represented as `external-gate`, not pending implementation.
- [ ] Task: Confirm mutating capabilities require the canonical mutation pipeline.
- [ ] Task: Run `python3 scripts/pdf_baseline_parity_check.py` and `python3 scripts/platform_parity_matrix_check.py`.

## Phase BQ3: Conductor Closeout

- [ ] Task: Run `python3 scripts/validate_schemas.py`.
- [ ] Task: Run `python3 scripts/conductor_lifecycle_check.py --require-git-note`.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
