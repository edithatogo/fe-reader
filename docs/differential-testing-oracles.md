# Differential Testing and External Oracles

## Purpose

Fe Reader should avoid isolated correctness claims. For parsing, rendering, extraction, metadata, repair, redaction and conversion, compare behaviour against established tools and engines where possible.

Differential testing does not mean another engine is always correct. It means disagreements are captured, classified and turned into fixtures.

## Oracle classes

| Oracle | Use | Notes |
|---|---|---|
| qpdf | Structural checks, object rewriting, xref behaviour, encryption details, QDF inspection. | Excellent for syntax and structural sanity checks. |
| veraPDF | PDF/A and PDF/UA validation. | Use for standards validation, not general rendering. |
| PDFium | Rendering reference and form behaviour. | Also Fe Reader production rendering adapter initially. |
| Poppler | Rendering/text extraction comparison. | Useful Linux/FOSS baseline. |
| MuPDF | Rendering/text extraction comparison and malformed PDF handling. | Useful high-performance C baseline. |
| Ghostscript | Print/prepress/postscript-style conversion checks. | Useful for print workflows and rasterisation diffs. |
| Pandoc | Markdown/HTML/DOCX conversion reference. | Use for source-conversion workflows, not arbitrary PDF text reconstruction. |
| LibreOffice | Office conversion and import/export reference. | Use provider-style and sandboxed. |

## Test classes

```text
syntax_validity
render_visual_similarity
text_extraction_similarity
metadata_roundtrip
redaction_leak_absence
repair_semantic_delta
conversion_output_quality
accessibility_validation
prepress_validation
```

## Differential pipeline

```text
fixture manifest
  -> run Fe Reader operation
  -> run oracle operation(s)
  -> normalise outputs
  -> compare metrics
  -> classify disagreement
  -> store report
```

## Disagreement classes

```text
fe_reader_bug
oracle_bug_or_limitation
spec_ambiguous
fixture_invalid
known_pdf_feature_gap
acceptable_visual_delta
security_policy_difference
```

## Phase gates

- Wave 0: define report schema and smoke script.
- Wave 1: compare rendering of at least one simple fixture.
- Wave 2: compare text extraction and metadata snapshots.
- Wave 3: compare redaction and repair outputs.
- Wave 4: run standards/preflight and conversion oracles.
- Wave 7: release cannot claim support for a document class unless it has accepted corpus coverage or a documented limitation.

See `contracts/rust/differential_oracle.rs` and `schemas/differential-test-report.schema.json`.
