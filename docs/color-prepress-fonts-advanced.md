# Advanced Colour, Prepress and Font Fidelity

## Purpose

Fe Reader should eventually support print, publishing and engineering workflows that typical readers treat superficially. This is not an MVP requirement, but the core model should avoid design decisions that make it impossible later.

## Feature areas

| Area | Capabilities |
|---|---|
| Colour spaces | DeviceGray/RGB/CMYK, ICCBased, Indexed, Separation, DeviceN, Lab where encountered. |
| Output intents | Document-level and page-level output intents, ICC profile inspection, PDF/X relevance. |
| Overprint | Detect and preview overprint/knockout differences where rendering backends support it. |
| Transparency | Transparency groups, blend modes, soft masks, isolated/knockout groups. |
| Separations | Spot colour names, plates, tint transforms, DeviceN alternates. |
| Boxes | MediaBox, CropBox, BleedBox, TrimBox, ArtBox, page labels and imposition diagnostics. |
| Fonts | Embedded/subset fonts, missing fonts, ToUnicode maps, CID fonts, glyph ids, fallback risk. |
| Font subsetting | Later: generate correct subsets for inserted text/signatures/stamps. |

## Implementation stance

- Do not attempt full prepress parity in early waves.
- Expose inspection and warning first.
- Use external validators and oracles for PDF/X and print workflows.
- Preserve unknown colour/font structures where safe.
- Never silently convert CMYK/spot-colour PDFs into RGB-only outputs in workflow operations that claim preservation.

## Possible libraries/tools

```text
pdfium-render      initial rendering adapter
lcms2-sys/lcms2    future colour-management adapter if needed
fontdb             font discovery/fallback support
rustybuzz          shaping/glyph diagnostics
cosmic-text        text layout support for UI/diagnostics
veraPDF            standards validation external oracle
Ghostscript        print/rasterisation oracle
```

## Output

See `contracts/rust/color_prepress.rs` and `schemas/color-prepress-report.schema.json`.
