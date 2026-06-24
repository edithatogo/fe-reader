# Track BJ: Reader, Render, Search and Accessibility Parity Plan

## Phase BJ1 - Render and navigation baseline

- [x] Task: Implement and verify reader navigation.
    - [x] Add open, page navigation, zoom, fit, rotation, thumbnails/page list and outline behavior.
    - [x] Add visual regression fixtures for common page types.
    - [x] Add performance budgets for open and navigation.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BJ1 --auto-fix`.

## Phase BJ2 - Text and search parity

- [x] Task: Harden extraction and search.
    - [x] Add text span, geometry, ligature, CJK, RTL and missing ToUnicode diagnostics.
    - [x] Add golden tests and oracle comparisons.
    - [x] Surface limitations in UI and CLI.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BJ2 --auto-fix`.

## Phase BJ3 - OCR and searchable-PDF provider gate

- [x] Task: Implement OCR capability discovery.
    - [x] Add opt-in provider contracts.
    - [x] Add searchable-PDF planning without default network or ML enablement.
    - [x] Add fixtures and release limitations for OCR quality.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BJ3 --auto-fix`.

## Phase BJ4 - Accessibility parity

- [x] Task: Implement accessibility reports and UI parity.
    - [x] Add tagged-PDF inspection.
    - [x] Add keyboard and screen-reader smoke coverage.
    - [x] Add PDF/UA oracle integration where available.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BJ4 --auto-fix`.

Track BJ completed after BJ1-BJ4 phase gates passed on 2026-06-24.
