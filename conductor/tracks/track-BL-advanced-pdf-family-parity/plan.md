# Track BL: Advanced PDF Family Parity Plan

## Phase BL1 - Standards and preflight parity

- [ ] Task: Implement standards diagnostics.
    - [ ] Integrate PDF/A, PDF/UA and PDF/X validator discovery.
    - [ ] Emit explainable reports with object references.
    - [ ] Add oracle-backed fixtures and limitations.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BL1 --auto-fix`.

## Phase BL2 - Attachments, portfolios and associated files

- [ ] Task: Implement embedded-file inspection and safe extraction planning.
    - [ ] Add fixtures for attachments, portfolios and associated files.
    - [ ] Add policy gates for extraction and mutation.
    - [ ] Add audit receipt and quarantine semantics.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BL2 --auto-fix`.

## Phase BL3 - Layers, active content and multimedia

- [ ] Task: Implement diagnostics for optional content and active content.
    - [ ] Inspect OCG/layers and visibility.
    - [ ] Quarantine JavaScript, launch actions, rich media and embedded executables.
    - [ ] Expose user-facing warnings and CLI reports.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BL3 --auto-fix`.

## Phase BL4 - Encryption, permissions and prepress

- [ ] Task: Implement encryption and prepress diagnostics.
    - [ ] Inspect encryption, permissions and signature risk.
    - [ ] Inspect color, output intent, spot color, overprint and font state.
    - [ ] Add validation and limitation evidence.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BL4 --auto-fix`.
