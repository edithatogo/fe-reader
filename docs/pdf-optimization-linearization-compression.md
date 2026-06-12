# PDF Optimisation, Linearisation and Compression

Many PDFs are slow, bloated or unnecessarily difficult to stream. Fe Reader should provide safe optimisation tools with measurable before/after reports.

## Optimisation goals

- Reduce file size without changing visible output.
- Improve first-page open time through linearisation where appropriate.
- Deduplicate repeated images, fonts and XObjects where safe.
- Recompress images according to policy.
- Subset or consolidate fonts where safe.
- Remove unused/unreachable objects during sanitising rewrites.
- Preserve signatures unless the user explicitly chooses an operation that invalidates them.
- Emit an optimisation receipt.

## Optimisation levels

| Level | Name | Behaviour |
|---|---|---|
| 0 | Inspect only | Report optimisation opportunities. |
| 1 | Safe rewrite | Remove unreachable objects, normalise xref, preserve visible output. |
| 2 | Size optimise | Deduplicate streams, recompress images conservatively, subset fonts when safe. |
| 3 | Aggressive | May alter image quality or advanced structures; requires explicit approval. |
| 4 | Web delivery | Linearise and optimise for streaming/first-page display. |

## External oracles

Use external tools as test oracles and development references, not as mandatory runtime dependencies:

- qpdf for linearisation, encryption/decryption and PDF inspection or transformation.
- Ghostscript for comparison of rendered output and some compression/reference behaviours.
- veraPDF for PDF/A and PDF/UA validation.
- PDFium/Poppler/MuPDF for rendering comparisons.

## Output receipt

```json
{
  "operation": "pdf_optimization",
  "input_sha256": "...",
  "output_sha256": "...",
  "level": "safe_rewrite",
  "bytes_before": 10485760,
  "bytes_after": 7340032,
  "linearized": true,
  "visible_regression_max_delta": 0.0,
  "signatures_preserved": true,
  "objects_removed": 241,
  "streams_deduplicated": 12,
  "warnings": []
}
```

## CLI examples

```bash
fe-reader optimize input.pdf --inspect
fe-reader optimize input.pdf --level safe-rewrite --out output.pdf
fe-reader optimize input.pdf --linearize --out output.linearized.pdf
fe-reader optimize input.pdf --level aggressive --image-quality 85 --out output.small.pdf
```

## Evidence

- `scripts/optimization_oracle_smoke.sh` validates the optimisation contract, schema and receipt shape.
- `scripts/optimization_oracle_smoke.sh` now validates receipt digests and the size-reduction invariant as well.
- Optimisation claims must remain receipt-backed and preserve signature warnings.

## Never do silently

- Never invalidate a signature without a warning and receipt.
- Never reduce image quality without explicit policy.
- Never remove attachments or metadata unless requested.
- Never call annotation-only redaction an optimisation.
- Never overwrite the source file before transaction/journal safety is in place.
