//! Local-first quality dashboard contracts.
//!
//! This crate aggregates local report metadata only. It does not upload analytics, phone home,
//! inspect PDFs, render pages, or mutate documents.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Dashboard schema version.
pub const DASHBOARD_VERSION: &str = "0.1.0";

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Quality report status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    /// Passing report.
    Pass,
    /// Warning report.
    Warn,
    /// Failing report.
    Fail,
    /// Report not run.
    NotRun,
}

/// One report entry shown on the public quality dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityReportEntry {
    /// Report kind.
    pub kind: String,
    /// Report status.
    pub status: ReportStatus,
    /// Local artifact path.
    pub artifact: String,
}

/// Public quality dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicQualityDashboard {
    /// Dashboard schema version.
    pub dashboard_version: String,
    /// Generation timestamp or stable smoke timestamp.
    pub generated_at: String,
    /// Report entries.
    pub reports: Vec<QualityReportEntry>,
}

impl PublicQualityDashboard {
    /// Creates a deterministic smoke dashboard.
    #[must_use]
    pub fn smoke() -> Self {
        Self {
            dashboard_version: DASHBOARD_VERSION.to_string(),
            generated_at: "1970-01-01T00:00:00Z".to_string(),
            reports: vec![
                QualityReportEntry {
                    kind: "schemas".to_string(),
                    status: ReportStatus::Pass,
                    artifact: "scripts/validate_schemas.py".to_string(),
                },
                QualityReportEntry {
                    kind: "search_index".to_string(),
                    status: ReportStatus::Pass,
                    artifact: "scripts/search_index_smoke.sh".to_string(),
                },
            ],
        }
    }

    /// Validates dashboard invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are empty.
    pub fn validate(&self) -> Result<(), QualityDashboardError> {
        if self.dashboard_version.trim().is_empty() {
            return Err(QualityDashboardError::invalid(
                "dashboard_version must not be empty",
            ));
        }
        if self.generated_at.trim().is_empty() {
            return Err(QualityDashboardError::invalid(
                "generated_at must not be empty",
            ));
        }
        for report in &self.reports {
            if report.kind.trim().is_empty() {
                return Err(QualityDashboardError::invalid(
                    "report kind must not be empty",
                ));
            }
            if report.artifact.trim().is_empty() {
                return Err(QualityDashboardError::invalid(
                    "report artifact must not be empty",
                ));
            }
        }
        Ok(())
    }
}

/// Quality dashboard validation error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("quality dashboard error: {message}")]
pub struct QualityDashboardError {
    /// Human-readable validation message.
    pub message: String,
}

impl QualityDashboardError {
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
    fn smoke_dashboard_validates() {
        let dashboard = PublicQualityDashboard::smoke();
        dashboard.validate().unwrap();
        assert_eq!(dashboard.reports.len(), 2);
    }

    #[test]
    fn dashboard_rejects_empty_report_artifact() {
        let mut dashboard = PublicQualityDashboard::smoke();
        dashboard.reports[0].artifact.clear();
        assert!(dashboard.validate().is_err());
    }
}
