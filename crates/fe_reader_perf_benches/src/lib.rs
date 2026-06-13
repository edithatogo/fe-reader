//! Deterministic performance benchmark inputs for Linux CI.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{DocumentFingerprint, DocumentId, OperationSource};
use fe_reader_ir::DocumentIr;
use fe_reader_metadata::{
    MetadataOperation, MetadataScrubMode, metadata_snapshot_bytes, plan_metadata_operations,
};
use fe_reader_pdf_model::{extract_text_spans_bytes, inspect_lab_bytes, sniff_pdf_bytes};
use fe_reader_text::summarize_extracted_text;
use fe_reader_workflows::{WorkflowPack, plan_workflow_pack};

/// Minimal fixture bytes used across deterministic benchmarks.
pub const MINIMAL_PDF: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/minimal/minimal.pdf"
));

/// Basic text fixture bytes used for parser and text extraction benchmarks.
pub const TEXT_FIXTURE_PDF: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/corpus/basic/text-search-fixture.pdf"
));

/// Parser hot path: sniff the minimal fixture.
#[must_use]
pub fn parser_sniff() -> usize {
    let summary = sniff_pdf_bytes(MINIMAL_PDF).expect("minimal fixture must be sniffable");
    summary.header.raw.len() + summary.fingerprint.byte_len as usize
}

/// Page-tree hot path: inspect the minimal fixture.
#[must_use]
pub fn page_tree_inspect() -> usize {
    let session = inspect_lab_bytes(MINIMAL_PDF).expect("minimal fixture must inspect");
    session.object_count + session.pages.len()
}

/// Text extraction hot path: extract spans and normalise them.
#[must_use]
pub fn text_extraction() -> usize {
    let summary = extract_text_spans_bytes(TEXT_FIXTURE_PDF).expect("text fixture must extract");
    summary
        .spans
        .iter()
        .map(|span| summarize_extracted_text(&span.text).normalized_char_count)
        .sum()
}

/// Template planning hot path: plan a legal workflow pack.
#[must_use]
pub fn template_planning() -> usize {
    let pack = WorkflowPack::wave3_baseline_packs()
        .into_iter()
        .find(|pack| pack.pack_id == "legal.affidavit.initials.every_page")
        .expect("baseline workflow pack must exist");
    let fingerprint = DocumentFingerprint::from_bytes(MINIMAL_PDF);
    let planned = plan_workflow_pack(OperationSource::Cli, DocumentId::new(), fingerprint, pack);
    planned.plan.operations.len()
}

/// Metadata diff hot path.
#[must_use]
pub fn metadata_diff() -> usize {
    let before = metadata_snapshot_bytes(MINIMAL_PDF);
    let mut summary = before.summary.clone();
    summary.document_info.title = Some("Fe Reader".to_string());
    let after = fe_reader_metadata::MetadataSnapshot::new(summary);
    fe_reader_metadata::diff_metadata_snapshots(before, after)
        .changes
        .len()
}

/// Metadata scrub planning hot path.
#[must_use]
pub fn metadata_scrub(mode: MetadataScrubMode) -> usize {
    let intent = fe_reader_core::OperationIntent::mutation(
        OperationSource::Cli,
        DocumentId::new(),
        fe_reader_core::OperationKind::PlanMutation,
        "metadata_scrub",
    );
    let plan = plan_metadata_operations(&intent, &[MetadataOperation::Scrub { mode }]);
    plan.operations.len()
}

/// IR smoke hot path.
#[must_use]
pub fn document_ir_smoke() -> usize {
    let ir = DocumentIr::minimal(
        "fixture:perf",
        "f7e2b4436614640779c890a882537d543cf4579ae6cc43ad5f43f193afa6cd7f",
    );
    ir.pages.len()
}
