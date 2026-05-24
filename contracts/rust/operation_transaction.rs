//! Operation transaction contract.
//!
//! Every mutating surface must flow through Intent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt.

use serde::{Deserialize, Serialize};

/// Transaction phase.
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

/// Minimal journal entry; concrete crates can add typed payloads while preserving this envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournalEntry {
    /// Transaction id.
    pub transaction_id: String,
    /// Operation intent id.
    pub intent_id: String,
    /// Patch plan id.
    pub plan_id: Option<String>,
    /// Current phase.
    pub phase: TransactionPhase,
    /// Monotonic sequence number within the journal.
    pub sequence: u64,
    /// Human-readable note for recovery diagnostics.
    pub note: String,
}
