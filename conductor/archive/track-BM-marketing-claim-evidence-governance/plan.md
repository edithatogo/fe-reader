# Track BM: Marketing Claim Evidence Governance Plan

## Phase BM1 - Readiness levels

- [x] Task: Define marketing readiness levels.
    - [x] Specify technical preview, public beta, stable desktop, mature stable and v2 roadmap criteria.
    - [x] Link criteria to release artifacts, evidence, support and parity status.
    - [x] Add rollback rules for overclaimed features.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BM1 --auto-fix`.

## Phase BM2 - Claim scanner

- [x] Task: Implement marketing claim checks.
    - [x] Scan README, docs site, release notes, package metadata and homepage.
    - [x] Link claims to parity registry and release readiness reports.
    - [x] Fail on unsupported stable/productivity/parity claims.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BM2 --auto-fix`.

## Phase BM3 - Marketing assets and screenshots

- [x] Task: Produce evidence-backed assets.
    - [x] Capture screenshots only from real working builds.
    - [x] Add feature copy tied to implemented flows.
    - [x] Add limitation and support links near download calls to action.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BM3 --auto-fix`.

## Phase BM4 - Approval and publication gate

- [x] Task: Gate marketing publication.
    - [x] Add maintainer approval checklist.
    - [x] Attach marketing readiness to release evidence.
    - [x] Block broad marketing until stable artifacts and claim evidence pass.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BM4 --auto-fix`.
