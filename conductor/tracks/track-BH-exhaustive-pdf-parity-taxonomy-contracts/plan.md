# Track BH: Exhaustive PDF Parity Taxonomy and Contracts Plan

## Phase BH1 - Taxonomy model

- [ ] Task: Define exhaustive parity families.
    - [ ] Inventory PDF ISO/version/feature families and common real-world document classes.
    - [ ] Define support levels: stable, beta, preview, plan-only, oracle-only, blocked and documented limitation.
    - [ ] Define evidence classes and release-blocking thresholds.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BH1 --auto-fix`.

## Phase BH2 - Schemas and contracts

- [ ] Task: Add parity schemas and Rust contracts.
    - [ ] Create `schemas/pdf-parity-claim.schema.json`.
    - [ ] Create `schemas/pdf-parity-registry.schema.json`.
    - [ ] Create `contracts/rust/pdf_parity.rs`.
    - [ ] Validate that mutating families require the operation pipeline.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BH2 --auto-fix`.

## Phase BH3 - Registry and validator

- [ ] Task: Build exhaustive registry and validation script.
    - [ ] Create `docs/pdf-parity-registry.json`.
    - [ ] Add `scripts/pdf_parity_registry_check.py`.
    - [ ] Ensure every claim has evidence, oracle requirements or a limitation.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BH3 --auto-fix`.

## Phase BH4 - Public claim enforcement

- [ ] Task: Enforce marketing and release claim alignment.
    - [ ] Scan README, docs site, release notes and homepage copy for PDF capability claims.
    - [ ] Fail on unregistered or over-supported claims.
    - [ ] Update launch limitations to point at the exhaustive registry.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BH4 --auto-fix`.
