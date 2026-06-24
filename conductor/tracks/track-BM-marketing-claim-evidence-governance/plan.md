# Track BM: Marketing Claim Evidence Governance Plan

## Phase BM1 - Readiness levels

- [ ] Task: Define marketing readiness levels.
    - [ ] Specify technical preview, public beta, stable desktop, mature stable and v2 roadmap criteria.
    - [ ] Link criteria to release artifacts, evidence, support and parity status.
    - [ ] Add rollback rules for overclaimed features.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BM1 --auto-fix`.

## Phase BM2 - Claim scanner

- [ ] Task: Implement marketing claim checks.
    - [ ] Scan README, docs site, release notes, package metadata and homepage.
    - [ ] Link claims to parity registry and release readiness reports.
    - [ ] Fail on unsupported stable/productivity/parity claims.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BM2 --auto-fix`.

## Phase BM3 - Marketing assets and screenshots

- [ ] Task: Produce evidence-backed assets.
    - [ ] Capture screenshots only from real working builds.
    - [ ] Add feature copy tied to implemented flows.
    - [ ] Add limitation and support links near download calls to action.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BM3 --auto-fix`.

## Phase BM4 - Approval and publication gate

- [ ] Task: Gate marketing publication.
    - [ ] Add maintainer approval checklist.
    - [ ] Attach marketing readiness to release evidence.
    - [ ] Block broad marketing until stable artifacts and claim evidence pass.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BM4 --auto-fix`.
