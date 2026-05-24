# Source Pipeline and Notebook Mode

## Purpose

PDFs are often final artifacts, but many professional workflows need traceability back to source documents. Fe Reader should eventually support a `Fe Project` workspace where source, transformations and output PDFs are tracked together.

## Supported source ecosystems

```text
Markdown
Pandoc
Quarto
Typst
LaTeX
HTML
DOCX/ODT via conversion providers
Images/scans
```

## Early implementation

Do not build a full authoring suite first. Start with inspection and reproducible export:

```text
source file -> conversion provider -> PDF -> Fe workflow operations -> verified output
```

## Notebook mode concept

A notebook/workspace records:

```text
source files
conversion command
converter version
input hashes
generated PDF hash
workflow operations
output PDF hash
receipts
```

This creates a reproducible document build trail without requiring cloud sync.

## Killer feature

Fe Reader can answer: "What source, commands, workflow operations and metadata produced this PDF?" That is not normal PDF-reader behaviour and would be highly valuable for research, publishing, policy, legal, clinical and regulatory workflows.
