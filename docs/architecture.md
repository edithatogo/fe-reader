# Architecture

## High-level structure

```text
Applications / surfaces
  Tauri app | Web/PWA | CLI | MCP server | COM | AppleScript/App Intents | D-Bus | Android intents | iOS share/actions | Plugins

Adapters
  fe_reader_platform_* | fe_reader_integrations_* | fe_reader_uniffi | fe_reader_mcp | fe_reader_plugin_host

Capabilities
  fe_reader_render | fe_reader_metadata | fe_reader_redaction | fe_reader_workflows | fe_reader_conversion | fe_reader_forms

Core
  fe_reader_core | fe_reader_pdf_model
```

## Workspace crates

| Crate | Role | Notes |
|---|---|---|
| `fe_reader_core` | Operation intents, patch plans, risk policy, write modes, audit receipts | Pure Rust, no platform/UI dependencies |
| `fe_reader_pdf_model` | PDF model types, page geometry, boxes, spans, object references | Pure types; preserves unknown features |
| `fe_reader_render` | Rendering traits, tile requests, bitmap abstractions | No PDFium dependency |
| `fe_reader_render_pdfium` | PDFium adapter for production rendering | Isolated FFI boundary |
| `fe_reader_render_gpu` | Optional GPU compositing/vector acceleration experiments | Later-wave crate |
| `fe_reader_metadata` | Document info, XMP, scrub/preserve/provenance modes | Standards-aware |
| `fe_reader_redaction` | Deterministic redaction planning and verification | No ML dependency early |
| `fe_reader_workflows` | Workflow pack engine and templates | Domain-neutral |
| `fe_reader_conversion` | Markdown, HTML, DOCX, Typst/Quarto/Pandoc pipelines | Provider-based |
| `fe_reader_forms` | AcroForm inspection/filling/authoring | Later Stage 3 |
| `fe_reader_cli` | Contract-first CLI | Test harness and automation |
| `fe_reader_uniffi` | Swift/Kotlin/Python/Ruby bindings, C# strategy wrapper | UniFFI boundary |
| `fe_reader_mcp` | Read-only-first MCP server | Late Wave 5 |
| `fe_reader_plugin_host` | WASM plugin host | Late Wave 5 |
| `fe_reader_platform_*` | Per-platform integrations | Native shell concerns only |
| `xtask` | Build, validation, fixture, release tasks | Repository automation |

## Operation model

```text
FeOperationIntent
  -> FePolicyDecision
  -> FePatchPlan
  -> FeReviewRecord
  -> FeApplyResult
  -> FeVerificationReport
  -> FeAuditReceipt
```

## Write modes

```rust
pub enum FeWriteMode {
    IncrementalAppend,
    FullRewritePreserveMetadata,
    FullSanitizingRewrite,
}
```

- Use `IncrementalAppend` for non-destructive stamping and audit-friendly annotations.
- Use `FullSanitizingRewrite` for secure redaction and metadata scrubbing.
- Never claim secure redaction for `MarkupOnly` overlays.

## Automation rule

Every automation surface must produce `FeOperationIntent`. No direct mutation path is allowed from COM, AppleScript, D-Bus, MCP, plugins, web postMessage, Android intents, iOS App Intents, CLI, or Tauri commands.

## Platform rule

OS integrations are implemented in adapters. `fe_reader_core` must not import platform crates or call OS APIs.

## Intelligence rule

Early redaction is deterministic: text spans, geometry, regex, dictionaries, checksum/normalisation, verification. Local NLP, embeddings, and local LLM/RAG are optional later-wave crates only.

## v4 architecture extension: operating-excellence adapters

v4 adds adapter crates that must remain outside `fe_reader_core`:

```text
fe_reader_security        policy engine and dangerous-operation gates
fe_reader_search          deterministic local index providers
fe_reader_text            Unicode/text/font diagnostics
fe_reader_compat          corpus and visual-regression helpers
fe_reader_updates         signed update manifest verification
fe_reader_observability   local diagnostics and support bundles
```

These crates can depend on platform/tooling libraries as needed, but the pure core must remain small, deterministic and fuzzable.
