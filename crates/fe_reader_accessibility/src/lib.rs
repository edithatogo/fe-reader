//! Accessibility audit contracts for Fe Reader.
//!
//! This crate keeps the accessibility report shape deterministic and local-first while the
//! platform-specific inspection adapters remain in later tracks.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_pdf_model::PdfTextExtractionSummary;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};

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

    /// Builds a PDF accessibility report from extracted text diagnostics.
    #[must_use]
    pub fn from_text_extraction(
        surface_id: impl Into<String>,
        extraction: &PdfTextExtractionSummary,
    ) -> Self {
        let mut findings = Vec::new();
        findings.push(AccessibilityFinding {
            target: AccessibilityTarget::PdfDocument,
            severity: AccessibilitySeverity::Info,
            location: "text-layer".to_string(),
            message: format!(
                "extraction adapter {} returned {} span(s)",
                extraction.adapter,
                extraction.spans.len()
            ),
            suggested_fix: None,
        });
        if extraction.spans.is_empty() {
            findings.push(AccessibilityFinding {
                target: AccessibilityTarget::PdfDocument,
                severity: AccessibilitySeverity::Warning,
                location: "text-layer".to_string(),
                message: "no extractable text spans were reported".to_string(),
                suggested_fix: Some(
                    "Inspect the tagged PDF structure and text extraction fallback".to_string(),
                ),
            });
        }
        if !extraction.precise_geometry {
            findings.push(AccessibilityFinding {
                target: AccessibilityTarget::PdfDocument,
                severity: AccessibilitySeverity::Warning,
                location: "text-layer".to_string(),
                message: "geometry is reported with fallback precision".to_string(),
                suggested_fix: Some(
                    "Use the reading-order and tagged-structure inspection adapters".to_string(),
                ),
            });
        }
        for diagnostic in &extraction.diagnostics {
            findings.push(AccessibilityFinding {
                target: AccessibilityTarget::PdfDocument,
                severity: AccessibilitySeverity::Info,
                location: "diagnostics".to_string(),
                message: diagnostic.clone(),
                suggested_fix: None,
            });
        }
        if let Some(error) = &extraction.error {
            findings.push(AccessibilityFinding {
                target: AccessibilityTarget::PdfDocument,
                severity: AccessibilitySeverity::Error,
                location: "parser".to_string(),
                message: error.clone(),
                suggested_fix: Some(
                    "Resolve the text extraction error before auditing accessibility".to_string(),
                ),
            });
        }
        Self {
            surface_id: surface_id.into(),
            targets: vec![AccessibilityTarget::PdfDocument],
            findings,
            wcag_target: Some("wcag22-aa".to_string()),
        }
    }

    /// Builds an accessibility audit report from a PDF file using local toolchain adapters when
    /// available.
    ///
    /// # Errors
    ///
    /// Returns an error if the PDF cannot be inspected or a configured external tool fails.
    pub fn from_pdf_path(path: impl AsRef<Path>) -> Result<Self, AccessibilityAuditError> {
        let path = path.as_ref();
        let summary = fe_reader_pdf_model::sniff_pdf_path(path)
            .map_err(|error| AccessibilityAuditError::invalid(error.to_string()))?;
        let extraction = fe_reader_pdf_model::extract_text_spans_path(path)
            .map_err(|error| AccessibilityAuditError::invalid(error.to_string()))?;
        let mut report = Self::from_text_extraction(summary.document_id.to_string(), &extraction);
        if let Ok(pdfinfo) = run_pdfinfo(path) {
            if let Some(tagged) = pdfinfo.tagged {
                report.findings.push(AccessibilityFinding {
                    target: AccessibilityTarget::PdfDocument,
                    severity: if tagged {
                        AccessibilitySeverity::Info
                    } else {
                        AccessibilitySeverity::Warning
                    },
                    location: "pdfinfo".to_string(),
                    message: if tagged {
                        "pdfinfo reported a tagged PDF".to_string()
                    } else {
                        "pdfinfo did not report tagged PDF structure".to_string()
                    },
                    suggested_fix: if tagged {
                        None
                    } else {
                        Some(
                            "Inspect the tagged structure and reading order before accessibility release"
                                .to_string(),
                        )
                    },
                });
            }
            if let Some(page_count) = pdfinfo.page_count {
                if page_count == 0 {
                    report.findings.push(AccessibilityFinding {
                        target: AccessibilityTarget::PdfDocument,
                        severity: AccessibilitySeverity::Error,
                        location: "pdfinfo".to_string(),
                        message: "pdfinfo reported zero pages".to_string(),
                        suggested_fix: Some(
                            "Verify the input file and parser before running accessibility audit"
                                .to_string(),
                        ),
                    });
                }
            }
        }
        Ok(report)
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

struct PdfInfoAdapterReport {
    page_count: Option<u32>,
    tagged: Option<bool>,
}

fn run_pdfinfo(path: &Path) -> Result<PdfInfoAdapterReport, AccessibilityAuditError> {
    let output = Command::new("pdfinfo")
        .arg(path)
        .output()
        .map_err(|error| AccessibilityAuditError::invalid(format!("pdfinfo failed: {error}")))?;
    if !output.status.success() {
        return Err(AccessibilityAuditError::invalid(format!(
            "pdfinfo exited unsuccessfully: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut page_count = None;
    let mut tagged = None;
    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("Pages:") {
            page_count = value.trim().parse::<u32>().ok();
        } else if let Some(value) = line.strip_prefix("Tagged:") {
            tagged = Some(value.trim().eq_ignore_ascii_case("yes"));
        }
    }
    Ok(PdfInfoAdapterReport { page_count, tagged })
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

    #[test]
    fn text_extraction_populates_pdf_findings() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("fixtures")
            .join("corpus")
            .join("basic")
            .join("text-search-fixture.pdf");
        let extraction = fe_reader_pdf_model::extract_text_spans_path(&fixture).unwrap();
        let report = AccessibilityAuditReport::from_text_extraction("surface-2", &extraction);
        assert!(!report.findings.is_empty());
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.target == AccessibilityTarget::PdfDocument)
        );
    }

    #[test]
    fn path_adapter_uses_system_reports() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("fixtures")
            .join("corpus")
            .join("basic")
            .join("text-search-fixture.pdf");
        let report = AccessibilityAuditReport::from_pdf_path(&fixture).unwrap();
        assert!(!report.findings.is_empty());
        assert!(!report.surface_id.is_empty());
    }
}
