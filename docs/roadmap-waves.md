# Roadmap Waves and Themes

## Themes

| Theme | Tracks | Purpose |
|---|---|---|
| Foundation | A, K, O, P, N | Core contracts, CLI, fixtures, threat model, corpus, testing, security gates, performance budgets |
| Reader & Native Shell | B, C, D, E, Q | Rendering, UI, bindings, OS integration, deterministic search, text/i18n |
| Editing & Workflows | F, G | Page ops, annotations, metadata, workflow packs, secure redaction |
| Conversion & Publishing | H, J, R | DOCX/Markdown/HTML/image conversion, installers, registries, signed updates |
| Integrations & Automation | I, L, S | App integrations, web/PWA/browser extension, automation APIs, SDK/devkit |
| Security & Operations | K, O, R | threat model, policy, sandboxing, SBOM, provenance, enterprise deployment |
| Quality & Performance | N, P, Q | Benchmarking, profiling, visual regression, corpus coverage, text/search quality |
| Frontier | M | Optional local intelligence and advanced GPU/vector paths |

## Waves

### Wave 0 — Foundation contracts, CLI harness, security policy and corpus baseline

- Workspace skeleton.
- Pure core contracts.
- CLI skeleton.
- Fixture layout and corpus manifest.
- JSON schemas.
- Architecture compliance script.
- Threat model and policy engine contract.
- Cargo deny/audit/vet/fuzz smoke setup.
- Performance budget registry and smoke benchmark harness.

### Wave 1 — Reader fundamentals, native shell basics, deterministic search and text baseline

- PDF open/render using PDFium adapter.
- First-page, tile-render, text-extraction, and startup performance scenarios.
- Tile-based rendering contracts.
- Text extraction and span geometry.
- In-memory deterministic search over spans.
- Recent documents on each OS.
- File association/open-with plan.
- Basic Tauri shell.

### Wave 2 — Annotation, metadata, page operations, i18n and visual regression

- Highlights, notes, drawing, shapes, stamps.
- Metadata view/edit/diff/scrub/preserve.
- Merge/split/delete/reorder/crop/rotate.
- Attachments and embedded files inspection.
- Visual regression runner.
- CJK, RTL, vertical text and font diagnostics fixtures.

### Wave 3 — Workflow packs and verified high-risk operations

- Workflow engine.
- Legal/affidavit, healthcare de-identification, government disclosure, research, publishing templates.
- Deterministic redaction with verification.
- Audit receipts.
- Forms inspection/filling.
- Automation policy enforcement for high-risk operations.
- Workflow performance benchmarks for bulk document operations.

### Wave 4 — Conversion, publishing/distribution and signed update readiness

- Markdown, text, image, HTML export.
- DOCX pipeline through Pandoc/LibreOffice providers.
- Typst/Quarto/LaTeX/Pandoc project integration.
- Installers, per-user/global install modes, registry publishing.
- Signed update manifest contract.
- SBOM/provenance release scaffolding.

### Wave 5 — Application integrations, safe automation, enterprise policy and SDKs

- MCP read-only-first server.
- Windows COM automation.
- macOS AppleScript/App Intents.
- Linux D-Bus.
- Android intents/DocumentsProvider.
- iOS App Intents/share/document actions.
- Browser extension and web/PWA.
- WASM plugin host.
- Enterprise policy loading and precedence.
- SDK/devkit examples.

### Wave 6 — Frontier optional intelligence and advanced acceleration

- Local NER-assisted redaction.
- Local embeddings/search over extracted spans.
- Grounded document Q&A with page/bounding-box citations.
- Advanced GPU compositing/vector acceleration.
- Provenance workflows and C2PA manifest authoring.
- PGO/BOLT release optimisation, optional GPU acceleration defaults only if benchmark evidence supports them.

### Wave 7 — Release hardening and operating excellence

- Stable-channel installer and update manifest validation.
- SBOM/provenance generation.
- Compatibility corpus report.
- Visual regression report.
- Performance budget report.
- Security/dependency report.
- Accessibility report.
- Enterprise deployment policy templates.
- Support-bundle/diagnostics review.

## Deferral of ML/RAG

Wave 0-5 must be valuable without ML, local LLMs, or RAG. Contracts may reserve extension points, but no early milestone may depend on model inference.
