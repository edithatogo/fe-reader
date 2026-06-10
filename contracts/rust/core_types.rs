//! Core cross-surface contract types. These live in fe_reader_core.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeOperationSource {
    Ui { session_id: String },
    Cli { argv_fingerprint: String },
    Mpc { client_id: String, tool_call_id: String },
    Plugin { plugin_id: String, plugin_version: String },
    WindowsCom { client_process: String },
    MacAppleScript { sender_bundle_id: Option<String> },
    LinuxDBus { bus_name: String },
    AndroidIntent { action: String, calling_package: Option<String> },
    IosAppIntent { intent_name: String },
    WebPostMessage { origin: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeRiskLevel {
    ReadOnly,
    LowMutation,
    HighMutation,
    Destructive,
    ExternalDisclosure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeWriteMode {
    IncrementalAppend,
    FullRewritePreserveMetadata,
    FullSanitizingRewrite,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeOperationKind {
    OpenDocument,
    ReadPageText,
    RenderTile,
    AddAnnotation,
    OrganisePages,
    EditMetadata,
    ScrubMetadata,
    PlanRedaction,
    ApplyRedaction,
    ApplyWorkflow,
    ConvertDocument,
    ExportDocument,
    PrintDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeOperationIntent {
    pub intent_id: String,
    pub source: FeOperationSource,
    pub document_id: Option<String>,
    pub document_sha256: Option<String>,
    pub operation: FeOperationKind,
    pub risk_level: FeRiskLevel,
    pub requires_user_review: bool,
    pub created_at_utc: String,
    pub payload_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FePatchPlan {
    pub patch_plan_id: String,
    pub intent_id: String,
    pub document_sha256_before: String,
    pub write_mode: FeWriteMode,
    pub summary: Vec<String>,
    pub warnings: Vec<String>,
    pub operations: Vec<FePatchOperation>,
    pub transformation_graph_id: Option<String>,
    pub transformation_passes: Vec<String>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FePatchOperation {
    pub op_id: String,
    pub kind: String,
    pub page_index: Option<u32>,
    pub bbox: Option<FeRect>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub coordinate_space: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeApprovalToken {
    pub token_id: String,
    pub patch_plan_id: String,
    pub document_sha256_before: String,
    pub policy_allow_rule: String,
    pub approved_by: String,
    pub approved_at_utc: String,
    pub approval_context: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeVerificationReport {
    pub verification_id: String,
    pub checks: Vec<FeVerificationCheck>,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeVerificationCheck {
    pub check_id: String,
    pub name: String,
    pub passed: bool,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeAuditReceipt {
    pub receipt_id: String,
    pub intent_id: String,
    pub patch_plan_id: String,
    pub document_sha256_before: String,
    pub document_sha256_after: String,
    pub verification: FeVerificationReport,
    pub created_at_utc: String,
}
