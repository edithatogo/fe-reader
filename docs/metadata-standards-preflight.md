# Metadata, Standards, and Preflight

## Metadata scope

Fe Reader should support:

- document info dictionary;
- XMP packet read/write;
- XMP extension schemas;
- metadata diff;
- metadata scrub profile;
- forensic preserve profile;
- embedded file metadata;
- annotation metadata;
- form field metadata;
- optional provenance manifests.

## Modes

| Mode | Behaviour |
|---|---|
| `view` | Show document info, XMP, embedded files, output intents, signatures, encryption, conformance flags |
| `edit` | Edit safe user-visible metadata |
| `diff` | Compare before/after metadata and hidden metadata |
| `clean_share` | Remove authorship/app paths/private custom fields/thumbnails as configured |
| `forensic_preserve` | Preserve metadata and emit warnings rather than scrubbing |
| `provenance_preview` | Show C2PA/Content Credentials if present |
| `provenance_author` | Later: create local provenance manifests |

## Standards/preflight

- PDF syntax validation.
- PDF 2.0 feature recognition.
- PDF/A validation via external adapter.
- PDF/UA accessibility validation via external adapter.
- PDF/X print validation via external adapter.
- Tagged PDF and reading order inspection.
- Output intents and colour profile inspection.
- Encryption and signature inspection.

## PDF/A, PDF/UA, PDF/X approach

Fe Reader should not pretend to implement all validation internally at first. Use adapters:

- `verapdf` CLI/JAR adapter for PDF/A/PDF/UA.
- Future call-outs to other validators for PDF/X/print production.
- Internal checks for common errors and document model consistency.

## Hidden/underused feature support

- Optional Content Groups/layers.
- Page labels and named destinations.
- Structure tree/tagged PDF.
- Article threads.
- Attachments/associated files.
- Portfolios/collections.
- OutputIntents and page-level output intents.
- Measurement dictionaries and point data.
- RichMedia recognition, disabled execution by default.
- Embedded-file thumbnails.
- Requirements dictionaries and viewer preferences.
- XMP extension schemas.
- PieceInfo/private app data inspection.
