//! Platform adapter contracts for Fe Reader.
//!
//! This crate records schema-compatible platform intent outcomes without calling
//! native platform APIs. Wave 1 stubs are no-op/read-only/default-deny.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Supported platform adapter contract targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformTarget {
    /// Windows shell integration surface.
    Windows,
    /// macOS application integration surface.
    Macos,
    /// Linux desktop integration surface.
    Linux,
    /// Android intent integration surface.
    Android,
    /// iOS App Intents integration surface.
    Ios,
}

impl PlatformTarget {
    /// Returns all Wave 1 recent-document smoke targets.
    #[must_use]
    pub const fn wave1_recent_document_targets() -> [Self; 5] {
        [
            Self::Windows,
            Self::Macos,
            Self::Linux,
            Self::Android,
            Self::Ios,
        ]
    }
}

/// Class of platform integration operation requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformOperation {
    /// Register a local document in the host platform's recent-document list.
    RegisterRecentDocument,
}

/// Default policy decision for a platform operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformPolicyDecision {
    /// Operation is observed or described only.
    ReadOnly,
    /// Mutation is denied by default.
    DefaultDenied,
}

/// Required guard names for mutating platform integration surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformMutationGuard {
    /// The caller must prove the document hash matches the planned mutation.
    DocumentHashMatch,
    /// The caller must provide a patch plan identifier.
    PatchPlanId,
    /// The request must pass policy evaluation.
    PolicyEvaluation,
    /// The request must include an approval token or interactive confirmation.
    ApprovalTokenOrInteractiveConfirmation,
    /// The adapter must emit an audit receipt for any approved mutation.
    AuditReceiptEmission,
}

impl PlatformMutationGuard {
    /// Returns the full guard set required before any platform mutation.
    #[must_use]
    pub const fn required_for_mutation() -> [Self; 5] {
        [
            Self::DocumentHashMatch,
            Self::PatchPlanId,
            Self::PolicyEvaluation,
            Self::ApprovalTokenOrInteractiveConfirmation,
            Self::AuditReceiptEmission,
        ]
    }
}

/// Schema-compatible recent-document registration request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentDocumentRegistrationRequest {
    /// Target platform contract.
    pub platform: PlatformTarget,
    /// Stable local document reference, such as a path or local URI.
    pub document_ref: String,
    /// Optional caller-known document hash.
    pub document_hash: Option<String>,
    /// Optional patch plan identifier.
    pub patch_plan_id: Option<String>,
    /// Optional approval token.
    pub approval_token: Option<String>,
}

impl RecentDocumentRegistrationRequest {
    /// Builds a Wave 1 smoke request for a local document reference.
    #[must_use]
    pub fn smoke(platform: PlatformTarget, document_ref: impl Into<String>) -> Self {
        Self {
            platform,
            document_ref: document_ref.into(),
            document_hash: None,
            patch_plan_id: None,
            approval_token: None,
        }
    }
}

/// No-op/default-deny platform operation receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformOperationReceipt {
    /// Target platform contract.
    pub platform: PlatformTarget,
    /// Operation class.
    pub operation: PlatformOperation,
    /// Policy decision made by the stub.
    pub decision: PlatformPolicyDecision,
    /// Whether a host platform mutation was applied.
    pub applied: bool,
    /// Guards required before this operation may mutate platform state.
    pub required_guards: Vec<PlatformMutationGuard>,
    /// Human-readable local diagnostic note.
    pub note: String,
}

impl PlatformOperationReceipt {
    /// Validates the Wave 1 default-deny invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when a receipt would imply a platform mutation.
    pub fn validate_default_deny(&self) -> Result<(), PlatformContractError> {
        if self.applied {
            return Err(PlatformContractError::UnexpectedPlatformMutation);
        }
        if self.decision != PlatformPolicyDecision::DefaultDenied {
            return Err(PlatformContractError::ExpectedDefaultDeny);
        }
        let required = PlatformMutationGuard::required_for_mutation();
        if required
            .iter()
            .any(|guard| !self.required_guards.contains(guard))
        {
            return Err(PlatformContractError::MissingMutationGuard);
        }
        Ok(())
    }
}

/// Platform contract validation errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PlatformContractError {
    /// A Wave 1 stub reported that it applied a host platform mutation.
    #[error("Wave 1 platform stubs must not mutate host platform state")]
    UnexpectedPlatformMutation,
    /// A Wave 1 stub did not return a default-deny policy decision.
    #[error("Wave 1 platform stubs must default deny mutating requests")]
    ExpectedDefaultDeny,
    /// A platform operation receipt omitted one or more required mutation guards.
    #[error("platform operation receipt omitted required mutation guards")]
    MissingMutationGuard,
}

/// No-op recent-document adapter stub.
#[derive(Debug, Clone, Copy, Default)]
pub struct RecentDocumentAdapter;

impl RecentDocumentAdapter {
    /// Constructs the no-op/default-deny adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns a default-deny receipt without calling native platform APIs.
    #[must_use]
    pub fn register_recent_document(
        &self,
        request: &RecentDocumentRegistrationRequest,
    ) -> PlatformOperationReceipt {
        PlatformOperationReceipt {
            platform: request.platform,
            operation: PlatformOperation::RegisterRecentDocument,
            decision: PlatformPolicyDecision::DefaultDenied,
            applied: false,
            required_guards: PlatformMutationGuard::required_for_mutation().to_vec(),
            note: format!(
                "recent-document registration for {} is no-op/default-deny in Wave 1",
                request.document_ref
            ),
        }
    }
}

/// Runs the bounded Wave 1 recent-document smoke contract.
///
/// # Errors
///
/// Returns an error if any target receipt violates default-deny invariants.
pub fn wave1_recent_document_smoke() -> Result<Vec<PlatformOperationReceipt>, PlatformContractError>
{
    let adapter = RecentDocumentAdapter::new();
    PlatformTarget::wave1_recent_document_targets()
        .into_iter()
        .map(|platform| {
            let request =
                RecentDocumentRegistrationRequest::smoke(platform, "fixtures/corpus/basic.pdf");
            let receipt = adapter.register_recent_document(&request);
            receipt.validate_default_deny()?;
            Ok(receipt)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn all_wave1_targets_are_covered() {
        assert_eq!(
            PlatformTarget::wave1_recent_document_targets(),
            [
                PlatformTarget::Windows,
                PlatformTarget::Macos,
                PlatformTarget::Linux,
                PlatformTarget::Android,
                PlatformTarget::Ios,
            ]
        );
    }

    #[test]
    fn recent_document_registration_is_default_deny_for_every_platform() {
        let receipts = wave1_recent_document_smoke().unwrap();
        assert_eq!(receipts.len(), 5);

        for receipt in receipts {
            assert_eq!(receipt.operation, PlatformOperation::RegisterRecentDocument);
            assert_eq!(receipt.decision, PlatformPolicyDecision::DefaultDenied);
            assert!(!receipt.applied);
            receipt.validate_default_deny().unwrap();
        }
    }

    #[test]
    fn default_deny_validation_rejects_applied_receipt() {
        let adapter = RecentDocumentAdapter::new();
        let request = RecentDocumentRegistrationRequest::smoke(
            PlatformTarget::Linux,
            "fixtures/corpus/basic.pdf",
        );
        let mut receipt = adapter.register_recent_document(&request);
        receipt.applied = true;

        assert_eq!(
            receipt.validate_default_deny().unwrap_err(),
            PlatformContractError::UnexpectedPlatformMutation
        );
    }

    #[test]
    fn default_deny_validation_requires_all_mutation_guards() {
        let adapter = RecentDocumentAdapter::new();
        let request = RecentDocumentRegistrationRequest::smoke(
            PlatformTarget::Windows,
            "fixtures/corpus/basic.pdf",
        );
        let mut receipt = adapter.register_recent_document(&request);
        receipt.required_guards.pop();

        assert_eq!(
            receipt.validate_default_deny().unwrap_err(),
            PlatformContractError::MissingMutationGuard
        );
    }
}
