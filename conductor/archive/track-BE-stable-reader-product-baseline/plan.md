# Track BE: Stable Reader Product Baseline Plan

## Phase BE1 - Stable reader acceptance contract

- [ ] Task: Define stable reader readiness schema and report.
    - [ ] Enumerate reader-first, professional-workflow and competitive desktop baseline requirements.
    - [ ] Map every requirement to UI, CLI, contract, fixture, smoke test or documented limitation evidence.
    - [ ] Add a release-blocking check for stable marketing claims.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BE1 --auto-fix`.

## Phase BE2 - Reader workflow implementation evidence

- [ ] Task: Implement or harden stable reader flows.
    - [ ] Verify local open, recents, page navigation, zoom, search, metadata and safe-open diagnostics.
    - [ ] Verify keyboard and accessibility paths.
    - [ ] Verify CLI parity for inspect, search, metadata and policy-visible plan flows.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BE2 --auto-fix`.

## Phase BE3 - Professional workflow launch boundary

- [ ] Task: Gate professional workflows.
    - [ ] Classify annotations, forms, metadata, redaction, conversion and audit receipts as supported, plan-only, preview or limitation.
    - [ ] Ensure every mutating workflow remains policy-gated and emits or plans audit receipts.
    - [ ] Update release notes, README and docs site claim language.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BE3 --auto-fix`.

## Phase BE4 - Stable reader release evidence

- [ ] Task: Produce release candidate evidence.
    - [ ] Run local launch QA, stable reader readiness, accessibility, performance and compatibility checks.
    - [ ] Attach stable-reader readiness to release evidence.
    - [ ] Block marketing if any required stable reader surface is placeholder-only.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BE4 --auto-fix`.
