//! Standards/preflight contract scaffold for PDF/A, PDF/UA and PDF/X reporting.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Summary of one output intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputIntentSummary {
    /// Indirect object id when known.
    pub object_id: Option<String>,
    /// Output intent subtype.
    pub subtype: Option<String>,
    /// Profile description.
    pub profile_description: Option<String>,
    /// Optional page scope.
    pub page_index: Option<u32>,
}

/// Colour finding from a preflight adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColourFinding {
    /// Optional page scope.
    pub page_index: Option<u32>,
    /// Stable finding category.
    pub category: String,
    /// Stable finding code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Whether the finding carries a preservation risk.
    pub preservation_risk: bool,
}

/// Font finding from a preflight adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontFinding {
    /// Optional page scope.
    pub page_index: Option<u32>,
    /// Font name.
    pub font_name: String,
    /// Whether the font is embedded.
    pub embedded: bool,
    /// Whether the font is subsetted.
    pub subset: bool,
    /// Whether a ToUnicode map is present.
    pub has_to_unicode: bool,
    /// Whether the finding represents an extraction risk.
    pub extraction_risk: bool,
}

/// Page-box finding from a preflight adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBoxFinding {
    /// Zero-based page index.
    pub page_index: u32,
    /// Media box.
    pub media_box: [f64; 4],
    /// Crop box if present.
    pub crop_box: Option<[f64; 4]>,
    /// Bleed box if present.
    pub bleed_box: Option<[f64; 4]>,
    /// Trim box if present.
    pub trim_box: Option<[f64; 4]>,
    /// Art box if present.
    pub art_box: Option<[f64; 4]>,
}

/// Report emitted by preflight adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepressReport {
    /// Local document identifier.
    pub document_id: String,
    /// Output-intent summary entries.
    pub output_intents: Vec<OutputIntentSummary>,
    /// Colour findings.
    pub colour_findings: Vec<ColourFinding>,
    /// Font findings.
    pub font_findings: Vec<FontFinding>,
    /// Page-box findings.
    pub page_box_findings: Vec<PageBoxFinding>,
}

impl PrepressReport {
    /// Creates a deterministic smoke report.
    #[must_use]
    pub fn smoke() -> Self {
        Self {
            document_id: "smoke-document".to_string(),
            output_intents: Vec::new(),
            colour_findings: vec![ColourFinding {
                page_index: Some(0),
                category: "device_rgb_placeholder".to_string(),
                code: "colour_smoke".to_string(),
                message: "DeviceRGB placeholder emitted for adapter scaffolding".to_string(),
                preservation_risk: false,
            }],
            font_findings: vec![FontFinding {
                page_index: Some(0),
                font_name: "SmokeFont".to_string(),
                embedded: false,
                subset: false,
                has_to_unicode: false,
                extraction_risk: true,
            }],
            page_box_findings: vec![PageBoxFinding {
                page_index: 0,
                media_box: [0.0, 0.0, 612.0, 792.0],
                crop_box: Some([0.0, 0.0, 612.0, 792.0]),
                bleed_box: None,
                trim_box: None,
                art_box: None,
            }],
        }
    }

    /// Validates report invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are empty or geometries are invalid.
    pub fn validate(&self) -> Result<(), PrepressError> {
        if self.document_id.trim().is_empty() {
            return Err(PrepressError::invalid("document_id must not be empty"));
        }
        if self
            .output_intents
            .iter()
            .any(|entry| entry.profile_description.as_deref() == Some(""))
        {
            return Err(PrepressError::invalid(
                "output intent descriptions must not be empty",
            ));
        }
        if self.page_box_findings.is_empty() {
            return Err(PrepressError::invalid(
                "page_box_findings must contain at least one entry",
            ));
        }
        for finding in &self.page_box_findings {
            if finding.media_box[2] <= finding.media_box[0]
                || finding.media_box[3] <= finding.media_box[1]
            {
                return Err(PrepressError::invalid("media_box must be non-empty"));
            }
        }
        Ok(())
    }
}

/// Prepress report validation error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("prepress error: {message}")]
pub struct PrepressError {
    /// Human-readable validation message.
    pub message: String,
}

impl PrepressError {
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
        let report = PrepressReport::smoke();
        report.validate().unwrap();
        assert_eq!(report.output_intents.len(), 0);
        assert!(!report.colour_findings.is_empty());
    }

    #[test]
    fn report_rejects_empty_document_id() {
        let mut report = PrepressReport::smoke();
        report.document_id.clear();
        assert!(report.validate().is_err());
    }
}
