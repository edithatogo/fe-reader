//! Fe Reader Document IR contract.
//!
//! This is a planning contract, not final implementation code. Keep this crate free
//! of rendering, platform, UI, plugin, MCP and ML dependencies.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIr {
    pub ir_version: String,
    pub document_id: String,
    pub source_sha256: String,
    pub pdf_version: Option<String>,
    pub pages: Vec<PageIr>,
    pub metadata: MetadataIr,
    pub active_content: Vec<ActiveContentFinding>,
    pub revisions: Vec<PdfRevisionIr>,
    pub attachments: Vec<AttachmentIr>,
    pub custom: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageIr {
    pub page_index: u32,
    pub label: Option<String>,
    pub media_box: Rect,
    pub crop_box: Option<Rect>,
    pub rotate_degrees: i32,
    pub text_spans: Vec<TextSpanIr>,
    pub annotations: Vec<AnnotationIr>,
    pub images: Vec<ImageIr>,
    pub form_fields: Vec<FormFieldIr>,
    pub optional_content_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSpanIr {
    pub span_id: String,
    pub text: String,
    pub bbox: Rect,
    pub font_ref: Option<String>,
    pub direction: TextDirection,
    pub unicode_confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextDirection {
    Ltr,
    Rtl,
    Vertical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataIr {
    pub document_info: BTreeMap<String, String>,
    pub xmp_packet_sha256: Option<String>,
    pub standards_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveContentFinding {
    pub finding_id: String,
    pub kind: ActiveContentKind,
    pub object_ref: Option<String>,
    pub risk: RiskLevel,
    pub default_action: QuarantineAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveContentKind {
    JavaScriptAction,
    LaunchAction,
    RemoteUriAction,
    RichMedia,
    EmbeddedExecutable,
    SubmitForm,
    UnknownAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuarantineAction {
    AllowReadOnly,
    DisableByDefault,
    RequireUserApproval,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel { Low, Medium, High, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfRevisionIr {
    pub revision_index: u32,
    pub byte_range_start: u64,
    pub byte_range_end: u64,
    pub object_count: u32,
    pub detected_signatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentIr {
    pub attachment_id: String,
    pub filename: String,
    pub media_type: Option<String>,
    pub size_bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationIr { pub annotation_id: String, pub subtype: String, pub bbox: Rect }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageIr { pub image_id: String, pub bbox: Rect, pub sha256: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldIr { pub field_id: String, pub name: String, pub bbox: Option<Rect> }
