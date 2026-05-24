# Next implementation slices after v8

## Slice 1: Parser inspect adapter

Use `lopdf` in a parser adapter crate or `fe_reader_pdf_model` feature lane to inspect page count, encryption status, trailer info and PDF version. Keep `fe_reader_core` pure.

## Slice 2: Transaction journal persistence

Add JSON sidecar journaling for patch plans. Include crash recovery tests using temporary directories.

## Slice 3: Metadata read-only inspection

Read document info and XMP metadata. Do not mutate bytes yet.

## Slice 4: Render adapter contract test

Keep `NullRenderBackend`; add a PDFium adapter behind `fe_reader_render_pdfium`. Ensure rendering tests can be skipped when PDFium is unavailable.

## Slice 5: Policy matrix tests

Add table-driven tests for CLI, UI, MCP, COM, AppleScript, D-Bus, Android intents, iOS App Intents, browser extension, plugins and local API.
