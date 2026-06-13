# Track AW: Launch QA, Documentation and Homepage Plan

## Phase AW1 - Launch QA command

- [x] Task: Add a launch QA aggregator.
    - [x] Run desktop smoke checks.
    - [x] Run CLI and contract checks.
    - [x] Run compatibility/performance/accessibility/security/visual checks or validate evidence.
    - [x] Emit a concise launch readiness summary.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AW1 --auto-fix`.

## Phase AW2 - README and install docs

- [x] Task: Update public install and verification documentation.
    - [x] Link release artifacts and checksums.
    - [x] Document macOS, Windows and Linux install flows.
    - [x] Document verification commands.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AW2 --auto-fix`.

## Phase AW3 - Docs site and homepage links

- [x] Task: Update docs/homepage release surfaces.
    - [x] Add stable desktop release page.
    - [x] Add registry/package links.
    - [x] Ensure GitHub homepage metadata represented in repo files is current.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AW3 --auto-fix`.

## Phase AW4 - Limitations and support

- [x] Task: Document launch limitations and support routes.
    - [x] Document mobile advisory status.
    - [x] Document ML/RAG deferral.
    - [x] Document cloud collaboration deferral.
    - [x] Link security and support policies.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AW4 --auto-fix`.

## Exit Criteria

- Public docs and launch QA evidence align with actual stable desktop release readiness.

## Completion Evidence

- Added `scripts/launch_qa_check.py`, wired it into the release workflow, and recorded `target/release-evidence/launch-qa.json`.
- Added stable desktop install/verification docs, launch limitations, support routing and Starlight release page.
- Updated README and repository metadata with release, QA, support and security links.
- Passed focused checks: `python3 scripts/launch_qa_check.py`, `python3 scripts/release_provenance_check.py`, `python3 scripts/ci_policy_check.py`, and `npm run build` in `docs-site/`.
- Passed phase gates: `scripts/conductor_phase_gate.sh --phase AW1 --auto-fix`, `AW2`, `AW3`, and `AW4`.
