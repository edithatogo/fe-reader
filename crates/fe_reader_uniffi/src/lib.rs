//! Wave 0 UniFFI facade for Fe Reader.
//!
//! Keep this crate as a narrow adapter over `fe_reader_core`. It may expose stable DTOs for
//! bindings, but it must not grow PDF parsing, rendering, platform, plugin, or automation logic.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentId, OperationIntent, OperationKind, OperationSource, PatchOperation, PatchPlan,
    RiskLevel, WriteMode,
};

uniffi::setup_scaffolding!();

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Binding-visible identity record for Fe Reader's UniFFI facade.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct FeBindingInfo {
    /// Adapter crate name.
    pub crate_name: String,
    /// Adapter crate version.
    pub crate_version: String,
    /// Core crate identity used by the facade.
    pub core_identity: String,
}

/// Binding-visible operation source.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FeOperationSource {
    /// Human-driven UI request.
    Ui,
    /// Command line request.
    Cli,
    /// Model Context Protocol tool request.
    Mcp,
    /// Native automation surface request.
    Automation,
    /// Browser extension or web request.
    Web,
    /// WASM plugin proposal request.
    Plugin,
}

/// Binding-visible risk class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FeRiskLevel {
    /// Inspection only.
    ReadOnly,
    /// Reversible local state change.
    LocalState,
    /// PDF mutation that needs review.
    DocumentMutation,
    /// High-risk mutation such as redaction or signing.
    HighRisk,
}

/// Binding-visible read-only operation intent DTO.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct FeOperationIntent {
    /// Stable operation identifier.
    pub intent_id: String,
    /// Calling surface.
    pub source: FeOperationSource,
    /// Target document identifier.
    pub document_id: String,
    /// Optional document hash captured before planning.
    pub document_sha256: Option<String>,
    /// Human-readable operation label.
    pub label: String,
    /// Risk classification.
    pub risk_level: FeRiskLevel,
    /// Whether policy requires explicit review before apply.
    pub requires_review: bool,
}

/// Binding-visible patch plan DTO.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct FePatchPlan {
    /// Stable patch plan identifier.
    pub plan_id: String,
    /// Operation identifier that produced this plan.
    pub intent_id: String,
    /// Target document identifier.
    pub document_id: String,
    /// Human-readable plan summary.
    pub summary: String,
    /// Required write mode.
    pub write_mode: String,
    /// Risk classification.
    pub risk_level: FeRiskLevel,
    /// Whether this plan can be applied without further review.
    pub approved_for_apply: bool,
    /// Number of planned operations.
    pub operation_count: u32,
}

/// Returns the identity of the UniFFI facade and the core crate it delegates to.
#[must_use]
#[uniffi::export]
pub fn binding_info() -> FeBindingInfo {
    FeBindingInfo {
        crate_name: CRATE_NAME.to_owned(),
        crate_version: CRATE_VERSION.to_owned(),
        core_identity: format!(
            "{}@{}",
            fe_reader_core::CRATE_NAME,
            fe_reader_core::CRATE_VERSION
        ),
    }
}

/// Creates a read-only inspect intent for binding smoke tests.
///
/// This function creates an intent only. It does not expose any document mutation or apply path.
#[must_use]
#[uniffi::export]
pub fn create_read_only_intent(
    source: FeOperationSource,
    document_id: String,
    label: String,
) -> FeOperationIntent {
    let intent = OperationIntent::read_only(source.to_core(), DocumentId(document_id), label);
    intent.into()
}

/// Creates a no-op patch plan from a read-only binding intent.
///
/// The returned plan is `no_write`, read-only, and not approved for apply.
#[must_use]
#[uniffi::export]
pub fn draft_noop_plan(intent: FeOperationIntent, summary: String) -> FePatchPlan {
    let core_intent = intent.to_core();
    let plan = PatchPlan::draft(&core_intent, summary, vec![PatchOperation::Noop]);
    plan.into()
}

impl FeOperationSource {
    fn to_core(&self) -> OperationSource {
        match self {
            Self::Ui => OperationSource::Ui,
            Self::Cli => OperationSource::Cli,
            Self::Mcp => OperationSource::Mcp,
            Self::Automation => OperationSource::Automation,
            Self::Web => OperationSource::Web,
            Self::Plugin => OperationSource::Plugin,
        }
    }
}

impl FeRiskLevel {
    fn from_core(risk_level: RiskLevel) -> Self {
        match risk_level {
            RiskLevel::ReadOnly => Self::ReadOnly,
            RiskLevel::LocalState => Self::LocalState,
            RiskLevel::DocumentMutation => Self::DocumentMutation,
            RiskLevel::HighRisk => Self::HighRisk,
        }
    }

    fn to_core(self) -> RiskLevel {
        match self {
            Self::ReadOnly => RiskLevel::ReadOnly,
            Self::LocalState => RiskLevel::LocalState,
            Self::DocumentMutation => RiskLevel::DocumentMutation,
            Self::HighRisk => RiskLevel::HighRisk,
        }
    }
}

impl FeOperationIntent {
    fn to_core(&self) -> OperationIntent {
        OperationIntent {
            intent_id: fe_reader_core::OperationId(self.intent_id.clone()),
            source: self.source.to_core(),
            document_id: DocumentId(self.document_id.clone()),
            document_fingerprint: None,
            kind: OperationKind::Inspect,
            label: self.label.clone(),
            risk_level: self.risk_level.to_core(),
            requires_review: self.requires_review,
        }
    }
}

impl From<OperationIntent> for FeOperationIntent {
    fn from(intent: OperationIntent) -> Self {
        Self {
            intent_id: intent.intent_id.0,
            source: FeOperationSource::from_core(&intent.source),
            document_id: intent.document_id.0,
            document_sha256: intent
                .document_fingerprint
                .map(|fingerprint| fingerprint.sha256_hex),
            label: intent.label,
            risk_level: FeRiskLevel::from_core(intent.risk_level),
            requires_review: intent.requires_review,
        }
    }
}

impl FeOperationSource {
    fn from_core(source: &OperationSource) -> Self {
        match source {
            OperationSource::Ui => Self::Ui,
            OperationSource::Cli => Self::Cli,
            OperationSource::Mcp => Self::Mcp,
            OperationSource::Automation => Self::Automation,
            OperationSource::Web => Self::Web,
            OperationSource::Plugin => Self::Plugin,
        }
    }
}

impl From<PatchPlan> for FePatchPlan {
    fn from(plan: PatchPlan) -> Self {
        Self {
            plan_id: plan.plan_id.0,
            intent_id: plan.intent_id.0,
            document_id: plan.document_id.0,
            summary: plan.summary,
            write_mode: write_mode_name(plan.write_mode).to_owned(),
            risk_level: FeRiskLevel::from_core(plan.risk_level),
            approved_for_apply: plan.approved_for_apply,
            operation_count: plan.operations.len() as u32,
        }
    }
}

fn write_mode_name(write_mode: WriteMode) -> &'static str {
    match write_mode {
        WriteMode::NoWrite => "no_write",
        WriteMode::IncrementalAppend => "incremental_append",
        WriteMode::FullRewrite => "full_rewrite",
        WriteMode::SanitizingRewrite => "sanitizing_rewrite",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn binding_info_references_core() {
        let info = binding_info();

        assert_eq!(info.crate_name, CRATE_NAME);
        assert_eq!(info.crate_version, CRATE_VERSION);
        assert!(info.core_identity.contains(fe_reader_core::CRATE_NAME));
    }

    #[test]
    fn read_only_intent_uses_core_contract_defaults() {
        let intent = create_read_only_intent(
            FeOperationSource::Cli,
            "doc-1".to_owned(),
            "inspect".to_owned(),
        );

        assert_eq!(intent.document_id, "doc-1");
        assert_eq!(intent.source, FeOperationSource::Cli);
        assert_eq!(intent.label, "inspect");
        assert_eq!(intent.risk_level, FeRiskLevel::ReadOnly);
        assert!(!intent.requires_review);
    }

    #[test]
    fn noop_plan_uses_no_write_policy() {
        let intent = create_read_only_intent(
            FeOperationSource::Cli,
            "doc-1".to_owned(),
            "inspect".to_owned(),
        );
        let plan = draft_noop_plan(intent.clone(), "read-only inspect".to_owned());

        assert_eq!(plan.intent_id, intent.intent_id);
        assert_eq!(plan.document_id, "doc-1");
        assert_eq!(plan.summary, "read-only inspect");
        assert_eq!(plan.write_mode, "no_write");
        assert_eq!(plan.risk_level, FeRiskLevel::ReadOnly);
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.operation_count, 1);
    }

    #[test]
    fn android_intent_style_smoke_stays_read_only() {
        let intent = create_read_only_intent(
            FeOperationSource::Automation,
            "android-doc-1".to_owned(),
            "android.intent.action.VIEW".to_owned(),
        );

        assert_eq!(intent.source, FeOperationSource::Automation);
        assert_eq!(intent.label, "android.intent.action.VIEW");
        assert_eq!(intent.risk_level, FeRiskLevel::ReadOnly);
        assert!(!intent.requires_review);
    }

    #[test]
    fn ios_app_intent_style_smoke_stays_plan_only() {
        let intent = create_read_only_intent(
            FeOperationSource::Automation,
            "ios-doc-1".to_owned(),
            "FeOpenDocumentIntent".to_owned(),
        );
        let plan = draft_noop_plan(intent, "iOS App Intent smoke plan".to_owned());

        assert_eq!(plan.document_id, "ios-doc-1");
        assert_eq!(plan.write_mode, "no_write");
        assert_eq!(plan.risk_level, FeRiskLevel::ReadOnly);
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.operation_count, 1);
    }
}
