//! Compatibility and differential-oracle contracts for Fe Reader.
//!
//! External engines are adapters. This crate records normalized comparison outcomes without
//! invoking tools directly.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a stable identity string for diagnostics.
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Differential oracle operation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleOperation {
    /// Structural syntax validation.
    SyntaxValidity,
    /// Rendered image comparison.
    RenderVisualSimilarity,
    /// Extracted text comparison.
    TextExtractionSimilarity,
    /// Metadata snapshot or roundtrip comparison.
    MetadataRoundtrip,
    /// Redaction leak absence comparison.
    RedactionLeakAbsence,
    /// Repair semantic delta comparison.
    RepairSemanticDelta,
    /// Conversion quality comparison.
    ConversionOutputQuality,
    /// Accessibility validator comparison.
    AccessibilityValidation,
    /// Prepress validator comparison.
    PrepressValidation,
}

/// Oracle result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleStatus {
    /// Fe Reader and oracle outputs match.
    Match,
    /// Outputs differ inside an accepted tolerance.
    AcceptableDelta,
    /// Outputs disagree and need classification.
    Disagreement,
    /// Oracle executable or runtime was unavailable.
    OracleUnavailable,
}

/// Classified disagreement type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementClass {
    /// Fe Reader appears incorrect.
    FeReaderBug,
    /// The oracle has a known limitation or bug.
    OracleBugOrLimitation,
    /// PDF specification behaviour is ambiguous.
    SpecAmbiguous,
    /// Fixture is invalid for the claimed test class.
    FixtureInvalid,
    /// Fe Reader has a documented feature gap.
    KnownFeatureGap,
    /// Difference is expected because Fe Reader policy is stricter.
    SecurityPolicyDifference,
}

/// One normalized oracle tool result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleToolResult {
    /// Tool name, such as qpdf, poppler, mupdf or pdfium.
    pub tool: String,
    /// Optional tool version.
    pub version: Option<String>,
    /// Optional process exit code.
    pub exit_code: Option<i32>,
    /// Optional normalized output hash.
    pub normalized_output_sha256: Option<String>,
    /// Tool warnings retained after normalisation.
    pub warnings: Vec<String>,
}

impl OracleToolResult {
    /// Creates an unavailable-tool result.
    #[must_use]
    pub fn unavailable(tool: impl Into<String>, warning: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            version: None,
            exit_code: None,
            normalized_output_sha256: None,
            warnings: vec![warning.into()],
        }
    }
}

/// Comparison summary across Fe Reader and oracle outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleComparison {
    /// Comparison status.
    pub status: OracleStatus,
    /// Optional classified disagreement.
    pub disagreement_class: Option<DisagreementClass>,
    /// Human-readable notes suitable for local quality reports.
    pub notes: Vec<String>,
}

/// Schema-compatible differential test report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifferentialTestReport {
    /// Stable run id.
    pub run_id: String,
    /// Fixture id from the corpus manifest.
    pub fixture_id: String,
    /// Operation class.
    pub operation: OracleOperation,
    /// Tool results included in this report.
    pub tools: Vec<OracleToolResult>,
    /// Comparison outcome.
    pub comparison: OracleComparison,
}

impl DifferentialTestReport {
    /// Builds the Wave 1 smoke report for a simple render fixture.
    #[must_use]
    pub fn wave1_render_smoke(fixture_id: impl Into<String>) -> Self {
        Self {
            run_id: "wave1-render-smoke".to_string(),
            fixture_id: fixture_id.into(),
            operation: OracleOperation::RenderVisualSimilarity,
            tools: vec![OracleToolResult::unavailable(
                "external-render-oracle",
                "external rendering oracle not configured; null render contract used for smoke",
            )],
            comparison: OracleComparison {
                status: OracleStatus::OracleUnavailable,
                disagreement_class: Some(DisagreementClass::KnownFeatureGap),
                notes: vec![
                    "Wave 1 captures fixture-linked oracle report shape before enabling external tools"
                        .to_string(),
                ],
            },
        }
    }

    /// Validates required report invariants before writing JSON evidence.
    ///
    /// # Errors
    ///
    /// Returns an error string when required fields are missing or inconsistent.
    pub fn validate(&self) -> Result<(), String> {
        if self.run_id.trim().is_empty() {
            return Err("run_id must not be empty".to_string());
        }
        if self.fixture_id.trim().is_empty() {
            return Err("fixture_id must not be empty".to_string());
        }
        if self.tools.is_empty() {
            return Err("at least one oracle tool result is required".to_string());
        }
        if self.comparison.status == OracleStatus::Disagreement
            && self.comparison.disagreement_class.is_none()
        {
            return Err("disagreements require a disagreement_class".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn wave1_render_smoke_report_validates() {
        let report = DifferentialTestReport::wave1_render_smoke("basic-text-search-fixture");
        report.validate().unwrap();
        assert_eq!(report.operation, OracleOperation::RenderVisualSimilarity);
        assert_eq!(report.comparison.status, OracleStatus::OracleUnavailable);
    }

    #[test]
    fn disagreement_requires_classification() {
        let mut report = DifferentialTestReport::wave1_render_smoke("fixture");
        report.comparison.status = OracleStatus::Disagreement;
        report.comparison.disagreement_class = None;
        assert_eq!(
            report.validate().unwrap_err(),
            "disagreements require a disagreement_class"
        );
    }
}
