//! Typed Document IR and transformation graph contracts.
//!
//! This crate is intentionally contract-only. It must not parse PDFs, render pages, call platform
//! APIs, execute plugins, perform network access, or mutate document bytes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Current Document IR schema version.
pub const IR_VERSION: &str = "0.1.0";

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Operation-neutral document intermediate representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentIr {
    /// IR schema version.
    pub ir_version: String,
    /// Stable document id used by Fe Reader surfaces.
    pub document_id: String,
    /// SHA-256 of the source bytes used to construct this IR.
    pub source_sha256: String,
    /// PDF version when known.
    pub pdf_version: Option<String>,
    /// Page-level semantic nodes.
    pub pages: Vec<PageIr>,
    /// Metadata layer.
    pub metadata: MetadataIr,
    /// Active-content findings discovered during read-only inspection.
    pub active_content: Vec<ActiveContentFinding>,
    /// Incremental revision timeline, if detected.
    pub revisions: Vec<PdfRevisionIr>,
    /// Embedded-file records.
    pub attachments: Vec<AttachmentIr>,
}

impl DocumentIr {
    /// Builds a minimal single-page IR for smoke tests and schema fixtures.
    #[must_use]
    pub fn minimal(document_id: impl Into<String>, source_sha256: impl Into<String>) -> Self {
        Self {
            ir_version: IR_VERSION.to_string(),
            document_id: document_id.into(),
            source_sha256: source_sha256.into(),
            pdf_version: Some("1.5".to_string()),
            pages: vec![PageIr {
                page_index: 0,
                label: Some("1".to_string()),
                media_box: Rect::new(0.0, 0.0, 612.0, 792.0),
                crop_box: None,
                rotate_degrees: 0,
                text_spans: Vec::new(),
            }],
            metadata: MetadataIr::default(),
            active_content: Vec::new(),
            revisions: Vec::new(),
            attachments: Vec::new(),
        }
    }

    /// Validates invariants represented in the JSON schema plus ordering constraints.
    ///
    /// # Errors
    ///
    /// Returns an error when the IR is malformed.
    pub fn validate(&self) -> Result<(), IrError> {
        validate_sha256(&self.source_sha256, "source_sha256")?;
        if self.ir_version.is_empty() {
            return Err(IrError::invalid("ir_version must not be empty"));
        }
        if self.document_id.is_empty() {
            return Err(IrError::invalid("document_id must not be empty"));
        }
        for (expected_index, page) in self.pages.iter().enumerate() {
            page.validate(expected_index as u32)?;
        }
        for attachment in &self.attachments {
            validate_sha256(&attachment.sha256, "attachment sha256")?;
        }
        Ok(())
    }
}

/// One page in the document IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageIr {
    /// Zero-based page index.
    pub page_index: u32,
    /// User-visible page label, if known.
    pub label: Option<String>,
    /// MediaBox in PDF user-space points.
    pub media_box: Rect,
    /// CropBox in PDF user-space points.
    pub crop_box: Option<Rect>,
    /// Clockwise rotation in degrees.
    pub rotate_degrees: i32,
    /// Text spans attached to this page.
    pub text_spans: Vec<TextSpanIr>,
}

impl PageIr {
    fn validate(&self, expected_index: u32) -> Result<(), IrError> {
        if self.page_index != expected_index {
            return Err(IrError::invalid(format!(
                "page_index must be contiguous: expected {expected_index}, got {}",
                self.page_index
            )));
        }
        if !self.media_box.is_non_empty() {
            return Err(IrError::invalid("media_box must be non-empty"));
        }
        if !matches!(self.rotate_degrees, 0 | 90 | 180 | 270) {
            return Err(IrError::invalid(
                "rotate_degrees must be 0, 90, 180, or 270",
            ));
        }
        for span in &self.text_spans {
            span.validate()?;
        }
        Ok(())
    }
}

/// Axis-aligned rectangle in PDF user-space points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Left x coordinate.
    pub x: f64,
    /// Bottom y coordinate.
    pub y: f64,
    /// Width in points.
    pub width: f64,
    /// Height in points.
    pub height: f64,
}

impl Rect {
    /// Creates a rectangle.
    #[must_use]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns true when the rectangle has positive dimensions.
    #[must_use]
    pub fn is_non_empty(self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}

/// Text span node in the document IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSpanIr {
    /// Stable span id within the document IR.
    pub span_id: String,
    /// Extracted text.
    pub text: String,
    /// Bounding box in PDF user-space points.
    pub bbox: Rect,
    /// Direction hint.
    pub direction: TextDirection,
}

impl TextSpanIr {
    fn validate(&self) -> Result<(), IrError> {
        if self.span_id.is_empty() {
            return Err(IrError::invalid("span_id must not be empty"));
        }
        if !self.bbox.is_non_empty() {
            return Err(IrError::invalid("text span bbox must be non-empty"));
        }
        Ok(())
    }
}

/// Text direction hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextDirection {
    /// Left-to-right text.
    LeftToRight,
    /// Right-to-left text.
    RightToLeft,
    /// Vertical writing.
    Vertical,
    /// Unknown or mixed direction.
    MixedOrUnknown,
}

/// Document metadata layer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataIr {
    /// Document info dictionary fields.
    pub document_info: BTreeMap<String, String>,
    /// XMP packet hash when present and extracted.
    pub xmp_packet_sha256: Option<String>,
    /// Standards claims observed in metadata.
    pub standards_claims: Vec<String>,
}

/// Active content finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveContentFinding {
    /// Stable finding id.
    pub finding_id: String,
    /// Finding kind.
    pub kind: ActiveContentKind,
    /// Object reference if known.
    pub object_ref: Option<String>,
    /// Risk classification.
    pub risk: ActiveContentRisk,
    /// Default quarantine action.
    pub default_action: QuarantineAction,
}

/// Active content category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveContentKind {
    /// JavaScript action.
    JavaScriptAction,
    /// Launch action.
    LaunchAction,
    /// Remote URI action.
    RemoteUriAction,
    /// Rich media content.
    RichMedia,
    /// Embedded executable.
    EmbeddedExecutable,
    /// Submit form action.
    SubmitForm,
    /// Unknown action.
    UnknownAction,
}

/// Active content risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveContentRisk {
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Critical risk.
    Critical,
}

/// Quarantine decision for active content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineAction {
    /// Allow read-only inspection.
    AllowReadOnly,
    /// Disable by default.
    DisableByDefault,
    /// Require user approval.
    RequireUserApproval,
    /// Block outright.
    Block,
}

/// Incremental PDF revision record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfRevisionIr {
    /// Zero-based revision index.
    pub revision_index: u32,
    /// Inclusive byte range start.
    pub byte_range_start: u64,
    /// Exclusive byte range end.
    pub byte_range_end: u64,
    /// Number of objects in the revision.
    pub object_count: u32,
    /// Detected signature ids.
    pub detected_signatures: Vec<String>,
}

/// Embedded-file record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentIr {
    /// Stable attachment id.
    pub attachment_id: String,
    /// File name.
    pub filename: String,
    /// Media type if known.
    pub media_type: Option<String>,
    /// Attachment size in bytes.
    pub size_bytes: u64,
    /// Attachment SHA-256.
    pub sha256: String,
}

/// Transformation graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationGraph {
    /// Stable graph id.
    pub graph_id: String,
    /// SHA-256 of the input document.
    pub input_document_sha256: String,
    /// Ordered transformation passes.
    pub passes: Vec<TransformationPassSpec>,
    /// Expected write mode if applied.
    pub expected_write_mode: TransformationWriteMode,
}

impl TransformationGraph {
    /// Creates a read-only graph with one preview pass.
    #[must_use]
    pub fn read_only_smoke(input_document_sha256: impl Into<String>) -> Self {
        Self {
            graph_id: "graph:read-only-smoke".to_string(),
            input_document_sha256: input_document_sha256.into(),
            passes: vec![TransformationPassSpec {
                pass_id: "pass:inspect-page-model".to_string(),
                pass_type: "InspectPageModel".to_string(),
                maturity: PassMaturity::Preview,
                policy_risk: PolicyRisk::ReadOnly,
                parameters: serde_json::json!({}),
            }],
            expected_write_mode: TransformationWriteMode::IncrementalAppend,
        }
    }

    /// Validates graph invariants represented in the JSON schema.
    ///
    /// # Errors
    ///
    /// Returns an error when the graph is malformed.
    pub fn validate(&self) -> Result<(), IrError> {
        validate_sha256(&self.input_document_sha256, "input_document_sha256")?;
        if self.graph_id.is_empty() {
            return Err(IrError::invalid("graph_id must not be empty"));
        }
        let mut seen = BTreeSet::new();
        for pass in &self.passes {
            pass.validate()?;
            if !seen.insert(pass.pass_id.clone()) {
                return Err(IrError::invalid(format!(
                    "duplicate pass_id: {}",
                    pass.pass_id
                )));
            }
        }
        Ok(())
    }
}

/// Transformation pass specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationPassSpec {
    /// Stable pass id.
    pub pass_id: String,
    /// Pass type name.
    pub pass_type: String,
    /// Pass maturity.
    pub maturity: PassMaturity,
    /// Policy risk.
    pub policy_risk: PolicyRisk,
    /// Pass parameters.
    pub parameters: serde_json::Value,
}

impl TransformationPassSpec {
    fn validate(&self) -> Result<(), IrError> {
        if self.pass_id.is_empty() {
            return Err(IrError::invalid("pass_id must not be empty"));
        }
        if self.pass_type.is_empty() {
            return Err(IrError::invalid("pass_type must not be empty"));
        }
        Ok(())
    }
}

/// Transformation pass maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassMaturity {
    /// Experimental pass.
    Experimental,
    /// Preview pass.
    Preview,
    /// Stable pass.
    Stable,
}

/// Transformation pass policy risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRisk {
    /// Read-only pass.
    ReadOnly,
    /// Low-risk mutation.
    LowMutation,
    /// High-risk mutation.
    HighRiskMutation,
    /// Security-critical pass.
    SecurityCritical,
}

/// Expected write mode for a transformation graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformationWriteMode {
    /// Incremental append.
    IncrementalAppend,
    /// Full rewrite.
    FullRewrite,
    /// Full sanitizing rewrite.
    FullSanitizingRewrite,
}

/// IR validation error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("IR validation error: {message}")]
pub struct IrError {
    /// Human-readable validation message.
    pub message: String,
}

impl IrError {
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

fn validate_sha256(value: &str, label: &str) -> Result<(), IrError> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(IrError::invalid(format!(
            "{label} must be a 64-character hex SHA-256"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA256: &str = "f7e2b4436614640779c890a882537d543cf4579ae6cc43ad5f43f193afa6cd7f";

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn minimal_document_ir_validates() {
        let ir = DocumentIr::minimal("fixture:text-search-fixture", SHA256);

        ir.validate().unwrap();
        assert_eq!(ir.ir_version, IR_VERSION);
        assert_eq!(ir.pages.len(), 1);
    }

    #[test]
    fn document_ir_rejects_bad_hash_and_page_order() {
        let mut ir = DocumentIr::minimal("fixture:text-search-fixture", "not-a-hash");
        assert!(ir.validate().is_err());

        ir.source_sha256 = SHA256.to_string();
        ir.pages[0].page_index = 3;
        assert!(ir.validate().is_err());
    }

    #[test]
    fn transformation_graph_validates_unique_passes() {
        let graph = TransformationGraph::read_only_smoke(SHA256);
        graph.validate().unwrap();
        assert_eq!(
            graph.expected_write_mode,
            TransformationWriteMode::IncrementalAppend
        );
    }

    #[test]
    fn transformation_graph_rejects_duplicate_pass_ids() {
        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes.push(graph.passes[0].clone());

        assert!(graph.validate().is_err());
    }
}
