//! UI and PDF accessibility audit contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityAuditReport {
    pub report_id: String,
    pub target: AccessibilityTarget,
    pub standard: String,
    pub findings: Vec<AccessibilityFinding>,
    pub summary: AccessibilitySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityTarget { AppUi, PdfDocument, WorkflowUi }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityFinding {
    pub finding_id: String,
    pub severity: AccessibilitySeverity,
    pub area: String,
    pub message: String,
    pub page_index: Option<u32>,
    pub selector_or_object_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilitySeverity { Info, Warning, Error, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySummary {
    pub passed: bool,
    pub error_count: u32,
    pub warning_count: u32,
}
