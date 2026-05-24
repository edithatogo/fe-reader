//! Contract sketch for the PDF Engineering Lab.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfLabSession {
    pub session_id: String,
    pub document_id: String,
    pub document_sha256: String,
    pub mode: LabMode,
    pub findings: Vec<LabFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabMode {
    ObjectTree,
    PageGraph,
    ContentStream,
    TextGlyphMap,
    AnnotationForms,
    Metadata,
    ColourPrepress,
    IncrementalTimeline,
    RedactionLeakScan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabFinding {
    pub severity: FindingSeverity,
    pub location: PdfLocation,
    pub code: String,
    pub message: String,
    pub suggested_next_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingSeverity { Info, Warning, Error, SecurityRisk }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfLocation {
    pub object_id: Option<String>,
    pub page_index: Option<u32>,
    pub byte_range: Option<(u64, u64)>,
    pub operator_index: Option<u32>,
}
