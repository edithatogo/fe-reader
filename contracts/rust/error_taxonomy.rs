//! Stable user-facing and developer-facing error taxonomy.

use serde::{Deserialize, Serialize};

/// High-level error category used for UX, telemetry, support bundles and automation responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// File cannot be opened, read, written or persisted.
    FileAccess,
    /// PDF syntax or compatibility issue.
    PdfCompatibility,
    /// Security policy blocked the request.
    PolicyDenied,
    /// Operation requires user approval.
    ApprovalRequired,
    /// Redaction, signing, verification or sanitisation failed.
    VerificationFailed,
    /// Platform integration failed.
    PlatformIntegration,
    /// Plugin, workflow, or automation integration failed.
    Integration,
    /// User cancelled the operation.
    Cancelled,
    /// Resource budget exceeded.
    ResourceLimit,
    /// Internal bug.
    Internal,
}

/// Error envelope returned across CLI, MCP, local API, plugins and platform adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeErrorEnvelope {
    /// Stable error code, e.g. `fe.policy.approval_required`.
    pub code: String,
    /// Error category.
    pub category: ErrorCategory,
    /// Safe user-facing message; must not leak sensitive document contents.
    pub user_message: String,
    /// Developer diagnostic message; may be redacted in support bundles.
    pub diagnostic: Option<String>,
    /// Whether retry could succeed without changing inputs.
    pub retryable: bool,
}
