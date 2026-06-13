# Source-Linked Authoring Workflows

Fe Reader should not only edit existing PDFs. It should understand that many PDFs are generated from source documents. The goal is to make source-linked PDF workflows first-class without forcing every converter into `fe_reader_core`.

## Supported provider families

| Provider | Role | Integration mode |
|---|---|---|
| Pandoc | Markdown, HTML, DOCX, LaTeX and many document format conversions | External CLI/provider first; optional service later. |
| Quarto | Scientific/technical publishing projects, Markdown notebooks, Typst/PDF outputs | Project detector and external CLI provider. |
| Typst | Modern source-to-PDF authoring, fast PDF builds | External CLI provider first; source map support later. |
| Tectonic | Self-contained TeX/LaTeX engine | External CLI/provider; useful for LaTeX workflows. |
| LibreOffice/OnlyOffice | DOCX/ODT conversion | External provider/oracle; not inside core. |

## Source-linked project model

A source-linked project is not just a PDF. It is:

```text
source files
build configuration
generated PDF
source map / page map
bibliography/assets
operation receipts
export targets
```

The project contract should support:

- A `source_map` sidecar field that can point to the generated source-map artifact.

- Open generated PDF and reveal source file/line for a selected page or object when available.
- Rebuild PDF after source edits.
- Export highlights/comments back to Markdown sidecars.
- Preserve citation keys and bibliography context.
- Compare PDF output between builds.
- Attach build receipts to the workspace, not necessarily to the PDF.

## Source map strategy

Initial implementation can be coarse:

```text
page -> source file -> heading/section -> approximate line range
```

Later implementation can support:

```text
PDF object/text span -> source range -> build provider provenance
```

## CLI examples

```bash
fe-reader project init --kind typst --source main.typ --pdf main.pdf
fe-reader project build ./paper.feproj
fe-reader project open-output ./paper.feproj
fe-reader project locate-source ./paper.feproj --page 8 --bbox 10,20,200,80
fe-reader export-highlights paper.pdf --to markdown --source-linked ./paper.feproj
```

## Safety rules

- Source builds must not run automatically without explicit user approval.
- Build providers are external tools with sandbox/permission policies.
- No project file may execute arbitrary commands unless policy explicitly permits it.
- Source-linked workflows must work offline.
- Generated PDFs remain ordinary PDFs; project metadata is sidecar-first unless the user chooses to embed metadata.

## Milestones

| Wave | Work |
|---|---|
| 2 | Conversion provider trait and project schema. |
| 4 | Pandoc/Typst/Quarto/Tectonic external provider stubs and smoke tests. |
| 5 | Source-linked workspace UI and CLI. |
| 6 | Fine-grained source-map research and optional semantic helpers. |
