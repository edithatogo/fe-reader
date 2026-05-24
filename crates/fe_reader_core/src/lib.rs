//! Fe Reader core contracts.
//!
//! Wave 0 rule: this crate owns stable document-operation contracts only. It must not depend on
//! Tauri, PDFium, GPU crates, OS APIs, MCP, UniFFI, plugin runtimes, or ML libraries.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use uuid::Uuid;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stable document identifier used by UI, CLI, MCP, plugins, and platform adapters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

impl DocumentId {
    /// Creates a fresh local document id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable operation identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OperationId(pub String);

impl OperationId {
    /// Creates a fresh operation id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for OperationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable patch plan identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatchPlanId(pub String);

impl PatchPlanId {
    /// Creates a fresh patch plan id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for PatchPlanId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable transaction id for crash-safe apply flows.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub String);

impl TransactionId {
    /// Creates a fresh transaction id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::new()
    }
}

/// SHA-256 and size fingerprint for a document or output artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentFingerprint {
    /// Lower-case hex SHA-256 digest.
    pub sha256_hex: String,
    /// Byte length of the file or buffer.
    pub byte_len: u64,
}

impl DocumentFingerprint {
    /// Computes a fingerprint from bytes.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            sha256_hex: sha256_hex(bytes),
            byte_len: bytes.len() as u64,
        }
    }
}

/// Surface that requested an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationSource {
    /// Human-driven UI request.
    Ui,
    /// Command line request.
    Cli,
    /// Model Context Protocol tool request.
    Mcp,
    /// Native automation surface such as COM, AppleScript, D-Bus, Android intents, or App Intents.
    Automation,
    /// Browser extension or web postMessage request.
    Web,
    /// WASM plugin proposal.
    Plugin,
}

/// Mutability/risk class for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// Inspection only.
    ReadOnly,
    /// Reversible local state change, not a PDF mutation.
    LocalState,
    /// PDF mutation that should be reviewable before apply.
    DocumentMutation,
    /// High-risk mutation such as redaction, signing, destructive rewrite, or external export.
    HighRisk,
}

impl RiskLevel {
    /// Returns true when policy should normally require explicit human review.
    #[must_use]
    pub fn normally_requires_review(self) -> bool {
        matches!(self, Self::DocumentMutation | Self::HighRisk)
    }
}

/// Intent kind, still intentionally coarse in Wave 0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    /// Inspect or read document state.
    Inspect,
    /// Render a page, tile, or thumbnail.
    Render,
    /// Search deterministic text/index content.
    Search,
    /// Plan a document mutation without applying it.
    PlanMutation,
    /// Apply an approved patch plan.
    ApplyPatch,
    /// Export, convert, share, print, or otherwise move bytes outside the current document.
    Export,
    /// Execute a plugin proposal.
    PluginProposal,
    /// Execute an external authoring/conversion/preflight tool.
    ExternalTool,
    /// A named operation not yet represented by a stable enum case.
    Custom(String),
}

/// Persistence strategy for document writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    /// No document bytes are written.
    NoWrite,
    /// Append-only revision. Useful for some stamps/receipts, not secure redaction.
    IncrementalAppend,
    /// Full rewrite preserving reachable content as much as possible.
    FullRewrite,
    /// Full rewrite intended to remove old revisions/unreachable sensitive content.
    SanitizingRewrite,
}

/// One planned operation inside a patch plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOperation {
    /// No-op used by tests and read-only planning.
    Noop,
    /// Placeholder for metadata mutation planning.
    SetMetadata {
        /// Metadata key.
        key: String,
        /// Metadata value.
        value: String,
    },
    /// Placeholder for a redaction operation. Geometry is crate-specific until Wave 1.
    RedactRegion {
        /// Page index, zero-based.
        page_index: u32,
        /// Region label or opaque bbox string.
        region: String,
    },
    /// Placeholder for a stamp/signature-like operation.
    PlaceStamp {
        /// Page index, zero-based.
        page_index: u32,
        /// Stamp template id or opaque asset reference.
        stamp_ref: String,
    },
}

impl PatchOperation {
    /// Returns true if this operation mutates document bytes.
    #[must_use]
    pub fn mutates_document(&self) -> bool {
        !matches!(self, Self::Noop)
    }
}

/// First-class request object for every operation surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationIntent {
    /// Unique operation id.
    pub intent_id: OperationId,
    /// Calling surface.
    pub source: OperationSource,
    /// Target document.
    pub document_id: DocumentId,
    /// Optional hash of the document at the time the intent was created.
    pub document_fingerprint: Option<DocumentFingerprint>,
    /// Operation kind.
    pub kind: OperationKind,
    /// Human-readable operation label, e.g. `inspect`, `plan_redaction`, `apply_patch`.
    pub label: String,
    /// Risk classification decided before planning.
    pub risk_level: RiskLevel,
    /// Whether policy requires explicit review before apply.
    pub requires_review: bool,
}

impl OperationIntent {
    /// Creates a read-only operation intent.
    #[must_use]
    pub fn read_only(
        source: OperationSource,
        document_id: DocumentId,
        label: impl Into<String>,
    ) -> Self {
        Self {
            intent_id: OperationId::new(),
            source,
            document_id,
            document_fingerprint: None,
            kind: OperationKind::Inspect,
            label: label.into(),
            risk_level: RiskLevel::ReadOnly,
            requires_review: false,
        }
    }

    /// Creates a mutating intent that must be reviewed unless policy explicitly says otherwise.
    #[must_use]
    pub fn mutation(
        source: OperationSource,
        document_id: DocumentId,
        kind: OperationKind,
        label: impl Into<String>,
    ) -> Self {
        Self {
            intent_id: OperationId::new(),
            source,
            document_id,
            document_fingerprint: None,
            kind,
            label: label.into(),
            risk_level: RiskLevel::DocumentMutation,
            requires_review: true,
        }
    }

    /// Attaches the document fingerprint captured at intent creation time.
    #[must_use]
    pub fn with_document_fingerprint(mut self, fingerprint: DocumentFingerprint) -> Self {
        self.document_fingerprint = Some(fingerprint);
        self
    }
}

/// Immutable patch-plan draft or approved plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchPlan {
    /// Unique patch plan id.
    pub plan_id: PatchPlanId,
    /// Intent that produced this plan.
    pub intent_id: OperationId,
    /// Target document.
    pub document_id: DocumentId,
    /// Human-readable summary.
    pub summary: String,
    /// Planned operations.
    pub operations: Vec<PatchOperation>,
    /// Required write mode.
    pub write_mode: WriteMode,
    /// Risk classification inherited or raised by the planner.
    pub risk_level: RiskLevel,
    /// Whether the plan may be applied without further review.
    pub approved_for_apply: bool,
}

impl PatchPlan {
    /// Creates a non-approved plan draft.
    #[must_use]
    pub fn draft(
        intent: &OperationIntent,
        summary: impl Into<String>,
        operations: Vec<PatchOperation>,
    ) -> Self {
        let mut risk_level = intent.risk_level;
        if operations.iter().any(PatchOperation::mutates_document)
            && risk_level == RiskLevel::ReadOnly
        {
            risk_level = RiskLevel::DocumentMutation;
        }
        let write_mode = if operations.iter().any(PatchOperation::mutates_document) {
            WriteMode::FullRewrite
        } else {
            WriteMode::NoWrite
        };
        Self {
            plan_id: PatchPlanId::new(),
            intent_id: intent.intent_id.clone(),
            document_id: intent.document_id.clone(),
            summary: summary.into(),
            operations,
            write_mode,
            risk_level,
            approved_for_apply: false,
        }
    }

    /// Marks a plan as approved. The caller must already have passed policy review.
    #[must_use]
    pub fn approved(mut self) -> Self {
        self.approved_for_apply = true;
        self
    }
}

/// Durable transaction state used for crash recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionState {
    /// Created but not yet written.
    Planned,
    /// Journal has been persisted.
    Journaled,
    /// Apply is in progress.
    Applying,
    /// Apply completed and verification may proceed.
    Applied,
    /// Verification passed.
    Verified,
    /// Operation was rolled back or abandoned.
    RolledBack,
    /// Operation failed.
    Failed,
}

/// Journal entry for one patch plan application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournal {
    /// Transaction id.
    pub transaction_id: TransactionId,
    /// Patch plan id.
    pub plan_id: PatchPlanId,
    /// Document id.
    pub document_id: DocumentId,
    /// State.
    pub state: TransactionState,
    /// Human-readable recovery note.
    pub recovery_note: String,
}

impl TransactionJournal {
    /// Creates a new journal in `planned` state.
    #[must_use]
    pub fn planned(plan: &PatchPlan) -> Self {
        Self {
            transaction_id: TransactionId::new(),
            plan_id: plan.plan_id.clone(),
            document_id: plan.document_id.clone(),
            state: TransactionState::Planned,
            recovery_note: "no document bytes have been modified".to_string(),
        }
    }

    /// Returns a copy with a new state and note.
    #[must_use]
    pub fn transition(mut self, state: TransactionState, note: impl Into<String>) -> Self {
        self.state = state;
        self.recovery_note = note.into();
        self
    }
}

/// Result of post-apply verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// No verification was required.
    NotRequired,
    /// Verification passed.
    Passed,
    /// Verification failed.
    Failed,
}

/// Receipt generated after an operation completes or fails.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationReceipt {
    /// Operation id.
    pub intent_id: OperationId,
    /// Optional patch plan id.
    pub plan_id: Option<PatchPlanId>,
    /// Optional transaction id.
    pub transaction_id: Option<TransactionId>,
    /// Verification status.
    pub verification_status: VerificationStatus,
    /// Human-readable summary.
    pub summary: String,
}

/// Resource limits applied to parsing, rendering, conversion, plugin, or automation tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Max wall-clock milliseconds.
    pub max_wall_time_ms: u64,
    /// Max memory in MiB.
    pub max_memory_mib: u64,
    /// Max number of pages a task may touch without escalation.
    pub max_pages: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_wall_time_ms: 30_000,
            max_memory_mib: 1024,
            max_pages: 500,
        }
    }
}

/// Error taxonomy shared by all crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeErrorKind {
    /// File or filesystem error.
    Io,
    /// PDF syntax or model parse error.
    Parse,
    /// Security policy denied the operation.
    PolicyDenied,
    /// User approval was required or denied.
    ApprovalRequired,
    /// Operation exceeded configured resource limits.
    ResourceLimit,
    /// Operation was cancelled.
    Cancelled,
    /// Verification failed after an operation.
    VerificationFailed,
    /// Input was invalid.
    InvalidInput,
    /// Internal bug or invariant violation.
    Internal,
}

/// Shared error shape for APIs that need structured user-facing failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{kind:?}: {message}")]
pub struct FeError {
    /// Error kind.
    pub kind: FeErrorKind,
    /// Developer-facing message.
    pub message: String,
    /// Optional user-facing message key.
    pub user_message_key: Option<String>,
}

impl FeError {
    /// Creates a new structured error.
    #[must_use]
    pub fn new(kind: FeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            user_message_key: None,
        }
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Computes a lower-case SHA-256 digest.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
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
    fn read_only_intent_does_not_require_review() {
        let intent = OperationIntent::read_only(OperationSource::Cli, DocumentId::new(), "inspect");
        assert_eq!(intent.risk_level, RiskLevel::ReadOnly);
        assert!(!intent.requires_review);
    }

    #[test]
    fn mutating_patch_plan_defaults_to_not_approved() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "set_metadata",
        );
        let plan = PatchPlan::draft(
            &intent,
            "set metadata",
            vec![PatchOperation::SetMetadata {
                key: "title".into(),
                value: "x".into(),
            }],
        );
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.intent_id, intent.intent_id);
        assert_eq!(plan.write_mode, WriteMode::FullRewrite);
        assert_eq!(plan.risk_level, RiskLevel::DocumentMutation);
    }

    #[test]
    fn sha256_is_stable() {
        assert_eq!(
            sha256_hex(b"Fe Reader"),
            "60f341dcf33d7e42a16d0aa7b18a7fb943a06ff75622be05d8d01b4f8b564768"
        );
    }

    #[test]
    fn transaction_transitions_are_copyable() {
        let intent = OperationIntent::read_only(OperationSource::Cli, DocumentId::new(), "inspect");
        let plan = PatchPlan::draft(&intent, "inspect", vec![PatchOperation::Noop]);
        let journal =
            TransactionJournal::planned(&plan).transition(TransactionState::Journaled, "persisted");
        assert_eq!(journal.state, TransactionState::Journaled);
    }
}
