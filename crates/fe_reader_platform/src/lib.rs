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
    /// Open or inspect a document through a native automation surface.
    AutomationRead,
    /// Plan a workflow, redaction or conversion through a native automation surface.
    AutomationPlan,
    /// Attempt to apply an approved patch through a native automation surface.
    AutomationApply,
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

/// Native automation contract surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSurface {
    /// Windows COM automation.
    WindowsCom,
    /// macOS AppleScript.
    MacosAppleScript,
    /// macOS App Intents.
    MacosAppIntent,
    /// Linux D-Bus.
    LinuxDbus,
    /// Android intents/DocumentsProvider.
    AndroidIntent,
    /// iOS App Intents.
    IosAppIntent,
}

impl AutomationSurface {
    /// Returns all Wave 5 native automation contract surfaces.
    #[must_use]
    pub const fn wave5_surfaces() -> [Self; 6] {
        [
            Self::WindowsCom,
            Self::MacosAppleScript,
            Self::MacosAppIntent,
            Self::LinuxDbus,
            Self::AndroidIntent,
            Self::IosAppIntent,
        ]
    }
}

/// Native automation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractRequest {
    /// Automation surface.
    pub surface: AutomationSurface,
    /// Requested operation.
    pub operation: PlatformOperation,
    /// Optional document hash supplied by the caller.
    pub document_hash: Option<String>,
    /// Optional patch plan id supplied by the caller.
    pub patch_plan_id: Option<String>,
    /// Optional approval token supplied by the caller.
    pub approval_token: Option<String>,
}

impl AutomationContractRequest {
    /// Creates a read-only automation request.
    #[must_use]
    pub const fn read_only(surface: AutomationSurface) -> Self {
        Self {
            surface,
            operation: PlatformOperation::AutomationRead,
            document_hash: None,
            patch_plan_id: None,
            approval_token: None,
        }
    }

    /// Creates a plan-only automation request.
    #[must_use]
    pub const fn plan_only(surface: AutomationSurface) -> Self {
        Self {
            surface,
            operation: PlatformOperation::AutomationPlan,
            document_hash: None,
            patch_plan_id: None,
            approval_token: None,
        }
    }
}

/// Native automation contract response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractReceipt {
    /// Automation surface.
    pub surface: AutomationSurface,
    /// Requested operation.
    pub operation: PlatformOperation,
    /// Policy decision.
    pub decision: PlatformPolicyDecision,
    /// Whether native automation mutated document or host state.
    pub applied: bool,
    /// Required mutation guards.
    pub required_guards: Vec<PlatformMutationGuard>,
    /// Local diagnostic note.
    pub note: String,
}

impl AutomationContractReceipt {
    /// Validates Wave 5 read-only/plan-only automation invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if the receipt implies mutation or omits mutation guards.
    pub fn validate_wave5(&self) -> Result<(), PlatformContractError> {
        if self.applied {
            return Err(PlatformContractError::UnexpectedPlatformMutation);
        }
        if matches!(self.operation, PlatformOperation::AutomationApply)
            && self.decision != PlatformPolicyDecision::DefaultDenied
        {
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

/// No-op native automation adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct NativeAutomationAdapter;

impl NativeAutomationAdapter {
    /// Constructs the no-op automation adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Handles automation without calling native APIs.
    #[must_use]
    pub fn handle(&self, request: &AutomationContractRequest) -> AutomationContractReceipt {
        let decision = if matches!(request.operation, PlatformOperation::AutomationRead) {
            PlatformPolicyDecision::ReadOnly
        } else {
            PlatformPolicyDecision::DefaultDenied
        };
        AutomationContractReceipt {
            surface: request.surface,
            operation: request.operation,
            decision,
            applied: false,
            required_guards: PlatformMutationGuard::required_for_mutation().to_vec(),
            note: "native automation is read-only or plan-only by default in Wave 5".to_string(),
        }
    }
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

/// Runs the bounded Wave 5 native automation smoke contract.
///
/// # Errors
///
/// Returns an error if any surface violates read-only/default-deny invariants.
pub fn wave5_native_automation_smoke()
-> Result<Vec<AutomationContractReceipt>, PlatformContractError> {
    let adapter = NativeAutomationAdapter::new();
    let mut receipts = Vec::new();
    for surface in AutomationSurface::wave5_surfaces() {
        for request in [
            AutomationContractRequest::read_only(surface),
            AutomationContractRequest::plan_only(surface),
        ] {
            let receipt = adapter.handle(&request);
            receipt.validate_wave5()?;
            receipts.push(receipt);
        }
    }
    Ok(receipts)
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
    fn native_automation_surfaces_are_read_only_or_plan_only() {
        let receipts = wave5_native_automation_smoke().expect("automation smoke");
        assert_eq!(
            receipts.len(),
            AutomationSurface::wave5_surfaces().len() * 2
        );
        assert!(receipts.iter().all(|receipt| !receipt.applied));
        assert!(
            receipts
                .iter()
                .any(|receipt| matches!(receipt.decision, PlatformPolicyDecision::ReadOnly))
        );
        assert!(
            receipts
                .iter()
                .any(|receipt| matches!(receipt.decision, PlatformPolicyDecision::DefaultDenied))
        );
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
