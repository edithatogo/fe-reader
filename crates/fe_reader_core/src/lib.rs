//! Fe Reader core contracts.
//!
//! Wave 0 rule: this crate owns stable document-operation contracts only. It must not depend on
//! Tauri, PDFium, GPU crates, OS APIs, MCP, UniFFI, plugin runtimes, or ML libraries.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
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

/// Stable audit receipt id emitted after plan, verify, or failure checkpoints.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReceiptId(pub String);

impl ReceiptId {
    /// Creates a fresh receipt id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for ReceiptId {
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

impl WriteMode {
    fn precedence(self) -> u8 {
        match self {
            Self::NoWrite => 0,
            Self::IncrementalAppend => 1,
            Self::FullRewrite => 2,
            Self::SanitizingRewrite => 3,
        }
    }

    fn strongest(left: Self, right: Self) -> Self {
        if left.precedence() >= right.precedence() {
            left
        } else {
            right
        }
    }
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
    /// Plan-only page deletion.
    DeletePages {
        /// Zero-based page indexes to delete.
        page_indexes: Vec<u32>,
    },
    /// Plan-only page rotation.
    RotatePages {
        /// Zero-based page indexes to rotate.
        page_indexes: Vec<u32>,
        /// Clockwise rotation in degrees. Must be 0, 90, 180, or 270.
        rotation_degrees: u16,
    },
    /// Plan-only page reordering.
    ReorderPages {
        /// Complete new zero-based page ordering.
        new_order: Vec<u32>,
    },
}

impl PatchOperation {
    /// Returns true if this operation mutates document bytes.
    #[must_use]
    pub fn mutates_document(&self) -> bool {
        !matches!(self, Self::Noop)
    }

    /// Returns the minimum write mode required by this operation.
    #[must_use]
    pub fn required_write_mode(&self) -> WriteMode {
        match self {
            Self::Noop => WriteMode::NoWrite,
            Self::SetMetadata { key, .. } if key == "metadata_scrub_mode" => {
                WriteMode::SanitizingRewrite
            }
            Self::SetMetadata { .. } => WriteMode::IncrementalAppend,
            Self::DeletePages { .. } | Self::RotatePages { .. } | Self::ReorderPages { .. } => {
                WriteMode::FullRewrite
            }
            Self::RedactRegion { .. } => WriteMode::SanitizingRewrite,
            Self::PlaceStamp { .. } => WriteMode::IncrementalAppend,
        }
    }

    /// Creates a page deletion operation.
    ///
    /// # Errors
    ///
    /// Returns an error when no page indexes are supplied.
    pub fn delete_pages(page_indexes: Vec<u32>) -> Result<Self, FeError> {
        if page_indexes.is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "delete_pages requires at least one page index",
            ));
        }
        Ok(Self::DeletePages { page_indexes })
    }

    /// Creates a page rotation operation.
    ///
    /// # Errors
    ///
    /// Returns an error when no page indexes are supplied or the rotation is not a right angle.
    pub fn rotate_pages(page_indexes: Vec<u32>, rotation_degrees: u16) -> Result<Self, FeError> {
        if page_indexes.is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "rotate_pages requires at least one page index",
            ));
        }
        if !matches!(rotation_degrees, 0 | 90 | 180 | 270) {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "page rotation must be 0, 90, 180, or 270 degrees",
            ));
        }
        Ok(Self::RotatePages {
            page_indexes,
            rotation_degrees,
        })
    }

    /// Creates a page reorder operation.
    ///
    /// # Errors
    ///
    /// Returns an error when the new order is empty.
    pub fn reorder_pages(new_order: Vec<u32>) -> Result<Self, FeError> {
        if new_order.is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "reorder_pages requires at least one page index",
            ));
        }
        Ok(Self::ReorderPages { new_order })
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
    /// Creates an operation intent with an explicit risk classification.
    #[must_use]
    pub fn new(
        source: OperationSource,
        document_id: DocumentId,
        kind: OperationKind,
        label: impl Into<String>,
        risk_level: RiskLevel,
    ) -> Self {
        Self {
            intent_id: OperationId::new(),
            source,
            document_id,
            document_fingerprint: None,
            kind,
            label: label.into(),
            risk_level,
            requires_review: risk_level.normally_requires_review(),
        }
    }

    /// Creates a read-only operation intent.
    #[must_use]
    pub fn read_only(
        source: OperationSource,
        document_id: DocumentId,
        label: impl Into<String>,
    ) -> Self {
        Self::new(
            source,
            document_id,
            OperationKind::Inspect,
            label,
            RiskLevel::ReadOnly,
        )
    }

    /// Creates a mutating intent that must be reviewed unless policy explicitly says otherwise.
    #[must_use]
    pub fn mutation(
        source: OperationSource,
        document_id: DocumentId,
        kind: OperationKind,
        label: impl Into<String>,
    ) -> Self {
        Self::new(
            source,
            document_id,
            kind,
            label,
            RiskLevel::DocumentMutation,
        )
    }

    /// Creates a high-risk intent, such as redaction, signing, destructive rewrite, or export.
    #[must_use]
    pub fn high_risk(
        source: OperationSource,
        document_id: DocumentId,
        kind: OperationKind,
        label: impl Into<String>,
    ) -> Self {
        Self::new(source, document_id, kind, label, RiskLevel::HighRisk)
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
        let write_mode = operations
            .iter()
            .map(PatchOperation::required_write_mode)
            .fold(WriteMode::NoWrite, WriteMode::strongest);
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

/// Schema-compatible operation transaction phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionPhase {
    /// Intent received and policy classified.
    IntentReceived,
    /// Patch plan generated but not applied.
    PlanGenerated,
    /// User or managed policy approved apply.
    Approved,
    /// Mutation is being written to a temporary output.
    Applying,
    /// Output verification is running.
    Verifying,
    /// Transaction completed successfully.
    Committed,
    /// Transaction aborted before commit.
    Aborted,
    /// Crash recovery must inspect and repair state.
    RecoveryRequired,
}

impl TransactionPhase {
    /// Returns true when this phase requires a patch plan id.
    #[must_use]
    pub fn requires_plan_id(self) -> bool {
        matches!(
            self,
            Self::PlanGenerated
                | Self::Approved
                | Self::Applying
                | Self::Verifying
                | Self::Committed
        )
    }

    /// Returns true when crash recovery should inspect the transaction.
    #[must_use]
    pub fn requires_recovery(self) -> bool {
        matches!(
            self,
            Self::Applying | Self::Verifying | Self::RecoveryRequired
        )
    }
}

/// Schema-compatible journal entry for one transaction checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournalEntry {
    /// Transaction id.
    pub transaction_id: String,
    /// Operation intent id.
    pub intent_id: String,
    /// Patch plan id when a plan exists.
    pub plan_id: Option<String>,
    /// Current phase.
    pub phase: TransactionPhase,
    /// Monotonic sequence number within the sidecar.
    pub sequence: u64,
    /// Human-readable note for recovery diagnostics.
    pub note: String,
}

impl TransactionJournalEntry {
    /// Creates an intent-received entry.
    #[must_use]
    pub fn intent_received(
        transaction_id: &TransactionId,
        intent: &OperationIntent,
        note: impl Into<String>,
    ) -> Self {
        Self {
            transaction_id: transaction_id.0.clone(),
            intent_id: intent.intent_id.0.clone(),
            plan_id: None,
            phase: TransactionPhase::IntentReceived,
            sequence: 0,
            note: note.into(),
        }
    }

    /// Creates a plan-linked entry.
    #[must_use]
    pub fn for_plan(
        transaction_id: &TransactionId,
        intent: &OperationIntent,
        plan: &PatchPlan,
        phase: TransactionPhase,
        sequence: u64,
        note: impl Into<String>,
    ) -> Self {
        Self {
            transaction_id: transaction_id.0.clone(),
            intent_id: intent.intent_id.0.clone(),
            plan_id: Some(plan.plan_id.0.clone()),
            phase,
            sequence,
            note: note.into(),
        }
    }

    /// Validates schema-level invariants that Rust's type system cannot express.
    ///
    /// # Errors
    ///
    /// Returns an error when plan id presence or sequencing is invalid.
    pub fn validate(&self) -> Result<(), FeError> {
        if self.transaction_id.is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "transaction_id must not be empty",
            ));
        }
        if self.intent_id.is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "intent_id must not be empty",
            ));
        }
        if self.phase.requires_plan_id()
            && self
                .plan_id
                .as_ref()
                .is_none_or(|plan_id| plan_id.is_empty())
        {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "phase requires non-empty plan_id",
            ));
        }
        Ok(())
    }
}

/// Durable transaction sidecar containing ordered journal entries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournalSidecar {
    /// Ordered journal entries.
    pub entries: Vec<TransactionJournalEntry>,
}

impl TransactionJournalSidecar {
    /// Creates an empty sidecar.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Appends a journal entry after validating sequence order.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry is invalid or not the next sequence.
    pub fn append(&mut self, entry: TransactionJournalEntry) -> Result<(), FeError> {
        entry.validate()?;
        let expected = self.entries.len() as u64;
        if entry.sequence != expected {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                format!(
                    "transaction journal sequence must be {expected}, got {}",
                    entry.sequence
                ),
            ));
        }
        if let Some(first) = self.entries.first()
            && (entry.transaction_id != first.transaction_id || entry.intent_id != first.intent_id)
        {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "transaction journal entry does not match sidecar identity",
            ));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Returns the latest entry, if any.
    #[must_use]
    pub fn latest(&self) -> Option<&TransactionJournalEntry> {
        self.entries.last()
    }

    /// Returns true when the latest phase requires recovery inspection.
    #[must_use]
    pub fn recovery_required(&self) -> bool {
        self.latest()
            .is_some_and(|entry| entry.phase.requires_recovery())
    }

    /// Validates all entries in order.
    ///
    /// # Errors
    ///
    /// Returns an error when any entry is invalid or sequence order is broken.
    pub fn validate(&self) -> Result<(), FeError> {
        let mut rebuilt = Self::new();
        for entry in &self.entries {
            rebuilt.append(entry.clone())?;
        }
        Ok(())
    }
}

/// Writes a transaction sidecar as pretty JSON using temp-file then rename.
///
/// # Errors
///
/// Returns an error when validation, serialization, or filesystem persistence fails.
pub fn write_transaction_sidecar(
    path: impl AsRef<Path>,
    sidecar: &TransactionJournalSidecar,
) -> Result<(), FeError> {
    sidecar.validate()?;
    let path = path.as_ref();
    let tmp_path = path.with_extension("tmp");
    let json = serde_json::to_vec_pretty(sidecar)
        .map_err(|error| FeError::new(FeErrorKind::Internal, error.to_string()))?;
    fs::write(&tmp_path, json).map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    fs::rename(&tmp_path, path).map_err(|error| {
        let _ = fs::remove_file(&tmp_path);
        FeError::new(FeErrorKind::Io, error.to_string())
    })?;
    Ok(())
}

/// Reads and validates a transaction sidecar from JSON.
///
/// # Errors
///
/// Returns an error when the sidecar cannot be read, parsed, or validated.
pub fn read_transaction_sidecar(
    path: impl AsRef<Path>,
) -> Result<TransactionJournalSidecar, FeError> {
    let bytes = fs::read(path).map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    let sidecar: TransactionJournalSidecar = serde_json::from_slice(&bytes)
        .map_err(|error| FeError::new(FeErrorKind::InvalidInput, error.to_string()))?;
    sidecar.validate()?;
    Ok(sidecar)
}

/// Lists readable transaction sidecars in a directory.
///
/// # Errors
///
/// Returns an error when the directory cannot be read.
pub fn list_transaction_sidecars(
    dir: impl AsRef<Path>,
) -> Result<Vec<TransactionJournalSidecar>, FeError> {
    let mut sidecars = Vec::new();
    let entries =
        fs::read_dir(dir).map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
        if entry
            .file_type()
            .map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?
            .is_file()
            && entry.path().extension().is_some_and(|ext| ext == "json")
        {
            sidecars.push(read_transaction_sidecar(entry.path())?);
        }
    }
    Ok(sidecars)
}

/// Returns whether a transaction sidecar path exists.
///
/// # Errors
///
/// Returns an error for filesystem failures other than not-found.
pub fn transaction_sidecar_exists(path: impl AsRef<Path>) -> Result<bool, FeError> {
    match fs::metadata(path.as_ref()) {
        Ok(metadata) => Ok(metadata.is_file()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(FeError::new(FeErrorKind::Io, error.to_string())),
    }
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

/// Receipt generated after an operation plan, verification step, or failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationReceipt {
    /// Unique audit receipt id.
    pub receipt_id: ReceiptId,
    /// Operation id.
    pub intent_id: OperationId,
    /// Optional patch plan id.
    pub plan_id: Option<PatchPlanId>,
    /// Optional transaction id.
    pub transaction_id: Option<TransactionId>,
    /// Target document.
    pub document_id: DocumentId,
    /// Write mode captured from the plan, if a plan exists.
    pub write_mode: WriteMode,
    /// Risk classification captured from the intent or plan.
    pub risk_level: RiskLevel,
    /// Document fingerprint captured before planning or apply.
    pub document_fingerprint_before: Option<DocumentFingerprint>,
    /// Document fingerprint captured after apply or verification.
    pub document_fingerprint_after: Option<DocumentFingerprint>,
    /// Verification status.
    pub verification_status: VerificationStatus,
    /// UTC timestamp when the receipt was created.
    pub created_at_utc: String,
    /// Human-readable summary.
    pub summary: String,
}

impl OperationReceipt {
    /// Creates a plan-stage audit receipt without implying approval or application.
    #[must_use]
    pub fn planned(intent: &OperationIntent, plan: &PatchPlan, summary: impl Into<String>) -> Self {
        Self {
            receipt_id: ReceiptId::new(),
            intent_id: intent.intent_id.clone(),
            plan_id: Some(plan.plan_id.clone()),
            transaction_id: None,
            document_id: plan.document_id.clone(),
            write_mode: plan.write_mode,
            risk_level: plan.risk_level,
            document_fingerprint_before: intent.document_fingerprint.clone(),
            document_fingerprint_after: None,
            verification_status: VerificationStatus::NotRequired,
            created_at_utc: utc_now_rfc3339(),
            summary: summary.into(),
        }
    }

    /// Creates a post-apply audit receipt linked to a transaction journal.
    #[must_use]
    pub fn verified(
        intent: &OperationIntent,
        plan: &PatchPlan,
        journal: &TransactionJournal,
        verification_status: VerificationStatus,
        document_fingerprint_after: DocumentFingerprint,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            receipt_id: ReceiptId::new(),
            intent_id: intent.intent_id.clone(),
            plan_id: Some(plan.plan_id.clone()),
            transaction_id: Some(journal.transaction_id.clone()),
            document_id: plan.document_id.clone(),
            write_mode: plan.write_mode,
            risk_level: plan.risk_level,
            document_fingerprint_before: intent.document_fingerprint.clone(),
            document_fingerprint_after: Some(document_fingerprint_after),
            verification_status,
            created_at_utc: utc_now_rfc3339(),
            summary: summary.into(),
        }
    }
}

/// Audit receipt alias used by the mutation pipeline contract.
pub type AuditReceipt = OperationReceipt;

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

fn utc_now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
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
    fn explicit_risk_constructor_uses_review_defaults() {
        let local_state = OperationIntent::new(
            OperationSource::Ui,
            DocumentId::new(),
            OperationKind::Custom("select_sidebar_item".to_string()),
            "select_sidebar_item",
            RiskLevel::LocalState,
        );
        let high_risk = OperationIntent::new(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::Export,
            "export_document",
            RiskLevel::HighRisk,
        );

        assert_eq!(local_state.risk_level, RiskLevel::LocalState);
        assert!(!local_state.requires_review);
        assert_eq!(high_risk.risk_level, RiskLevel::HighRisk);
        assert!(high_risk.requires_review);
    }

    #[test]
    fn high_risk_intent_requires_review() {
        let intent = OperationIntent::high_risk(
            OperationSource::Automation,
            DocumentId::new(),
            OperationKind::ApplyPatch,
            "apply_redaction",
        );

        assert_eq!(intent.risk_level, RiskLevel::HighRisk);
        assert!(intent.requires_review);
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
        assert_eq!(plan.write_mode, WriteMode::IncrementalAppend);
        assert_eq!(plan.risk_level, RiskLevel::DocumentMutation);
    }

    #[test]
    fn noop_patch_plan_is_no_write_and_unapproved() {
        let intent = OperationIntent::read_only(OperationSource::Cli, DocumentId::new(), "inspect");
        let plan = PatchPlan::draft(&intent, "inspect", vec![PatchOperation::Noop]);

        assert_eq!(plan.write_mode, WriteMode::NoWrite);
        assert_eq!(plan.risk_level, RiskLevel::ReadOnly);
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.operations, vec![PatchOperation::Noop]);
    }

    #[test]
    fn mutating_operation_raises_read_only_plan_risk() {
        let intent = OperationIntent::read_only(
            OperationSource::Cli,
            DocumentId::new(),
            "bad_metadata_plan",
        );
        let plan = PatchPlan::draft(
            &intent,
            "set metadata",
            vec![PatchOperation::SetMetadata {
                key: "title".into(),
                value: "x".into(),
            }],
        );

        assert_eq!(plan.write_mode, WriteMode::IncrementalAppend);
        assert_eq!(plan.risk_level, RiskLevel::DocumentMutation);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn high_risk_redaction_plan_preserves_high_risk() {
        let intent = OperationIntent::high_risk(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "plan_redaction",
        );
        let plan = PatchPlan::draft(
            &intent,
            "redact region",
            vec![PatchOperation::RedactRegion {
                page_index: 0,
                region: "10,10,20,20".into(),
            }],
        );

        assert_eq!(plan.write_mode, WriteMode::SanitizingRewrite);
        assert_eq!(plan.risk_level, RiskLevel::HighRisk);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn write_mode_policy_maps_operations_to_minimum_safe_mode() {
        assert_eq!(
            PatchOperation::Noop.required_write_mode(),
            WriteMode::NoWrite
        );
        assert_eq!(
            PatchOperation::PlaceStamp {
                page_index: 0,
                stamp_ref: "sig".into()
            }
            .required_write_mode(),
            WriteMode::IncrementalAppend
        );
        assert_eq!(
            PatchOperation::SetMetadata {
                key: "title".into(),
                value: "Fe".into()
            }
            .required_write_mode(),
            WriteMode::IncrementalAppend
        );
        assert_eq!(
            PatchOperation::SetMetadata {
                key: "metadata_scrub_mode".into(),
                value: "Aggressive".into()
            }
            .required_write_mode(),
            WriteMode::SanitizingRewrite
        );
        assert_eq!(
            PatchOperation::RedactRegion {
                page_index: 0,
                region: "10,10,20,20".into()
            }
            .required_write_mode(),
            WriteMode::SanitizingRewrite
        );
    }

    #[test]
    fn mixed_operations_use_strongest_write_mode() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "stamp_and_redact",
        );
        let plan = PatchPlan::draft(
            &intent,
            "stamp and redact",
            vec![
                PatchOperation::PlaceStamp {
                    page_index: 0,
                    stamp_ref: "approved".into(),
                },
                PatchOperation::RedactRegion {
                    page_index: 0,
                    region: "10,10,20,20".into(),
                },
            ],
        );

        assert_eq!(plan.write_mode, WriteMode::SanitizingRewrite);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn page_operations_validate_before_planning() {
        assert_eq!(
            PatchOperation::rotate_pages(vec![2], 90).unwrap(),
            PatchOperation::RotatePages {
                page_indexes: vec![2],
                rotation_degrees: 90
            }
        );
        assert_eq!(
            PatchOperation::delete_pages(vec![1]).unwrap(),
            PatchOperation::DeletePages {
                page_indexes: vec![1]
            }
        );
        assert!(PatchOperation::rotate_pages(vec![0], 45).is_err());
        assert!(PatchOperation::delete_pages(Vec::new()).is_err());
        assert!(PatchOperation::reorder_pages(Vec::new()).is_err());
    }

    #[test]
    fn page_operation_patch_plan_is_mutating_and_unapproved() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "rotate_page",
        );
        let plan = PatchPlan::draft(
            &intent,
            "rotate page 1",
            vec![PatchOperation::rotate_pages(vec![1], 90).unwrap()],
        );

        assert_eq!(plan.write_mode, WriteMode::FullRewrite);
        assert_eq!(plan.risk_level, RiskLevel::DocumentMutation);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn page_operation_raises_read_only_intent_risk() {
        let intent = OperationIntent::read_only(OperationSource::Cli, DocumentId::new(), "bad");
        let plan = PatchPlan::draft(
            &intent,
            "delete pages",
            vec![PatchOperation::delete_pages(vec![0]).unwrap()],
        );

        assert_eq!(plan.write_mode, WriteMode::FullRewrite);
        assert_eq!(plan.risk_level, RiskLevel::DocumentMutation);
        assert!(!plan.approved_for_apply);
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

    #[test]
    fn transaction_sidecar_persists_schema_compatible_entries() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::ApplyPatch,
            "apply_patch",
        );
        let plan = PatchPlan::draft(&intent, "noop", vec![PatchOperation::Noop]);
        let transaction_id = TransactionId::new();
        let mut sidecar = TransactionJournalSidecar::new();

        sidecar
            .append(TransactionJournalEntry::intent_received(
                &transaction_id,
                &intent,
                "intent received",
            ))
            .unwrap();
        sidecar
            .append(TransactionJournalEntry::for_plan(
                &transaction_id,
                &intent,
                &plan,
                TransactionPhase::PlanGenerated,
                1,
                "plan generated",
            ))
            .unwrap();

        let path = unique_temp_path("transaction-sidecar.json");
        write_transaction_sidecar(&path, &sidecar).unwrap();
        let loaded = read_transaction_sidecar(&path).unwrap();
        let _ = fs::remove_file(&path);

        assert_eq!(loaded, sidecar);
        assert!(!loaded.recovery_required());
    }

    #[test]
    fn transaction_sidecar_recovery_required_for_incomplete_apply() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::ApplyPatch,
            "apply_patch",
        );
        let plan = PatchPlan::draft(&intent, "noop", vec![PatchOperation::Noop]);
        let transaction_id = TransactionId::new();
        let mut sidecar = TransactionJournalSidecar::new();

        sidecar
            .append(TransactionJournalEntry::intent_received(
                &transaction_id,
                &intent,
                "intent received",
            ))
            .unwrap();
        sidecar
            .append(TransactionJournalEntry::for_plan(
                &transaction_id,
                &intent,
                &plan,
                TransactionPhase::Applying,
                1,
                "writing output",
            ))
            .unwrap();

        assert!(sidecar.recovery_required());
    }

    #[test]
    fn transaction_sidecar_rejects_bad_sequence_and_missing_plan() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::ApplyPatch,
            "apply_patch",
        );
        let transaction_id = TransactionId::new();
        let mut sidecar = TransactionJournalSidecar::new();
        let bad_sequence = TransactionJournalEntry {
            transaction_id: transaction_id.0.clone(),
            intent_id: intent.intent_id.0.clone(),
            plan_id: None,
            phase: TransactionPhase::IntentReceived,
            sequence: 2,
            note: "bad sequence".to_string(),
        };

        assert_eq!(
            sidecar.append(bad_sequence).unwrap_err().kind,
            FeErrorKind::InvalidInput
        );

        let missing_plan = TransactionJournalEntry {
            transaction_id: transaction_id.0,
            intent_id: intent.intent_id.0,
            plan_id: None,
            phase: TransactionPhase::Applying,
            sequence: 0,
            note: "missing plan".to_string(),
        };

        assert_eq!(
            missing_plan.validate().unwrap_err().kind,
            FeErrorKind::InvalidInput
        );
    }

    #[test]
    fn malformed_transaction_sidecar_fails_closed() {
        let path = unique_temp_path("malformed-transaction-sidecar.json");
        fs::write(&path, b"{not json").unwrap();
        let error = read_transaction_sidecar(&path).unwrap_err();
        let _ = fs::remove_file(&path);

        assert_eq!(error.kind, FeErrorKind::InvalidInput);
    }

    #[test]
    fn planned_receipt_links_intent_plan_and_before_fingerprint_without_apply() {
        let fingerprint_before = DocumentFingerprint::from_bytes(b"before");
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "set_metadata",
        )
        .with_document_fingerprint(fingerprint_before.clone());
        let plan = PatchPlan::draft(
            &intent,
            "set metadata",
            vec![PatchOperation::SetMetadata {
                key: "title".into(),
                value: "Fe Reader".into(),
            }],
        );
        let receipt = OperationReceipt::planned(&intent, &plan, "plan generated");

        assert_eq!(receipt.intent_id, intent.intent_id);
        assert_eq!(receipt.plan_id, Some(plan.plan_id.clone()));
        assert_eq!(receipt.transaction_id, None);
        assert_eq!(receipt.document_id, intent.document_id);
        assert_eq!(receipt.write_mode, WriteMode::IncrementalAppend);
        assert_eq!(receipt.risk_level, RiskLevel::DocumentMutation);
        assert_eq!(
            receipt.document_fingerprint_before,
            Some(fingerprint_before)
        );
        assert_eq!(receipt.document_fingerprint_after, None);
        assert_eq!(receipt.verification_status, VerificationStatus::NotRequired);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn verified_receipt_links_transaction_and_after_fingerprint() {
        let fingerprint_before = DocumentFingerprint::from_bytes(b"before");
        let fingerprint_after = DocumentFingerprint::from_bytes(b"after");
        let intent = OperationIntent::high_risk(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "plan_redaction",
        )
        .with_document_fingerprint(fingerprint_before.clone());
        let plan = PatchPlan::draft(
            &intent,
            "redact region",
            vec![PatchOperation::RedactRegion {
                page_index: 0,
                region: "10,10,20,20".into(),
            }],
        );
        let journal =
            TransactionJournal::planned(&plan).transition(TransactionState::Verified, "verified");
        let receipt = OperationReceipt::verified(
            &intent,
            &plan,
            &journal,
            VerificationStatus::Passed,
            fingerprint_after.clone(),
            "verification passed",
        );

        assert_eq!(receipt.intent_id, intent.intent_id);
        assert_eq!(receipt.plan_id, Some(plan.plan_id.clone()));
        assert_eq!(receipt.transaction_id, Some(journal.transaction_id));
        assert_eq!(receipt.write_mode, WriteMode::SanitizingRewrite);
        assert_eq!(receipt.risk_level, RiskLevel::HighRisk);
        assert_eq!(
            receipt.document_fingerprint_before,
            Some(fingerprint_before)
        );
        assert_eq!(receipt.document_fingerprint_after, Some(fingerprint_after));
        assert_eq!(receipt.verification_status, VerificationStatus::Passed);
    }

    #[test]
    fn failed_receipt_preserves_verification_failure() {
        let fingerprint_after = DocumentFingerprint::from_bytes(b"failed-output");
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::ApplyPatch,
            "apply_patch",
        )
        .with_document_fingerprint(DocumentFingerprint::from_bytes(b"before"));
        let plan = PatchPlan::draft(
            &intent,
            "rotate page",
            vec![PatchOperation::rotate_pages(vec![0], 90).unwrap()],
        );
        let journal = TransactionJournal::planned(&plan)
            .transition(TransactionState::Failed, "verify failed");
        let receipt = OperationReceipt::verified(
            &intent,
            &plan,
            &journal,
            VerificationStatus::Failed,
            fingerprint_after,
            "verification failed",
        );

        assert_eq!(receipt.transaction_id, Some(journal.transaction_id));
        assert_eq!(receipt.write_mode, WriteMode::FullRewrite);
        assert_eq!(receipt.risk_level, RiskLevel::DocumentMutation);
        assert_eq!(receipt.verification_status, VerificationStatus::Failed);
    }

    fn unique_temp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("fe-reader-{}-{name}", Uuid::new_v4()))
    }
}
