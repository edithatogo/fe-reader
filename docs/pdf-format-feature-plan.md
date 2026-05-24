# PDF Format Feature Plan

## Read/recognise targets

| Feature | Early handling | Later handling |
|---|---|---|
| PDF 1.0-1.7 | Read, preserve, manipulate core objects | Full coverage tests |
| PDF 2.0 | Recognise and preserve unknowns | Feature-specific authoring where useful |
| Encrypted PDFs | Open where supported | Modern encryption write support |
| Digital signatures | Inspect and warn on invalidation | Standards-aware signing later |
| Tagged PDFs | Inspect structure tree | Accessibility repair/authoring |
| Optional Content Groups | View layer list | Toggle/export layer states |
| Portfolios/collections | Inspect and extract | Author simple portfolios |
| Embedded files | Inspect/extract/add | Associated-file workflows |
| RichMedia | Recognise and sandbox/disable | Policy-gated preview only |
| Output intents | Inspect | Author/repair for print workflows |
| Measurement data | Inspect | Engineering measurement tools |
| Forms | AcroForm fill/read | Author forms; XFA recognise/warn |
| JavaScript actions | Detect/disable by default | Policy-gated inspection only |

## Write targets

- Prefer PDF 1.7-compatible output unless a feature requires PDF 2.0.
- Preserve unknown objects where safe.
- Warn on downgrade or feature loss.
- Store Fe receipts in XMP extension schemas and/or associated files, not arbitrary undocumented blobs.
