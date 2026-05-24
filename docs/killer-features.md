# Differentiating / Killer Features

## 1. Reviewable Patch Plans

Before any destructive operation, Fe Reader shows a structured patch plan:

```text
Delete 2 pages
Redact 18 text spans
Remove 4 metadata fields
Rewrite 3 image streams
Preserve 1 digital signature warning as invalidated-by-edit
```

Users and agents can review before applying.

## 2. Receipted Redaction

Secure redaction emits a receipt proving what was done and what verification passed:

- extraction absence check;
- object-stream scan;
- metadata scrub result;
- optional OCR rescan;
- before/after document hash;
- workflow pack and template version.

## 3. Metadata Diff/Scrub/Preserve Modes

Most PDF tools expose metadata weakly. Fe Reader should make metadata first-class:

- `clean-share` mode;
- `forensic-preserve` mode;
- XMP extension schema editor;
- provenance manifest preview;
- embedded file/portfolio metadata.

## 4. Workflow Packs

Declarative packs for healthcare, government disclosure, research, legal, publishing, education, and engineering.

## 5. Page/Span Deep Links

Deep links to page, bounding box, annotation, redaction candidate, and workflow receipt:

```text
fe-reader://open?doc=<hash>&page=12&bbox=72,100,300,140
```

## 6. Local Automation Without Cloud Lock-in

CLI, MCP, COM, AppleScript, D-Bus, Android intents, iOS App Intents, browser extension, and plugins share one operation contract.

## 7. Conversion with Source Toolchains

Fe Reader can treat Quarto, Typst, LaTeX, Markdown, and Pandoc projects as source-linked PDF projects, not just final PDFs.

## 8. Standards-Aware Output

Fe Reader should know when a PDF is PDF/A, PDF/UA, PDF/X, PDF 2.0, encrypted, signed, tagged, layered, a portfolio, or contains rich media.

## 9. Controlled RichMedia Handling

Recognise RichMedia but disable execution by default. Provide a policy-aware viewer that explains what is embedded.

## 10. Later: Local Intelligence with Evidence

When local intelligence is added, every answer/suggestion must cite page and bounding-box evidence. No model output may directly mutate a document.
