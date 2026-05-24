# Product Strategy

## Product statement

Fe Reader is a local-first, cross-platform PDF workflow platform. It combines a high-performance Rust core, native platform integration, structured workflow packs, metadata control, conversion pipelines, secure redaction, and automation surfaces.

The product must avoid being domain-specific. Legal workflows are important but are one workflow family among others.

## Workflow families

| Workflow family | Examples | Priority |
|---|---|---|
| Core reading | fast rendering, search, outline, thumbnails, recent documents | Must |
| Annotation | highlight, free text, ink, shapes, stamps, comments | Must |
| Page operations | merge, split, delete, rotate, extract, crop, reorder | Must |
| Metadata | document info, XMP, scrub/preserve modes, provenance | Must |
| Legal | affidavits, initials, witness blocks, Bates numbering, exhibit packs | Should |
| Healthcare | deterministic de-identification, audit receipt, metadata scrubbing | Should |
| Government disclosure | exemption-coded redaction, review queue, disclosure bundles | Should |
| Research | highlights to Markdown/Zotero, paper metadata, PDF-to-Markdown | Should |
| Publishing/print | PDF/A, PDF/X, output intents, trim/bleed/crop checks | Should |
| Accessibility | tags, reading order, alt text, PDF/UA reports | Should |
| Engineering/construction | layers, measurements, revision clouds, approval stamps | Could |
| Education | grading stamps, worksheet annotation, classroom packs | Could |
| Local intelligence | optional local PII assist, embeddings, grounded Q&A | Later |

## Differentiators

- **Local-first by default**: no mandatory cloud services.
- **Workflow packs**: structured declarative workflows rather than one-off tools.
- **Safe automation**: CLI, MCP, COM, AppleScript, D-Bus, Android intents, iOS App Intents, and plugins use the same operation model.
- **Verified document changes**: high-risk changes are planned, reviewed, applied, verified, and receipted.
- **Metadata transparency**: diff, scrub, preserve, and provenance modes.
- **Cross-platform native behaviour**: recent files, permissions, notifications, search indexing, file association, and printing per OS.

## Non-goals for early waves

- Cloud sync as a built-in product.
- ML-first redaction.
- Local LLM/RAG-first UX.
- Implementing a renderer from scratch before proving product value.
- Full paragraph reflow editing before object-level editing, metadata, redaction, forms, and conversion foundations are stable.
