# Post-launch PDF Baseline Parity Matrix

This matrix is the Track AY public claim boundary. It is post-launch work and does not block desktop stable launch. The feature gate is `advanced_pdf_baseline`, and the matrix is nested under the exhaustive registry in [`pdf-parity-registry.md`](pdf-parity-registry.md).

Every public claim must point to a fixture, contract smoke, visual regression report, differential oracle, or documented limitation. Write-capable features must stay on:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

The machine-readable source is [`pdf-baseline-parity-matrix.json`](pdf-baseline-parity-matrix.json). The exhaustive registry lives in [`pdf-parity-registry.json`](pdf-parity-registry.json). Validate the nested claim boundary with:

```bash
python3 scripts/pdf_parity_registry_check.py
python3 scripts/pdf_baseline_parity_check.py
```

| Claim ID | Family | Claim | Support level | Evidence or limitation |
|---|---|---|---|---|
| `open-basic-pdf` | Reading | Open and inspect generated baseline PDFs. | Supported preview | `fixtures/corpus/basic/minimal-v1_0.pdf`, `fixtures/corpus/basic/minimal-v2_0.pdf`, `scripts/v8_cli_smoke.sh`, `scripts/pdf_model_contract_check.py`, `docs/pdf-parity-registry.md` |
| `reader-navigation` | Reading | Navigate pages, zoom, fit, rotate, inspect thumbnails and restore the current reader session for the baseline PDF experience. | Supported preview | `scripts/v8_cli_smoke.sh`, `scripts/wave1_render_smoke.sh`, `scripts/wave5_integration_smoke.sh`, `docs/stable-reader-readiness.md`; navigation remains limited to the baseline reader workflow until broader UI automation lands. |
| `malformed-safe-open` | Reading | Treat malformed PDFs as expected input and report parser diagnostics without mutating bytes. | Supported preview | `fixtures/corpus/malformed-adversarial/truncated-catalog.pdf`, `scripts/pdf_repair_smoke.sh`, `scripts/pdf_lab_inspect_smoke.sh`, `docs/pdf-parity-registry.md` |
| `text-search` | Search | Build deterministic text/search evidence for simple generated PDFs. | Supported preview | Evidence exists; complex glyph shaping, OCR and missing ToUnicode cases remain documented corpus placeholders until generated PDF fixtures and oracle results are accepted. See `docs/pdf-parity-registry.md`. |
| `text-diagnostics` | Search | Surface deterministic text extraction diagnostics, missing ToUnicode warnings and geometry fallbacks for search and accessibility workflows. | Supported preview | `scripts/metadata_wave2_smoke.sh`, `scripts/pdf_lab_text_map_smoke.sh`, `scripts/search_index_smoke.sh`, `scripts/stable_reader_readiness_check.py`; CJK, RTL and ligature shaping remain corpus-dependent. |
| `rendering-visual-regression` | Rendering | Run deterministic render smoke and visual regression comparison for the current null-renderer baseline. | Supported preview | Evidence exists; production renderer parity must remain a post-launch claim until reference-renderer oracle results are accepted. See `docs/pdf-parity-registry.md`. |
| `accessibility-reader-parity` | Accessibility | Provide keyboard and screen-reader accessibility reports for the reader baseline and tagged-PDF inspection workflows. | Supported preview | `scripts/accessibility_audit_smoke.py`, `scripts/stable_reader_readiness_check.py`, `target/accessibility-reports/smoke.json`; tagged-PDF and PDF/UA oracle coverage remains limited. |
| `page-organization` | Page organization | Plan page operations through the mutation pipeline without auto-approval. | Plan-only | `scripts/page_ops_contract_check.py`; apply/save paths and UI page organization remain gated. |
| `annotations` | Annotations | Plan annotation operations through core contracts without bypassing policy. | Plan-only | `scripts/annotation_contract_smoke.py`; editable annotation preservation and renderer/UI round trips need additional fixtures. |
| `forms-acroform` | Forms | Plan AcroForm fill/flatten operations with policy-visible write modes. | Plan-only | `scripts/forms_contract_smoke.sh`; XFA and JavaScript execution remain disabled or warning-only until separately approved. |
| `metadata-xmp` | Metadata | Inspect and plan metadata/XMP changes with explicit scrub policies. | Supported preview | `scripts/metadata_wave2_smoke.sh`, `scripts/prepress_smoke.sh`; standards-grade conformance claims require external oracle reports. |
| `secure-redaction` | Redaction | Require sanitizing rewrite and verification receipts for secure redaction plans. | Plan and verify | `fixtures/corpus/redaction/secure-redaction-smoke.recipe.json`, `scripts/redaction_verification_smoke.sh`, `scripts/pdf_lab_redaction_scan_smoke.sh`, `docs/pdf-parity-registry.md` |
| `conversion-source-pipelines` | Conversion | Expose conversion and source-pipeline planning with capability discovery and clear failure modes. | Plan-only | `scripts/conversion_contract_smoke.sh`, `scripts/source_linked_smoke.sh`; external converters require provider discovery, policy approval and fixtures before output fidelity claims. See `docs/pdf-parity-registry.md`. |
| `signed-documents` | Signatures | Track signed-document compatibility as a documented limitation. | Documented limitation | `fixtures/corpus/signed/README.md`; signature preservation, validation and signing are not public baseline claims yet. |
| `portfolio-attachments` | Attachments and portfolios | Track portfolio and attachment workflows as documented limitations. | Documented limitation | `fixtures/corpus/attachments-portfolios/README.md`; portfolio extraction and embedded-file mutation are not public baseline claims yet. |
| `scanned-ocr` | OCR | Track OCR/searchable-PDF workflows as documented limitations. | Documented limitation | `fixtures/corpus/scanned-ocr/README.md`; OCR remains provider-gated and opt-in. See `docs/pdf-parity-registry.md`. |
| `searchable-pdf-gating` | OCR | Keep OCR-backed searchable-PDF planning provider-gated and opt-in until quality evidence lands. | Documented limitation | `scripts/ocr_searchable_pdf_contract_smoke.py`, `docs/scanning-ocr-ingestion.md`, `docs/stable-reader-readiness.md`; cloud OCR is not enabled by default and local OCR remains later-provider gated. |

## Rollback

If evidence regresses, remove or qualify the affected claim in this file, release notes, homepage copy and `docs/pdf-parity-registry.md`, then keep the related workflow pack disabled until fixtures and oracle results pass again.
