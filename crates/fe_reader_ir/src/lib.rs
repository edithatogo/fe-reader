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
    /// Extension bag for provider-specific passive facts.
    pub custom: BTreeMap<String, serde_json::Value>,
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
                annotations: Vec::new(),
                images: Vec::new(),
                form_fields: Vec::new(),
                optional_content_refs: Vec::new(),
            }],
            metadata: MetadataIr::default(),
            active_content: Vec::new(),
            revisions: Vec::new(),
            attachments: Vec::new(),
            custom: BTreeMap::new(),
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
        for key in self.custom.keys() {
            if key.is_empty() {
                return Err(IrError::invalid("custom keys must not be empty"));
            }
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
    /// Passive annotation descriptors attached to this page.
    pub annotations: Vec<AnnotationIr>,
    /// Passive image descriptors attached to this page.
    pub images: Vec<ImageIr>,
    /// Passive form field descriptors attached to this page.
    pub form_fields: Vec<FormFieldIr>,
    /// Optional content group references observed on this page.
    pub optional_content_refs: Vec<String>,
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
        for annotation in &self.annotations {
            annotation.validate()?;
        }
        for image in &self.images {
            image.validate()?;
        }
        for field in &self.form_fields {
            field.validate()?;
        }
        if self
            .optional_content_refs
            .iter()
            .any(|reference| reference.is_empty())
        {
            return Err(IrError::invalid(
                "optional_content_refs must not contain empty values",
            ));
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
    /// Font reference when available.
    pub font_ref: Option<String>,
    /// Direction hint.
    pub direction: TextDirection,
    /// Confidence that extracted text maps cleanly to Unicode.
    pub unicode_confidence: Option<f32>,
}

impl TextSpanIr {
    fn validate(&self) -> Result<(), IrError> {
        if self.span_id.is_empty() {
            return Err(IrError::invalid("span_id must not be empty"));
        }
        if !self.bbox.is_non_empty() {
            return Err(IrError::invalid("text span bbox must be non-empty"));
        }
        if matches!(self.font_ref.as_deref(), Some("")) {
            return Err(IrError::invalid("font_ref must not be empty"));
        }
        if let Some(confidence) = self.unicode_confidence
            && !(0.0..=1.0).contains(&confidence)
        {
            return Err(IrError::invalid(
                "unicode_confidence must be between 0.0 and 1.0",
            ));
        }
        Ok(())
    }
}

/// Passive annotation descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnotationIr {
    /// Stable annotation id.
    pub annotation_id: String,
    /// PDF annotation subtype or normalized adapter subtype.
    pub subtype: String,
    /// Annotation bounding box when known.
    pub bbox: Rect,
}

impl AnnotationIr {
    fn validate(&self) -> Result<(), IrError> {
        if self.annotation_id.is_empty() {
            return Err(IrError::invalid("annotation_id must not be empty"));
        }
        if self.subtype.is_empty() {
            return Err(IrError::invalid("annotation subtype must not be empty"));
        }
        if !self.bbox.is_non_empty() {
            return Err(IrError::invalid("annotation bbox must be non-empty"));
        }
        Ok(())
    }
}

/// Passive image descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageIr {
    /// Stable image id.
    pub image_id: String,
    /// Image bounding box.
    pub bbox: Rect,
    /// Image stream hash when available.
    pub sha256: Option<String>,
}

impl ImageIr {
    fn validate(&self) -> Result<(), IrError> {
        if self.image_id.is_empty() {
            return Err(IrError::invalid("image_id must not be empty"));
        }
        if !self.bbox.is_non_empty() {
            return Err(IrError::invalid("image bbox must be non-empty"));
        }
        if let Some(sha256) = &self.sha256 {
            validate_sha256(sha256, "image sha256")?;
        }
        Ok(())
    }
}

/// Passive form field descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormFieldIr {
    /// Stable form field id.
    pub field_id: String,
    /// User-visible field name.
    pub name: String,
    /// Field widget bounding box when known.
    pub bbox: Option<Rect>,
}

impl FormFieldIr {
    fn validate(&self) -> Result<(), IrError> {
        if self.field_id.is_empty() {
            return Err(IrError::invalid("field_id must not be empty"));
        }
        if self.name.is_empty() {
            return Err(IrError::invalid("form field name must not be empty"));
        }
        if matches!(self.bbox, Some(bbox) if !bbox.is_non_empty()) {
            return Err(IrError::invalid("form field bbox must be non-empty"));
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
                inputs: vec!["document_ir".to_string()],
                outputs: vec!["page_model_report".to_string()],
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

    /// Compiles this graph against a passive pass registry.
    ///
    /// Compilation only validates and binds pass metadata. It does not execute transformation
    /// passes, read document bytes, render pages, create patch plans, or mutate a document.
    ///
    /// # Errors
    ///
    /// Returns an error when the graph is invalid or references an unknown or incompatible pass.
    pub fn compile(
        &self,
        registry: &TransformationPassRegistry,
    ) -> Result<CompilationReport, IrError> {
        self.validate()?;
        registry.validate()?;
        let mut accepted_passes = Vec::with_capacity(self.passes.len());
        let mut diagnostics = Vec::new();

        for pass in &self.passes {
            let definition = registry.get(&pass.pass_type).ok_or_else(|| {
                IrError::invalid(format!("unknown pass_type: {}", pass.pass_type))
            })?;
            definition.accept(pass)?;
            accepted_passes.push(CompiledPass {
                pass_id: pass.pass_id.clone(),
                pass_type: pass.pass_type.clone(),
                definition_version: definition.version.clone(),
                policy_risk: pass.policy_risk,
                inputs: pass.inputs.clone(),
                outputs: pass.outputs.clone(),
            });
            diagnostics.push(format!(
                "bound {} to registry definition {}@{}",
                pass.pass_id, definition.pass_type, definition.version
            ));
        }

        Ok(CompilationReport {
            graph_id: self.graph_id.clone(),
            input_document_sha256: self.input_document_sha256.clone(),
            accepted_passes,
            expected_write_mode: self.expected_write_mode,
            mutation_policy: "passive_compile_only_no_execution_or_patch_plan".to_string(),
            diagnostics,
        })
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
    /// Named graph inputs consumed by this pass.
    pub inputs: Vec<String>,
    /// Named graph outputs produced by this pass.
    pub outputs: Vec<String>,
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
        if self.inputs.iter().any(|input| input.is_empty()) {
            return Err(IrError::invalid(
                "pass inputs must not contain empty values",
            ));
        }
        if self.outputs.iter().any(|output| output.is_empty()) {
            return Err(IrError::invalid(
                "pass outputs must not contain empty values",
            ));
        }
        Ok(())
    }
}

/// Passive transformation pass registry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformationPassRegistry {
    /// Registered pass definitions keyed by pass type.
    pub definitions: BTreeMap<String, TransformationPassDefinition>,
}

impl TransformationPassRegistry {
    /// Builds the default preview registry for Wave 0/AA Phase 1 smoke tests.
    #[must_use]
    pub fn preview() -> Self {
        let mut definitions = BTreeMap::new();
        let definition = TransformationPassDefinition {
            pass_type: "InspectPageModel".to_string(),
            version: IR_VERSION.to_string(),
            maturity: PassMaturity::Preview,
            allowed_policy_risks: vec![PolicyRisk::ReadOnly],
            required_inputs: vec!["document_ir".to_string()],
            produced_outputs: vec!["page_model_report".to_string()],
        };
        definitions.insert(definition.pass_type.clone(), definition);
        Self { definitions }
    }

    /// Returns a pass definition by pass type.
    #[must_use]
    pub fn get(&self, pass_type: &str) -> Option<&TransformationPassDefinition> {
        self.definitions.get(pass_type)
    }

    /// Validates registry shape.
    ///
    /// # Errors
    ///
    /// Returns an error when a pass definition is malformed.
    pub fn validate(&self) -> Result<(), IrError> {
        for (pass_type, definition) in &self.definitions {
            if pass_type != &definition.pass_type {
                return Err(IrError::invalid(format!(
                    "registry key {pass_type} does not match definition {}",
                    definition.pass_type
                )));
            }
            definition.validate()?;
        }
        Ok(())
    }
}

/// Passive transformation pass definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformationPassDefinition {
    /// Pass type accepted by this definition.
    pub pass_type: String,
    /// Definition version.
    pub version: String,
    /// Definition maturity.
    pub maturity: PassMaturity,
    /// Policy risks accepted for this pass.
    pub allowed_policy_risks: Vec<PolicyRisk>,
    /// Required named inputs.
    pub required_inputs: Vec<String>,
    /// Outputs produced by this pass.
    pub produced_outputs: Vec<String>,
}

impl TransformationPassDefinition {
    fn validate(&self) -> Result<(), IrError> {
        if self.pass_type.is_empty() {
            return Err(IrError::invalid(
                "pass definition pass_type must not be empty",
            ));
        }
        if self.version.is_empty() {
            return Err(IrError::invalid(
                "pass definition version must not be empty",
            ));
        }
        if self.allowed_policy_risks.is_empty() {
            return Err(IrError::invalid(
                "pass definition allowed_policy_risks must not be empty",
            ));
        }
        validate_non_empty_strings(&self.required_inputs, "pass definition required_inputs")?;
        validate_non_empty_strings(&self.produced_outputs, "pass definition produced_outputs")?;
        Ok(())
    }

    fn accept(&self, pass: &TransformationPassSpec) -> Result<(), IrError> {
        self.validate()?;
        if pass.pass_type != self.pass_type {
            return Err(IrError::invalid(format!(
                "pass {} has type {}, expected {}",
                pass.pass_id, pass.pass_type, self.pass_type
            )));
        }
        if !self.allowed_policy_risks.contains(&pass.policy_risk) {
            return Err(IrError::invalid(format!(
                "pass {} policy risk is not allowed by registry",
                pass.pass_id
            )));
        }
        for required_input in &self.required_inputs {
            if !pass.inputs.contains(required_input) {
                return Err(IrError::invalid(format!(
                    "pass {} missing required input {required_input}",
                    pass.pass_id
                )));
            }
        }
        for produced_output in &self.produced_outputs {
            if !pass.outputs.contains(produced_output) {
                return Err(IrError::invalid(format!(
                    "pass {} missing produced output {produced_output}",
                    pass.pass_id
                )));
            }
        }
        Ok(())
    }
}

/// Passive compilation report for a transformation graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilationReport {
    /// Compiled graph id.
    pub graph_id: String,
    /// SHA-256 of the input document.
    pub input_document_sha256: String,
    /// Passes accepted by the registry.
    pub accepted_passes: Vec<CompiledPass>,
    /// Expected write mode declared by the graph.
    pub expected_write_mode: TransformationWriteMode,
    /// Compile-only mutation policy.
    pub mutation_policy: String,
    /// Human-readable compile diagnostics.
    pub diagnostics: Vec<String>,
}

/// One pass accepted during passive compilation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledPass {
    /// Stable pass id from the graph.
    pub pass_id: String,
    /// Pass type from the graph and registry.
    pub pass_type: String,
    /// Registry definition version.
    pub definition_version: String,
    /// Policy risk accepted for this pass.
    pub policy_risk: PolicyRisk,
    /// Bound graph inputs.
    pub inputs: Vec<String>,
    /// Bound graph outputs.
    pub outputs: Vec<String>,
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

fn validate_non_empty_strings(values: &[String], label: &str) -> Result<(), IrError> {
    if values.is_empty() {
        return Err(IrError::invalid(format!("{label} must not be empty")));
    }
    if values.iter().any(|value| value.is_empty()) {
        return Err(IrError::invalid(format!(
            "{label} must not contain empty values"
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
        assert!(ir.custom.is_empty());
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
        assert_eq!(graph.passes[0].inputs, vec!["document_ir"]);
        assert_eq!(graph.passes[0].outputs, vec!["page_model_report"]);
        assert_eq!(graph.passes[0].policy_risk, PolicyRisk::ReadOnly);
    }

    #[test]
    fn transformation_graph_rejects_duplicate_pass_ids() {
        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes.push(graph.passes[0].clone());

        assert!(graph.validate().is_err());
    }

    #[test]
    fn document_ir_round_trips_extended_passive_nodes() {
        let mut ir = DocumentIr::minimal("fixture:text-search-fixture", SHA256);
        let page = ir.pages.first_mut().unwrap();
        page.text_spans.push(TextSpanIr {
            span_id: "span:1".to_string(),
            text: "Fe Reader".to_string(),
            bbox: Rect::new(10.0, 20.0, 80.0, 12.0),
            font_ref: Some("font:F1".to_string()),
            direction: TextDirection::LeftToRight,
            unicode_confidence: Some(0.99),
        });
        page.annotations.push(AnnotationIr {
            annotation_id: "annot:1".to_string(),
            subtype: "Highlight".to_string(),
            bbox: Rect::new(10.0, 20.0, 80.0, 12.0),
        });
        page.images.push(ImageIr {
            image_id: "image:1".to_string(),
            bbox: Rect::new(0.0, 0.0, 32.0, 32.0),
            sha256: Some(SHA256.to_string()),
        });
        page.form_fields.push(FormFieldIr {
            field_id: "field:1".to_string(),
            name: "signature".to_string(),
            bbox: Some(Rect::new(100.0, 100.0, 120.0, 24.0)),
        });
        page.optional_content_refs.push("ocg:layer-1".to_string());
        ir.custom.insert(
            "adapter_note".to_string(),
            serde_json::json!("passive fixture only"),
        );

        ir.validate().unwrap();
        let serialized = serde_json::to_string(&ir).unwrap();
        let round_trip: DocumentIr = serde_json::from_str(&serialized).unwrap();

        assert_eq!(round_trip, ir);
    }

    #[test]
    fn document_ir_rejects_invalid_nested_hashes() {
        let mut ir = DocumentIr::minimal("fixture:text-search-fixture", SHA256);
        ir.pages[0].images.push(ImageIr {
            image_id: "image:bad".to_string(),
            bbox: Rect::new(0.0, 0.0, 1.0, 1.0),
            sha256: Some("not-a-hash".to_string()),
        });

        assert!(ir.validate().is_err());
    }

    #[test]
    fn transformation_graph_rejects_empty_inputs_outputs() {
        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes[0].outputs.push(String::new());

        assert!(graph.validate().is_err());
    }

    #[test]
    fn preview_registry_compiles_read_only_smoke_graph() {
        let graph = TransformationGraph::read_only_smoke(SHA256);
        let registry = TransformationPassRegistry::preview();

        registry.validate().unwrap();
        let report = graph.compile(&registry).unwrap();

        assert_eq!(report.graph_id, graph.graph_id);
        assert_eq!(report.accepted_passes.len(), 1);
        assert_eq!(report.accepted_passes[0].pass_type, "InspectPageModel");
        assert_eq!(report.accepted_passes[0].policy_risk, PolicyRisk::ReadOnly);
        assert_eq!(
            report.mutation_policy,
            "passive_compile_only_no_execution_or_patch_plan"
        );
    }

    #[test]
    fn compiler_rejects_unknown_or_incompatible_passes() {
        let registry = TransformationPassRegistry::preview();
        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes[0].pass_type = "UnknownPass".to_string();
        assert!(graph.compile(&registry).is_err());

        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes[0].policy_risk = PolicyRisk::HighRiskMutation;
        assert!(graph.compile(&registry).is_err());

        let mut graph = TransformationGraph::read_only_smoke(SHA256);
        graph.passes[0].inputs.clear();
        assert!(graph.compile(&registry).is_err());
    }
}
