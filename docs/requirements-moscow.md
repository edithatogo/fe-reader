# MoSCoW Requirements Matrix

## Must

- Cross-platform Rust core with no UI/platform coupling.
- Open/read PDF 1.0 through PDF 2.0 where possible.
- Tile-based rendering API and PDFium adapter.
- Searchable text layer with bounding boxes.
- Basic annotations: highlight, note, ink, free text, shapes, stamps.
- Page operations: insert, delete, rotate, reorder, split, merge, extract.
- Metadata read/edit: document info and XMP.
- Metadata scrub/preserve modes.
- Deterministic redaction planning and secure rewrite verification.
- CLI harness for every core operation.
- Platform recent documents and open/save permissions on each OS.
- Per-user and global installation strategy for desktop OSes.
- JSON schemas for workflows, operation intents, patch plans, plugin manifests, metadata ops, conversion jobs.
- Architecture compliance, clippy, tests, schema validation, supply-chain checks.

## Should

- Workflow packs: legal, healthcare, government disclosure, research, publishing, education.
- PDF/A, PDF/UA, PDF/X validation adapters.
- Accessibility inspection: tags, reading order, alt text.
- Forms filling and authoring.
- OCR and searchable PDF generation.
- DOCX/Markdown/HTML/image export.
- Typst/Quarto/LaTeX/Pandoc integration.
- PDF optimisation, linearisation and safe rewrite receipts.
- Long-running jobs with progress, cancellation and power/thermal budgets.
- MCP server with read-only default.
- COM/AppleScript/D-Bus/App Intents/Android intents automation.
- Web/PWA local viewer.
- App integrations: Zotero, Obsidian, LibreOffice, OnlyOffice, VS Code, email/share, WebDAV/Nextcloud.
- Hardware acceleration option for compositing and experimental vector rendering.

## Could

- WASM plugin system.
- Browser extension.
- Windows Search IFilter.
- macOS Spotlight deep links.
- Android AppSearch semantic index.
- iOS share extension and Shortcuts workflows.
- Advanced local NER redaction assist.
- Local embeddings and grounded Q&A.
- C2PA/Content Credentials provenance authoring.
- PDF portfolio/collection authoring.
- Advanced print-production tooling.

## Won't, initially

- Mandatory cloud sync.
- Remote AI dependency.
- Full paragraph reflow editor in early waves.
- RichMedia execution without sandboxing.
- Direct cloud storage integrations before OS document-provider integration.
- Model-driven automatic high-risk redaction without human review.
