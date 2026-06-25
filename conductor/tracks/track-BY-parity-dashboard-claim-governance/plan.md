# Implementation Plan

## Phase BY1: Dashboard Data Model

- [ ] Task: Define local dashboard data generated from platform parity, PDF parity and QA reports.
- [ ] Task: Add a deterministic dashboard generator under `target/platform-reports/`.
- [ ] Task: Add checker coverage for supported, plan-only, documented limitation and external-gate counts.
- [ ] Task: Run the dashboard checker.

## Phase BY2: Claim Governance

- [ ] Task: Scan public docs and release surfaces for platform parity claims.
- [ ] Task: Fail claims that lack supported matrix evidence or documented limitations.
- [ ] Task: Preserve separate external-gate wording for signing, stores and registries.
- [ ] Task: Run claim-governance checks with PDF parity checks.

## Phase BY3: Conductor Closeout

- [ ] Task: Update docs/homepage surfaces only with evidence-backed wording.
- [ ] Task: Run `python3 scripts/platform_parity_matrix_check.py`, `python3 scripts/pdf_parity_registry_check.py` and dashboard checks.
- [ ] Task: Run `conductor-review`, apply high-confidence fixes and rerun focused validation.
- [ ] Task: Commit, add a `refs/notes/conductor` note, push commits and notes, wait for required GitHub Actions, then archive the track.
