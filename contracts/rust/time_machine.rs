//! PDF revision/time-machine contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfTimeMachineReport {
    pub document_sha256: String,
    pub revisions: Vec<RevisionSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionSummary {
    pub revision_index: u32,
    pub byte_start: u64,
    pub byte_end: u64,
    pub object_count: u32,
    pub changed_objects: Vec<String>,
    pub signatures: Vec<RevisionSignatureSummary>,
    pub metadata_digest: Option<String>,
    pub active_content_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionSignatureSummary {
    pub signature_id: String,
    pub covers_revision: bool,
    pub validation_state: String,
}

pub trait PdfTimeMachine {
    fn analyse_revisions(&self, pdf_bytes: &[u8]) -> Result<PdfTimeMachineReport, TimeMachineError>;
    fn diff_revisions(&self, pdf_bytes: &[u8], left: u32, right: u32) -> Result<RevisionDiff, TimeMachineError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionDiff { pub left: u32, pub right: u32, pub changed_objects: Vec<String>, pub summary: String }

#[derive(Debug)]
pub struct TimeMachineError { pub code: String, pub message: String }
