//! Accessibility contracts for Fe Reader UI, web and workflow surfaces.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityTarget {
    KeyboardNavigation,
    ScreenReaderLabels,
    HighContrast,
    ReducedMotion,
    TouchTargets,
    PdfTaggedStructure,
    ReadingOrder,
    AltText,
}

#[derive(Debug, Clone)]
pub struct AccessibilityAuditRequest {
    pub surface_id: String,
    pub targets: Vec<AccessibilityTarget>,
    pub include_pdf_accessibility: bool,
}

#[derive(Debug, Clone)]
pub struct AccessibilityFinding {
    pub target: AccessibilityTarget,
    pub severity: String,
    pub location: String,
    pub message: String,
    pub suggested_fix: Option<String>,
}

pub trait AccessibilityAuditor: Send + Sync {
    fn audit(&self, request: AccessibilityAuditRequest) -> Result<Vec<AccessibilityFinding>, String>;
}
