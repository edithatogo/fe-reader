# Post-launch PDF Baseline Parity Matrix

This matrix is the Track AY public claim boundary. It is post-launch work and does not block desktop stable launch. The feature gate is `advanced_pdf_baseline`.

Every public claim must point to a fixture, contract smoke, visual regression report, differential oracle, or documented limitation. Write-capable features must stay on:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

The machine-readable source is [`pdf-baseline-parity-matrix.json`](pdf-baseline-parity-matrix.json). Validate it with:

```bash
python3 scripts/pdf_baseline_parity_check.py
```

| Claim ID | Family | Claim | Support level | Evidence or limitation |
|---|---|---|---|---|
| `open-basic-pdf` | Reading | Open and inspect generated baseline PDFs. | Supported preview | `fixtures/corpus/basic/minimal-v1_0.pdf`, `fixtures/corpus/basic/minimal-v2_0.pdf`, `scripts/v8_cli_smoke.sh`, `scripts/pdf_model_contract_check.py` |
| `malformed-safe-open` | Reading | Treat malformed PDFs as expected input and report parser diagnostics without mutating bytes. | Supported preview | `fixtures/corpus/malformed-adversarial/truncated-catalog.pdf`, `scripts/pdf_repair_smoke.sh`, `scripts/pdf_lab_inspect_smoke.sh` |
| `text-search` | Search | Build deterministic text/search evidence for simple generated PDFs. | Supported preview | Evidence exists; complex glyph shaping, OCR and missing ToUnicode cases remain documented corpus placeholders until generated PDF fixtures and oracle results are accepted. |
| `rendering-visual-regression` | Rendering | Run deterministic render smoke and visual regression comparison for the current null-renderer baseline. | Supported preview | Evidence exists; production renderer parity must remain a post-launch claim until reference-renderer oracle results are accepted. |
| `page-organization` | Page organization | Plan page operations through the mutation pipeline without auto-approval. | Plan-only | `scripts/page_ops_contract_check.py`; apply/save paths and UI page organization remain gated. |
| `annotations` | Annotations | Plan annotation operations through core contracts without bypassing policy. | Plan-only | `scripts/annotation_contract_smoke.py`; editable annotation preservation and renderer/UI round trips need additional fixtures. |
| `forms-acroform` | Forms | Plan AcroForm fill/flatten operations with policy-visible write modes. | Plan-only | `scripts/forms_contract_smoke.sh`; XFA and JavaScript execution remain disabled or warning-only until separately approved. |
| `metadata-xmp` | Metadata | Inspect and plan metadata/XMP changes with explicit scrub policies. | Supported preview | `scripts/metadata_wave2_smoke.sh`, `scripts/prepress_smoke.sh`; standards-grade conformance claims require external oracle reports. |
| `secure-redaction` | Redaction | Require sanitizing rewrite and verification receipts for secure redaction plans. | Plan and verify | `fixtures/corpus/redaction/secure-redaction-smoke.recipe.json`, `scripts/redaction_verification_smoke.sh`, `scripts/pdf_lab_redaction_scan_smoke.sh` |
| `conversion-source-pipelines` | Conversion | Expose conversion and source-pipeline planning with capability discovery and clear failure modes. | Plan-only | `scripts/conversion_contract_smoke.sh`, `scripts/source_linked_smoke.sh`; external converters require provider discovery, policy approval and fixtures before output fidelity claims. |
| `signed-documents` | Signatures | Track signed-document compatibility as a documented limitation. | Documented limitation | `fixtures/corpus/signed/README.md`; signature preservation, validation and signing are not public baseline claims yet. |
| `portfolio-attachments` | Attachments and portfolios | Track portfolio and attachment workflows as documented limitations. | Documented limitation | `fixtures/corpus/attachments-portfolios/README.md`; portfolio extraction and embedded-file mutation are not public baseline claims yet. |
| `scanned-ocr` | OCR | Track OCR/searchable-PDF workflows as documented limitations. | Documented limitation | `fixtures/corpus/scanned-ocr/README.md`; OCR remains provider-gated and opt-in. |

## Rollback

If evidence regresses, remove or qualify the affected claim in this file, release notes and homepage copy, then keep the related workflow pack disabled until fixtures and oracle results pass again.
