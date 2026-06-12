//! Accessibility audit contracts for Fe Reader.
//!
//! This crate keeps the accessibility report shape deterministic and local-first while the
//! platform-specific inspection adapters remain in later tracks.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Accessibility inspection target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityTarget {
    /// User interface accessibility checks.
    AppUi,
    /// PDF document accessibility checks.
    PdfDocument,
    /// Workflow screen or panel accessibility checks.
    WorkflowUi,
}

/// Accessibility finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilitySeverity {
    /// Informational note.
    Info,
    /// Advisory warning.
    Warning,
    /// Blocking issue.
    Error,
}

/// One accessibility finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityFinding {
    /// Finding target.
    pub target: AccessibilityTarget,
    /// Severity of the finding.
    pub severity: AccessibilitySeverity,
    /// Stable location string.
    pub location: String,
    /// Human-readable explanation.
    pub message: String,
    /// Optional suggested fix.
    pub suggested_fix: Option<String>,
}

/// Accessibility audit summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityAuditReport {
    /// Surface identifier inspected by the adapter.
    pub surface_id: String,
    /// Targets that were requested for inspection.
    pub targets: Vec<AccessibilityTarget>,
    /// Findings emitted by the audit.
    pub findings: Vec<AccessibilityFinding>,
    /// Optional WCAG target used for UI audits.
    pub wcag_target: Option<String>,
}

impl AccessibilityAuditReport {
    /// Creates a deterministic smoke report.
    #[must_use]
    pub fn smoke(surface_id: impl Into<String>) -> Self {
        Self {
            surface_id: surface_id.into(),
            targets: vec![AccessibilityTarget::AppUi, AccessibilityTarget::PdfDocument],
            findings: vec![
                AccessibilityFinding {
                    target: AccessibilityTarget::AppUi,
                    severity: AccessibilitySeverity::Info,
                    location: "toolbar".to_string(),
                    message: "Keyboard and screen-reader labels are present".to_string(),
                    suggested_fix: None,
                },
                AccessibilityFinding {
                    target: AccessibilityTarget::PdfDocument,
                    severity: AccessibilitySeverity::Warning,
                    location: "page-0".to_string(),
                    message: "Tagged PDF structure not yet verified by a full external adapter"
                        .to_string(),
                    suggested_fix: Some("Run the PDF/UA adapter once it is available".to_string()),
                },
            ],
            wcag_target: Some("wcag22-aa".to_string()),
        }
    }

    /// Validates report invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are empty.
    pub fn validate(&self) -> Result<(), AccessibilityAuditError> {
        if self.surface_id.trim().is_empty() {
            return Err(AccessibilityAuditError::invalid(
                "surface_id must not be empty",
            ));
        }
        if self.findings.is_empty() {
            return Err(AccessibilityAuditError::invalid(
                "findings must not be empty",
            ));
        }
        if let Some(target) = &self.wcag_target {
            let allowed = ["none", "wcag22-a", "wcag22-aa", "wcag22-aaa"];
            if !allowed.contains(&target.as_str()) {
                return Err(AccessibilityAuditError::invalid(
                    "wcag_target must be one of none, wcag22-a, wcag22-aa, wcag22-aaa",
                ));
            }
        }
        Ok(())
    }
}

/// Accessibility audit validation error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("accessibility audit error: {message}")]
pub struct AccessibilityAuditError {
    /// Human-readable validation message.
    pub message: String,
}

impl AccessibilityAuditError {
    fn invalid(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn smoke_report_validates() {
        let report = AccessibilityAuditReport::smoke("surface-1");
        report.validate().unwrap();
        assert_eq!(report.targets.len(), 2);
        assert!(!report.findings.is_empty());
    }

    #[test]
    fn report_rejects_empty_surface_id() {
        let mut report = AccessibilityAuditReport::smoke("surface-1");
        report.surface_id.clear();
        assert!(report.validate().is_err());
    }
}
