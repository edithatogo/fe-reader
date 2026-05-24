# Industry Baseline Capability Matrix

This matrix tracks broad industry baseline parity without framing Fe Reader as a copy of any specific product.

| Capability family | Wave | Contract owner | Notes |
|---|---:|---|---|
| Reader/navigation/search | 1 | Render/Core/UI | Must be fast and stable before Pro features |
| Comments/annotations | 2 | Workflows/UI/Core | Preserve editable annotation objects where possible |
| Page organisation | 2 | Core/CLI/UI | Core CLI first, UI second |
| Metadata/XMP/scrub | 2 | Metadata | Fe should exceed baseline with diff and receipts |
| Secure redaction | 3 | Redaction/Core | Sanitising rewrite + verification required |
| OCR/searchable PDF | 3-4 | Conversion/Redaction | Optional engine adapters |
| Forms | 3 | Forms/Core/UI | AcroForms first, XFA recognise/warn only |
| Accessibility | 3-4 | Metadata/Preflight | Inspect first, repair later |
| Preflight/standards | 3-4 | Metadata/Preflight | PDF/A, PDF/UA, PDF/X adapters |
| Conversion | 4 | Conversion | Markdown/DOCX/HTML/image, source toolchain integration |
| Distribution | 4 | Packaging | Registry publishing and codesigning |
| Automation | 5 | Integrations/Core | Safe read-only-first mutation model |
| Plugins | 5 | Plugin host | Proposal-only at first |
| Local intelligence | 6 | Intelligence | Optional, local, never required |
