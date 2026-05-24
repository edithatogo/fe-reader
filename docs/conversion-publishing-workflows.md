# Conversion and Source-Linked Publishing Workflows

## Conversion philosophy

Fe Reader should not only convert PDFs after the fact. It should also recognise source-linked document projects: Markdown, Quarto, Typst, LaTeX, and Pandoc-driven projects.

## Export targets

| Target | Early provider | Later provider |
|---|---|---|
| Text | pdf_oxide / internal extraction | layout-aware structured text |
| Markdown | pdf_oxide / internal blocks | table/formula-aware Markdown |
| HTML | internal export | accessible semantic HTML |
| Images | render backend | tiled batch export |
| DOCX | Pandoc/LibreOffice provider | native structured exporter later |
| ODT | LibreOffice provider | optional native exporter |
| CSV | table extraction provider | layout/model-aware extraction |
| JSON | spans, geometry, metadata, annotations | stable machine-readable schema |
| Typst | extract-to-source approximation | source-linked project import/export |
| LaTeX | extract-to-source approximation | source-linked project import/export |
| Quarto | `.qmd` integration | source map and citation workflows |

## Import targets

- PDF.
- Markdown / Quarto / Typst / LaTeX projects for compilation to PDF.
- DOCX/ODT through conversion providers.
- Images for scanned PDF creation.

## Source-linked project model

```text
Source file(s) -> Build provider -> PDF -> SourceMap -> Fe annotations/workflows
```

This enables:

- click PDF text -> jump to source line where possible;
- export highlights back to Markdown/Quarto notes;
- regenerate PDF after edits;
- preserve citation keys and bibliography metadata.

## Provider contract

See `contracts/rust/conversion.rs` and `schemas/conversion-job.schema.json`.
