# Third-party Dependency Strategy

Prefer upstream crates/tools. Fork only when required. All forks must be listed in `fork-policy.yaml`.

Development starting points:

- Use PDFium via `pdfium-render` for production rendering.
- Use `lopdf` and `pdf-writer` for low-level manipulation/writing.
- Evaluate `pdf_oxide` for extraction/Markdown/validation ideas and contribute upstream.
- Use Pandoc/LibreOffice/Typst/Quarto as external conversion/build providers rather than vendoring them.
- Use veraPDF as an external validation adapter.
