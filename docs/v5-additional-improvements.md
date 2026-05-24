# v5 Additional Improvements Summary

v5 adds the layer that turns Fe Reader from a large feature plan into an inspectable, testable and governable PDF platform.

## Added themes

1. PDF Engineering Lab and object-level diagnostics.
2. Repair/recovery/safe-open mode for malformed and hostile PDFs.
3. Differential testing against external oracles.
4. API/ABI compatibility governance across every public surface.
5. Reproducible build evidence, attestations and release bundles.
6. Advanced colour, print/prepress and font fidelity planning.
7. Feature-flag/runtime-capability governance.
8. Maintainer/RFC/ADR process.
9. Zero-copy and memory-map policy under resource limits.

## Additional “killer” PDF ideas

| Feature | Why it matters |
|---|---|
| PDF X-Ray | Object tree, content stream, glyph map and revision timeline in one UI. |
| Verified Repair | Repair damaged PDFs with a transparent receipt, not a silent rewrite. |
| Redaction Leak Proof | Scan old revisions, object streams, extracted text and optional OCR after redaction. |
| Source Map Conversion | Markdown/DOCX/Typst/Quarto conversion with links back to page/bounding box. |
| Standards Explorer | Explain PDF/A, PDF/UA, PDF/X and PDF 2.0 issues in plain language with object references. |
| Feature Capability Matrix | Per-platform and per-install feature availability with policy reasons. |
| Performance Receipts | Show render/search/redaction/conversion timings and budgets for heavy workflows. |
| Corpus-Backed Claims | Public support claims tied to fixture classes and oracle reports. |

## Not early priorities

Local LLMs, RAG, cloud sync, collaborative cloud review rooms and autonomous high-risk edits remain deferred. Contracts exist so they can be added later without redesigning the core.
