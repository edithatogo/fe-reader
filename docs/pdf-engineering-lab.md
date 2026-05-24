# PDF Engineering Lab

## Purpose

Fe Reader should include a developer/power-user mode that explains *why* a PDF behaves the way it does. This is not just a debugging tool for contributors; it becomes a product differentiator for publishers, accessibility auditors, forensic workflows, prepress engineers and standards validators.

The lab must be read-only by default. It can propose repairs or rewrites, but any mutation must go through:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

## Capabilities

| Capability | Description | Wave |
|---|---|---:|
| Object tree inspector | Show trailer, xref, objects, streams, dictionaries, object ownership and references. | 1 |
| Page graph inspector | Show page tree inheritance, resources, media/crop/bleed/trim/art boxes, rotation and inherited attributes. | 1 |
| Content stream disassembler | Decode operators, graphics state, text state, path construction, clipping, XObject invocation and marked content. | 2 |
| Text/glyph map inspector | Show font, glyph id, Unicode mapping, ToUnicode availability, ligatures, writing mode and extraction confidence. | 2 |
| Annotation/form inspector | Show annotations, form fields, widget relationships, appearances, actions and JavaScript risk. | 2 |
| XMP/document-info inspector | Compare Info dictionary, XMP, custom schemas and workflow metadata. | 2 |
| Colour/prepress inspector | Show OutputIntents, ICC profiles, DeviceN, spot colours, overprint, transparency groups and blend modes. | 4 |
| Incremental update timeline | Show revisions, changed objects, byte ranges and signature invalidation risk. | 3 |
| Redaction leak scanner | Show residual sensitive text/images/object streams after redaction output. | 3 |
| Repair planner | Propose xref rebuild, orphan cleanup, stream decode repair and page tree recovery. | 3 |

## UI surface

The lab should be available through:

```text
fe-reader lab inspect input.pdf --json
fe-reader lab object input.pdf 42 --decode-stream
fe-reader lab page input.pdf --page 5 --resources
fe-reader lab text-map input.pdf --page 5
fe-reader lab timeline input.pdf
fe-reader lab redaction-scan redacted.pdf --receipt receipt.json
```

The graphical app may expose the same data as an optional “Document Internals” panel. Normal users should never need it, but advanced users and contributors should be able to diagnose rendering, extraction and conversion errors without leaving Fe Reader.

## Safety requirements

- Do not execute PDF JavaScript, RichMedia, Launch actions or embedded-file launch actions while inspecting.
- Decode streams under resource limits.
- Mark malformed, truncated or cyclic structures clearly.
- Never make a repair automatically just because inspection found a defect.
- Provide copyable diagnostic bundles that exclude document content by default.

## Data model

See `contracts/rust/pdf_lab.rs` and `schemas/pdf-lab-session.schema.json`.
