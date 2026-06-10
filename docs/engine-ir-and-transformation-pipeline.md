# Document IR and Transformation Pipeline

## Purpose

PDFs are object graphs, not word-processing documents. Fe Reader should not implement each operation as a direct one-off mutation against a low-level PDF library. Instead, mutating features should compile into a typed intermediate representation and run through transformation passes.

## Crates

```text
fe_reader_pdf_model   # parsed semantic model: pages, boxes, spans, annotations, forms, metadata
fe_reader_ir          # operation-neutral document IR and transformation graph
fe_reader_core        # patch planning, policy, verification and receipts
fe_reader_compat      # PDF syntax and standard-specific adapters
```

## Pipeline

```text
PDF bytes
  -> parse/index object graph
  -> build semantic page/document model
  -> build DocumentIR
  -> apply TransformationPass sequence
  -> produce PatchPlan
  -> policy/review gate
  -> transaction journal
  -> write through selected writer backend
  -> verify output
  -> emit receipt
```

## Transformation pass examples

```text
NormalizePageBoxesPass
ResolvePageLabelsPass
BuildTextSpanLayerPass
BuildAnnotationLayerPass
PlanWatermarkPass
PlanHeaderFooterPass
PlanPageReorderPass
PlanSecureRedactionPass
PlanAffidavitStampPass
PlanMetadataScrubPass
PlanPdfAConformanceFixPass
DetectActiveContentPass
DetectIncrementalRevisionsPass
```

## Patch-plan binding

- Passive transformation compilation can attach a graph id and ordered pass ids to a patch plan.
- The binding metadata is optional and does not approve a plan or change its write mode.
- Transaction journals can carry the graph id for recovery and audit correlation.

## Why not mutate directly?

Direct low-level mutation is useful for small experiments but risky for a platform. A transformation pipeline makes changes testable, explainable, undoable, fuzzable and comparable across low-level writer implementations.

## Contracts

See:

```text
contracts/rust/document_ir.rs
contracts/rust/transformation_pass.rs
schemas/document-ir.schema.json
schemas/transformation-pass.schema.json
```

## Acceptance criteria

- Every operation records the transformation passes used.
- The CLI can emit the planned transformation graph before applying it.
- Differential tests can compare two writer backends for the same PatchPlan.
- The object inspector can trace which IR nodes produced which PDF objects.
