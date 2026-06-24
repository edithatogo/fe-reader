# Track BK: Professional Workflow Parity Plan

## Phase BK1 - Annotation and comment parity

- [ ] Task: Implement annotation round trips.
    - [ ] Support common annotation types with fixtures and preservation tests.
    - [ ] Add UI and CLI workflows for view, add, edit and delete where approved.
    - [ ] Verify renderer/search/metadata consistency.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BK1 --auto-fix`.

## Phase BK2 - Forms parity

- [ ] Task: Implement AcroForm workflows.
    - [ ] Inspect fields, values, appearances and validation hints.
    - [ ] Plan fill and flatten operations through policy.
    - [ ] Detect XFA/JavaScript dependency and warn without executing unsafe content.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BK2 --auto-fix`.

## Phase BK3 - Page organization and metadata parity

- [ ] Task: Implement safe document organization.
    - [ ] Apply reorder, rotate, delete, extract and merge operations with transaction journals.
    - [ ] Apply metadata scrub/preserve policies.
    - [ ] Emit audit receipts and verification reports.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BK3 --auto-fix`.

## Phase BK4 - Redaction and signature-aware workflows

- [ ] Task: Harden high-risk workflows.
    - [ ] Verify secure redaction full rewrite and leak scans.
    - [ ] Detect signatures and permission flags before mutation.
    - [ ] Add public limitation text for signing, validation and unsupported cases.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BK4 --auto-fix`.
