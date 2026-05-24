# PDF Repair, Recovery and Safe Open

## Why this matters

A serious PDF engine must handle real-world PDFs: malformed xrefs, damaged trailers, missing ToUnicode maps, broken annotations, cyclic page trees, truncated streams, incorrect object lengths, stale incremental revisions and hostile files designed to exhaust memory or time.

Repair is not the same as normal editing. A repair can change document semantics, invalidate signatures and remove forensic evidence. It therefore needs a staged model.

## Modes

| Mode | User promise | Allowed operations |
|---|---|---|
| `NormalOpen` | Open well-formed PDFs with standard validation. | Parse, render, extract, inspect. |
| `SafeOpen` | Open suspect PDFs without active content or risky decoding. | Disable JavaScript/RichMedia/Launch; limit stream decoding; show warning. |
| `DiagnosticOpen` | Analyse structure without trusting the document. | Object graph, xref scan, stream inventory, page tree attempt. |
| `RepairPlanOnly` | Generate a proposed repair without writing. | Rebuild xref candidate, recover page tree, list orphaned objects. |
| `RepairAndSaveCopy` | Save a repaired copy with receipt. | Full rewrite only; never overwrite original by default. |

## Repair classes

```text
xref_rebuild
trailer_recovery
page_tree_recovery
stream_length_repair
object_stream_repair
font_map_warning
annotation_appearance_regeneration
metadata_xmp_normalisation
incremental_revision_prune
orphan_object_cleanup
```

## Signature and forensic policy

- If an existing signature is present, repair must default to `RepairPlanOnly`.
- If repair writes a copy, the receipt must state that signatures may be invalidated.
- Repair output must include original hash, repaired hash, repair classes, object counts and removed object counts.
- Secure redaction can use repair primitives, but must not preserve old sensitive revisions.

## CLI examples

```bash
fe-reader safe-open suspect.pdf --json
fe-reader repair plan suspect.pdf --out repair-plan.json
fe-reader repair apply suspect.pdf repair-plan.json --out suspect.repaired.pdf
fe-reader lab timeline suspect.pdf
```

## Existing tools to use as oracles

Use qpdf for structural comparison and syntax checks, veraPDF for PDF/A and PDF/UA validation, and renderer oracles for visual comparison where available.

## Contracts

See:

```text
contracts/rust/recovery.rs
schemas/recovery-report.schema.json
scripts/pdf_repair_smoke.sh
```
