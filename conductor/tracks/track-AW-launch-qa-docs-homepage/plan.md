# Track AW: Launch QA, Documentation and Homepage Plan

## Phase AW1 - Launch QA command

- [ ] Task: Add a launch QA aggregator.
    - [ ] Run desktop smoke checks.
    - [ ] Run CLI and contract checks.
    - [ ] Run compatibility/performance/accessibility/security/visual checks or validate evidence.
    - [ ] Emit a concise launch readiness summary.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AW1 --auto-fix`.

## Phase AW2 - README and install docs

- [ ] Task: Update public install and verification documentation.
    - [ ] Link release artifacts and checksums.
    - [ ] Document macOS, Windows and Linux install flows.
    - [ ] Document verification commands.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AW2 --auto-fix`.

## Phase AW3 - Docs site and homepage links

- [ ] Task: Update docs/homepage release surfaces.
    - [ ] Add stable desktop release page.
    - [ ] Add registry/package links.
    - [ ] Ensure GitHub homepage metadata represented in repo files is current.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AW3 --auto-fix`.

## Phase AW4 - Limitations and support

- [ ] Task: Document launch limitations and support routes.
    - [ ] Document mobile advisory status.
    - [ ] Document ML/RAG deferral.
    - [ ] Document cloud collaboration deferral.
    - [ ] Link security and support policies.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AW4 --auto-fix`.

## Exit Criteria

- Public docs and launch QA evidence align with actual stable desktop release readiness.

