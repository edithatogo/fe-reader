# Compatibility Corpus Governance

## Purpose

Feature claims are accepted only when they pass representative documents. The corpus is both a test suite and a product-quality asset.

## Corpus classes

```text
fixtures/corpus/
  basic/
  encrypted/
  signed/
  forms-acroform/
  forms-xfa/
  annotations/
  attachments-portfolios/
  layers-ocg/
  tagged-accessibility/
  rtl-cjk-complex-text/
  scanned-ocr/
  redaction/
  engineering-measurement/
  publishing-pdfa-pdfx-pdfua/
  malformed-adversarial/
  huge-files/
  incremental-updates/
```

## Fixture manifest

Each fixture must have:

- source and license
- allowed redistribution state
- PDF version and standard profile
- expected features
- expected unsupported features
- expected render hash or visual tolerance
- expected text spans where applicable
- risk tags

## Acceptance policy

- A fixture may be private, public, synthetic, or generated.
- Public fixtures must have a clear redistribution license.
- Private fixtures may be used in local development but cannot be committed.
- Synthetic fixtures are preferred for reproducible edge cases.
- Every bug fix involving a PDF parser/render/conversion issue should add a fixture or generator.

## Differential testing

Where possible, compare Fe Reader with:

- PDFium rendering
- Poppler rendering
- MuPDF rendering
- veraPDF validation for PDF/A and PDF/UA
- Pandoc/LibreOffice conversion output for conversion workflows

Differences are classified as:

```text
accepted_difference
fe_bug
reference_bug
undefined_or_implementation_defined
needs_human_review
```
