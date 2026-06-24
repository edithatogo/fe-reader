# Track BK: Professional Workflow Parity Plan

## Phase BK1 - Annotation and comment parity

- [x] Task: Implement annotation round trips.
    - [x] Support common annotation types with fixtures and preservation tests.
    - [x] Add UI and CLI workflows for view, add, edit and delete where approved.
    - [x] Verify renderer/search/metadata consistency.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BK1 --auto-fix`.

## Phase BK2 - Forms parity

- [x] Task: Implement AcroForm workflows.
    - [x] Inspect fields, values, appearances and validation hints.
    - [x] Plan fill and flatten operations through policy.
    - [x] Detect XFA/JavaScript dependency and warn without executing unsafe content.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BK2 --auto-fix`.

## Phase BK3 - Page organization and metadata parity

- [x] Task: Implement safe document organization.
    - [x] Apply reorder, rotate, delete, extract and merge operations with transaction journals.
    - [x] Apply metadata scrub/preserve policies.
    - [x] Emit audit receipts and verification reports.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BK3 --auto-fix`.

## Phase BK4 - Redaction and signature-aware workflows

- [x] Task: Harden high-risk workflows.
    - [x] Verify secure redaction full rewrite and leak scans.
    - [x] Detect signatures and permission flags before mutation.
    - [x] Add public limitation text for signing, validation and unsupported cases.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BK4 --auto-fix`.

Track BK completed after BK1-BK4 phase gates passed on 2026-06-24.
