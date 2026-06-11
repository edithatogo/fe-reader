# Next implementation slices after v8

## Slice 1: Parser inspect adapter

Use `lopdf` in a parser adapter crate or `fe_reader_pdf_model` feature lane to inspect page count, encryption status, trailer info and PDF version. Keep `fe_reader_core` pure.

## Slice 2: Transaction journal persistence

Implemented baseline: JSON sidecar journaling persists patch plans, `journal inspect` reads persisted sidecars, `journal recoveries` lists sidecars requiring crash-recovery inspection, and temporary-directory tests cover incomplete apply recovery detection.

## Slice 3: Metadata read-only inspection

Read document info and XMP metadata. Do not mutate bytes yet.

## Slice 4: Render adapter contract test

Implemented in Track B Wave 0 scaffolding. `NullRenderBackend` remains deterministic, `fe_reader_render_pdfium` exposes an unavailable-runtime adapter boundary, and GPU/hardware acceleration remain policy-only until runtime discovery is governed.

## Slice 5: Policy matrix tests

Add table-driven tests for CLI, UI, MCP, COM, AppleScript, D-Bus, Android intents, iOS App Intents, browser extension, plugins and local API.
