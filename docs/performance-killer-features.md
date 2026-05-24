# Performance-Oriented Differentiators

These are not merely optimisation tasks; they are product features that make Fe Reader feel better than typical PDF tools.

## 1. Instant Document Resume

Persist a compact resume index:

```json
{
  "document_sha256": "...",
  "last_page": 42,
  "zoom": 1.35,
  "rotation": 0,
  "visible_bbox": [0, 210, 612, 792],
  "text_index_version": "0.1",
  "tile_cache_hint": [41, 42, 43]
}
```

On reopen, display the last page and search index metadata before all thumbnails are available.

## 2. Progressive PDF Opening

Open in phases:

```text
validate header -> xref/page count -> first visible page -> text layer -> thumbnails -> full metadata/preflight
```

Never block first visual render on full metadata extraction, full thumbnail generation, AI, preflight, or conversion providers.

## 3. Verified Fast Redaction

Fast planning is not enough. The differentiator is verified removal:

```text
candidate detection -> human review -> patch plan -> sanitized rewrite -> text extraction check -> residual object scan -> optional OCR rescan -> receipt
```

## 4. Tile Scheduler with Predictive Prefetch

Use visible range + scroll velocity to prefetch likely tiles. Cancel stale tile jobs when the user scrolls away.

## 5. Page Object Hotspot Inspector

Developer/power-user view showing:

- page render time;
- text extraction time;
- image count and decoded bytes;
- fonts used;
- transparency/layer complexity;
- malformed objects;
- unsupported features.

This is useful for diagnostics and for users trying to understand why a PDF is slow.

## 6. Performance Receipt

For batch operations, optionally produce:

```json
{
  "operation": "secure_redaction",
  "duration_ms": 1482,
  "pages": 88,
  "bytes_in": 12310421,
  "bytes_out": 11999821,
  "peak_memory_mb": 286,
  "verification": "passed"
}
```

## 7. Adaptive Rendering Mode

Automatically choose between:

- CPU PDFium raster rendering;
- lower-resolution preview while scrolling;
- full-resolution render when idle;
- GPU overlay compositing for annotations/selection;
- accessibility-friendly high-contrast rendering.

## 8. Benchmark Corpus as Community Asset

Publish a non-sensitive open PDF corpus manifest with categories:

```text
small text
large legal bundle
scientific paper
scanned OCR
engineering drawing
tagged accessible PDF
layers/OCG
transparency-heavy
forms
encrypted
malformed/adversarial
```

This helps contributors reproduce regressions and compare renderers fairly.
