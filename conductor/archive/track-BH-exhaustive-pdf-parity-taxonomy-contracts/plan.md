# Track BH: Exhaustive PDF Parity Taxonomy and Contracts Plan

## Phase BH1 - Taxonomy model

- [x] Task: Define exhaustive parity families.
    - [x] Inventory PDF ISO/version/feature families and common real-world document classes.
    - [x] Define support levels: stable, beta, preview, plan-only, oracle-only, blocked and documented limitation.
    - [x] Define evidence classes and release-blocking thresholds.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BH1 --auto-fix`.

## Phase BH2 - Schemas and contracts

- [x] Task: Add parity schemas and Rust contracts.
    - [x] Create `schemas/pdf-parity-claim.schema.json`.
    - [x] Create `schemas/pdf-parity-registry.schema.json`.
    - [x] Create `contracts/rust/pdf_parity.rs`.
    - [x] Validate that mutating families require the operation pipeline.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BH2 --auto-fix`.

## Phase BH3 - Registry and validator

- [x] Task: Build exhaustive registry and validation script.
    - [x] Create `docs/pdf-parity-registry.json`.
    - [x] Add `scripts/pdf_parity_registry_check.py`.
    - [x] Ensure every claim has evidence, oracle requirements or a limitation.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BH3 --auto-fix`.

## Phase BH4 - Public claim enforcement

- [x] Task: Enforce marketing and release claim alignment.
    - [x] Scan README, docs site, release notes and homepage copy for PDF capability claims.
    - [x] Fail on unregistered or over-supported claims.
    - [x] Update launch limitations to point at the exhaustive registry.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BH4 --auto-fix`.
