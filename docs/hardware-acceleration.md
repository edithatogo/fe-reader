# Hardware Acceleration Plan

## Goals

- Smooth zoom/pan/scroll on large PDFs.
- Fast annotation overlay compositing.
- Efficient thumbnails and multi-page view.
- Optional future GPU vector rendering.

## Stages

### Stage 1 — CPU rendering + GPU compositing

- Render tiles through PDFium adapter.
- Cache tiles in memory/disk.
- UI composites tiles using native/webview GPU acceleration where available.
- Text layer and annotation overlays are separate layers.

### Stage 2 — Native GPU compositor crate

- Add `fe_reader_render_gpu` using `wgpu`.
- Use GPU for tile compositing, overlays, colour transforms, and selection/redaction previews.

### Stage 3 — Vector acceleration experiments

- Evaluate Vello for vector/text-heavy overlay rendering.
- Evaluate Skia-backed pipelines where PDFium/Skia integration provides clear benefits.
- Do not replace PDFium for core PDF rendering until conformance and performance are proven.

## Contract

See `contracts/rust/render_backend.rs`.

## Safety

- Hardware acceleration is optional.
- Rendering bugs must not affect document mutation.
- Rendered pixels must never be used as the only proof of redaction; verification must inspect saved content.
