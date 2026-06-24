# Track BI: PDF Corpus and Oracle Factory Plan

## Phase BI1 - Corpus manifest expansion

- [x] Task: Expand fixture manifest.
    - [x] Add fixture classes for every parity registry family.
    - [x] Add provenance, license, privacy and oracle expectation fields.
    - [x] Add placeholder entries only where the limitation text is explicit.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BI1 --auto-fix`.

## Phase BI2 - Fixture generation

- [x] Task: Generate redistributable fixtures.
    - [x] Generate baseline fixtures for PDF versions, text, images, fonts, annotations, forms, signatures, portfolios, layers and standards profiles.
    - [x] Generate malformed/adversarial fixtures under safe-open policy.
    - [x] Generate visual and text goldens.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BI2 --auto-fix`.

## Phase BI3 - Oracle runner expansion

- [x] Task: Add oracle capability discovery and execution.
    - [x] Discover qpdf, veraPDF, PDFium, Poppler, MuPDF, Ghostscript, OCR and conversion tools.
    - [x] Emit structured pass/fail/skip reports.
    - [x] Bind oracle outputs to parity claim IDs.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BI3 --auto-fix`.

## Phase BI4 - Release evidence integration

- [x] Task: Integrate corpus/oracle reports into launch gates.
    - [x] Generate release compatibility by parity family.
    - [x] Attach reports to release evidence bundles.
    - [x] Fail stable marketing if stable claims lack fixture and oracle evidence.
- [x] Task: Conductor phase gate.
    - [x] Run `scripts/conductor_phase_gate.sh --phase BI4 --auto-fix`.

Track BI completed after BI1-BI4 phase gates passed on 2026-06-24.
