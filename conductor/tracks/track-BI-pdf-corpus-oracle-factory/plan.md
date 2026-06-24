# Track BI: PDF Corpus and Oracle Factory Plan

## Phase BI1 - Corpus manifest expansion

- [ ] Task: Expand fixture manifest.
    - [ ] Add fixture classes for every parity registry family.
    - [ ] Add provenance, license, privacy and oracle expectation fields.
    - [ ] Add placeholder entries only where the limitation text is explicit.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BI1 --auto-fix`.

## Phase BI2 - Fixture generation

- [ ] Task: Generate redistributable fixtures.
    - [ ] Generate baseline fixtures for PDF versions, text, images, fonts, annotations, forms, signatures, portfolios, layers and standards profiles.
    - [ ] Generate malformed/adversarial fixtures under safe-open policy.
    - [ ] Generate visual and text goldens.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BI2 --auto-fix`.

## Phase BI3 - Oracle runner expansion

- [ ] Task: Add oracle capability discovery and execution.
    - [ ] Discover qpdf, veraPDF, PDFium, Poppler, MuPDF, Ghostscript, OCR and conversion tools.
    - [ ] Emit structured pass/fail/skip reports.
    - [ ] Bind oracle outputs to parity claim IDs.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BI3 --auto-fix`.

## Phase BI4 - Release evidence integration

- [ ] Task: Integrate corpus/oracle reports into launch gates.
    - [ ] Generate release compatibility by parity family.
    - [ ] Attach reports to release evidence bundles.
    - [ ] Fail stable marketing if stable claims lack fixture and oracle evidence.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BI4 --auto-fix`.
