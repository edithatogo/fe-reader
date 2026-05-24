# Testing and Fixtures Plan

## Fixture classes

```text
fixtures/
  corpus/
  malicious/
  signed/
  encrypted/
  redaction/
  metadata/
  forms/
  tagged/
  pdfa/
  pdfua/
  pdfx/
  rotated-cropped/
  scanned/
  cjk-rtl/
  layers/
  portfolios/
  richmedia/
  conversion/
  workflows/
```

## Golden tests

- Render checksum/perceptual diff with tolerance.
- Extracted text with bounding boxes.
- Metadata before/after diff.
- Redaction verification.
- Workflow plan snapshots.
- CLI JSON output stability.
- Platform contract smoke tests.
- Conversion output sanity checks.

## Invariant tests

- Unknown PDF objects preserved where safe.
- Secure redaction uses `FullSanitizingRewrite`.
- Incremental append is not used for secure redaction.
- Automation mutation requires approval token.
- Core crate has no forbidden dependencies.
