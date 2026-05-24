# Text, Fonts, Unicode and Internationalisation Plan

## Why this matters

PDF text is difficult: glyph order may not match reading order, text may be shaped, fonts may be subsetted, encodings may be custom, and pages may contain CJK, RTL, vertical writing, ligatures or missing ToUnicode maps.

## Requirements

- Preserve text spans with bounding boxes.
- Preserve original glyph and decoded Unicode where possible.
- Support page text direction, writing mode and rotation.
- Support CJK, RTL, combining marks, ligatures and emoji in UI text overlays.
- Provide accessible reading-order inspection, not just raw extraction.
- Never assume extracted text can be reinserted without layout analysis.

## Libraries

- `cosmic-text` for advanced text layout in Rust-side UI/overlay experiments.
- `rustybuzz` for HarfBuzz-compatible shaping when direct shaping is needed.
- `fontdb`/system font discovery through the rendering/text stack.
- PDFium remains the production rendering reference initially.

## Test fixtures

```text
fixtures/corpus/rtl-cjk-complex-text/
  arabic-rtl.pdf
  hebrew-rtl.pdf
  japanese-vertical.pdf
  chinese-cjk.pdf
  devanagari-combining.pdf
  ligatures.pdf
  emoji.pdf
  missing-tounicode.pdf
```

## User-facing features

- Text extraction confidence indicators.
- Reading-order view.
- Font substitution diagnostics.
- Missing glyph warnings.
- Accessibility text report.
