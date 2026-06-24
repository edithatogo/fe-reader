# Track BJ: Reader, Render, Search and Accessibility Parity Plan

## Phase BJ1 - Render and navigation baseline

- [ ] Task: Implement and verify reader navigation.
    - [ ] Add open, page navigation, zoom, fit, rotation, thumbnails/page list and outline behavior.
    - [ ] Add visual regression fixtures for common page types.
    - [ ] Add performance budgets for open and navigation.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BJ1 --auto-fix`.

## Phase BJ2 - Text and search parity

- [ ] Task: Harden extraction and search.
    - [ ] Add text span, geometry, ligature, CJK, RTL and missing ToUnicode diagnostics.
    - [ ] Add golden tests and oracle comparisons.
    - [ ] Surface limitations in UI and CLI.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BJ2 --auto-fix`.

## Phase BJ3 - OCR and searchable-PDF provider gate

- [ ] Task: Implement OCR capability discovery.
    - [ ] Add opt-in provider contracts.
    - [ ] Add searchable-PDF planning without default network or ML enablement.
    - [ ] Add fixtures and release limitations for OCR quality.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BJ3 --auto-fix`.

## Phase BJ4 - Accessibility parity

- [ ] Task: Implement accessibility reports and UI parity.
    - [ ] Add tagged-PDF inspection.
    - [ ] Add keyboard and screen-reader smoke coverage.
    - [ ] Add PDF/UA oracle integration where available.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BJ4 --auto-fix`.
