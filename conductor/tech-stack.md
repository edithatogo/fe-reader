# Tech Stack

## Core

- Rust 2024 by default with the latest verified stable toolchain; experimental/nightly work stays out of core.
- `lopdf = "0.40.0"`
- `pdf-writer = "0.14.0"`
- `pdf_oxide = "0.3.47"` target/evaluate
- `serde`, `serde_json`, `schemars`, `thiserror`, `anyhow`, `sha2`, `uuid`, `time`

## Rendering

- `pdfium-render = "0.9.1"` in `fe_reader_render_pdfium`
- Optional later: `wgpu = "29.0.3"`, `vello = "0.8.0"`, `skia-safe = "0.93.1"`

## UI

- Tauri v2 primary shell.
- Svelte/TypeScript initial frontend.
- Swift/Kotlin native wrappers where platform affordances require them.

## Bindings

- `uniffi = "0.31.1"`
- C# via third-party bindgen evaluation and/or `extern "C"` ABI fallback with NuGet wrapper.

## Automation

- `rmcp = "1.6.0"` target for MCP server.
- COM, AppleScript/App Intents, D-Bus, Android intents, iOS App Intents.

## Plugins

- `extism = "1.21.0"` target for WASM plugin MVP.
- Direct Wasmtime integration deferred unless Component Model requirements justify it.

## Conversion

- Pandoc provider.
- LibreOffice provider.
- Typst/Quarto/LaTeX source-linked project providers.

## Validation/security

- `cargo deny`, `cargo audit`, `cargo vet`, `cargo fuzz`.
- veraPDF external adapter for PDF/A/PDF/UA validation.


## v5 note

This v5 package adds PDF Engineering Lab, repair/recovery, differential oracles, API compatibility governance, release evidence, prepress/font fidelity, feature flags and ADR/RFC governance.
