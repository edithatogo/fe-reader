//! Workflow pack planning contracts.
//!
//! Wave 3 workflow packs only produce reviewable patch-plan drafts. This crate does not apply
//! plans, mutate PDF bytes, call automation surfaces, or load plugins.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentFingerprint, DocumentId, FeError, FeErrorKind, OperationIntent, OperationKind,
    OperationSource, PatchOperation, PatchPlan, RiskLevel, WriteMode,
};
use serde::{Deserialize, Serialize};

/// Maximum operations a workflow pack may draft in one plan without escalation.
pub const MAX_WORKFLOW_OPERATIONS: usize = 500;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Workflow pack domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDomain {
    /// Legal review and filing workflows.
    Legal,
    /// Healthcare privacy and release workflows.
    Healthcare,
    /// Government records and public-release workflows.
    Government,
    /// Research citation and evidence workflows.
    Research,
    /// Publishing/prepress workflows.
    Publishing,
}

/// Workflow pack definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowPack {
    /// Stable pack id.
    pub pack_id: String,
    /// User-visible display name.
    pub display_name: String,
    /// Domain covered by the pack.
    pub domain: WorkflowDomain,
    /// Risk assigned before planning.
    pub risk_level: RiskLevel,
    /// Plan-only operations.
    pub operations: Vec<PatchOperation>,
}

impl WorkflowPack {
    /// Returns the deterministic Wave 3 baseline workflow packs.
    #[must_use]
    pub fn wave3_baseline_packs() -> Vec<Self> {
        vec![
            Self {
                pack_id: "legal.affidavit.initials.every_page".to_string(),
                display_name: "Initial every selected page".to_string(),
                domain: WorkflowDomain::Legal,
                risk_level: RiskLevel::HighRisk,
                operations: vec![PatchOperation::PlaceStamp {
                    page_index: 0,
                    stamp_ref: "signature.initials.bottom_right".to_string(),
                }],
            },
            Self {
                pack_id: "healthcare.deidentify.basic".to_string(),
                display_name: "Basic healthcare de-identification".to_string(),
                domain: WorkflowDomain::Healthcare,
                risk_level: RiskLevel::HighRisk,
                operations: vec![PatchOperation::RedactRegion {
                    page_index: 0,
                    region: "healthcare:review_candidate_placeholder".to_string(),
                }],
            },
            Self {
                pack_id: "government.foi.redaction.exemption_tags".to_string(),
                display_name: "FOI/RTI redaction with exemption tags".to_string(),
                domain: WorkflowDomain::Government,
                risk_level: RiskLevel::HighRisk,
                operations: vec![PatchOperation::RedactRegion {
                    page_index: 0,
                    region: "government:review_candidate_placeholder".to_string(),
                }],
            },
            Self {
                pack_id: "research.highlights.to_markdown_zotero".to_string(),
                display_name: "Export highlights to Markdown/Zotero".to_string(),
                domain: WorkflowDomain::Research,
                risk_level: RiskLevel::ReadOnly,
                operations: vec![PatchOperation::Noop],
            },
            Self {
                pack_id: "publishing.pdfa.preflight".to_string(),
                display_name: "PDF/A preflight report".to_string(),
                domain: WorkflowDomain::Publishing,
                risk_level: RiskLevel::ReadOnly,
                operations: vec![PatchOperation::Noop],
            },
        ]
    }
}

/// Optional policy inputs for bounded workflow-pack planning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowPlanOptions {
    /// Caller-requested write mode. High-risk packs cannot downgrade below required mode.
    pub requested_write_mode: Option<WriteMode>,
}

/// Planned workflow pack output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedWorkflowPack {
    /// Pack that produced the plan.
    pub pack: WorkflowPack,
    /// Operation intent.
    pub intent: OperationIntent,
    /// Draft patch plan.
    pub plan: PatchPlan,
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Plans a workflow pack without applying it.
#[must_use]
pub fn plan_workflow_pack(
    source: OperationSource,
    document_id: DocumentId,
    fingerprint: DocumentFingerprint,
    pack: WorkflowPack,
) -> PlannedWorkflowPack {
    try_plan_workflow_pack(
        source,
        document_id,
        fingerprint,
        pack,
        WorkflowPlanOptions::default(),
    )
    .expect("Wave 3 baseline workflow packs must be valid")
}

/// Plans a workflow pack after enforcing bounded plan-only policy.
///
/// # Errors
///
/// Returns an error when the pack is empty, exceeds operation bounds, or requests an unsafe
/// write-mode downgrade for high-risk operations.
pub fn try_plan_workflow_pack(
    source: OperationSource,
    document_id: DocumentId,
    fingerprint: DocumentFingerprint,
    pack: WorkflowPack,
    options: WorkflowPlanOptions,
) -> Result<PlannedWorkflowPack, FeError> {
    validate_pack_bounds(&pack)?;
    let intent = OperationIntent::new(
        source,
        document_id,
        OperationKind::PlanMutation,
        format!("plan_workflow:{}", pack.pack_id),
        pack.risk_level,
    )
    .with_document_fingerprint(fingerprint);
    let plan = PatchPlan::draft(&intent, pack.display_name.clone(), pack.operations.clone());
    validate_requested_write_mode(&pack, &plan, options.requested_write_mode)?;
    Ok(PlannedWorkflowPack { pack, intent, plan })
}

/// Plans all Wave 3 baseline workflow packs.
#[must_use]
pub fn plan_wave3_baseline_packs(fingerprint: DocumentFingerprint) -> Vec<PlannedWorkflowPack> {
    WorkflowPack::wave3_baseline_packs()
        .into_iter()
        .map(|pack| {
            plan_workflow_pack(
                OperationSource::Cli,
                DocumentId::new(),
                fingerprint.clone(),
                pack,
            )
        })
        .collect()
}

fn validate_pack_bounds(pack: &WorkflowPack) -> Result<(), FeError> {
    if pack.operations.is_empty() {
        return Err(FeError::new(
            FeErrorKind::InvalidInput,
            format!(
                "workflow pack {} must produce at least one operation",
                pack.pack_id
            ),
        ));
    }
    if pack.operations.len() > MAX_WORKFLOW_OPERATIONS {
        return Err(FeError::new(
            FeErrorKind::ResourceLimit,
            format!(
                "workflow pack {} exceeds operation bound {}",
                pack.pack_id, MAX_WORKFLOW_OPERATIONS
            ),
        ));
    }
    Ok(())
}

fn validate_requested_write_mode(
    pack: &WorkflowPack,
    plan: &PatchPlan,
    requested_write_mode: Option<WriteMode>,
) -> Result<(), FeError> {
    let Some(requested_write_mode) = requested_write_mode else {
        return Ok(());
    };
    if pack.risk_level == RiskLevel::HighRisk
        && write_mode_rank(requested_write_mode) < write_mode_rank(plan.write_mode)
    {
        return Err(FeError::new(
            FeErrorKind::PolicyDenied,
            format!(
                "workflow pack {} requires {:?}; requested {:?}",
                pack.pack_id, plan.write_mode, requested_write_mode
            ),
        ));
    }
    Ok(())
}

fn write_mode_rank(write_mode: WriteMode) -> u8 {
    match write_mode {
        WriteMode::NoWrite => 0,
        WriteMode::IncrementalAppend => 1,
        WriteMode::FullRewrite => 2,
        WriteMode::SanitizingRewrite => 3,
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
    fn wave3_baseline_covers_required_domains() {
        let packs = WorkflowPack::wave3_baseline_packs();
        let domains = packs.iter().map(|pack| pack.domain).collect::<Vec<_>>();

        assert_eq!(packs.len(), 5);
        assert!(domains.contains(&WorkflowDomain::Legal));
        assert!(domains.contains(&WorkflowDomain::Healthcare));
        assert!(domains.contains(&WorkflowDomain::Government));
        assert!(domains.contains(&WorkflowDomain::Research));
        assert!(domains.contains(&WorkflowDomain::Publishing));
    }

    #[test]
    fn workflow_packs_are_plan_only_and_unapproved() {
        let planned = plan_wave3_baseline_packs(DocumentFingerprint::from_bytes(b"%PDF-1.7\n"));

        for item in planned {
            assert!(!item.plan.approved_for_apply);
            assert_eq!(item.plan.document_id, item.intent.document_id);
            assert_eq!(item.plan.intent_id, item.intent.intent_id);
            assert!(matches!(
                item.plan.risk_level,
                RiskLevel::ReadOnly | RiskLevel::HighRisk
            ));
        }
    }

    #[test]
    fn high_risk_workflow_packs_preserve_required_write_modes() {
        let planned = plan_wave3_baseline_packs(DocumentFingerprint::from_bytes(b"%PDF-1.7\n"));

        let legal = planned
            .iter()
            .find(|item| item.pack.domain == WorkflowDomain::Legal)
            .unwrap();
        assert_eq!(legal.plan.risk_level, RiskLevel::HighRisk);
        assert_eq!(legal.plan.write_mode, WriteMode::IncrementalAppend);

        for item in planned.iter().filter(|item| {
            matches!(
                item.pack.domain,
                WorkflowDomain::Healthcare | WorkflowDomain::Government
            )
        }) {
            assert_eq!(item.plan.risk_level, RiskLevel::HighRisk);
            assert_eq!(item.plan.write_mode, WriteMode::SanitizingRewrite);
        }
    }

    #[test]
    fn high_risk_workflow_packs_reject_unsafe_write_mode_downgrades() {
        for pack in WorkflowPack::wave3_baseline_packs()
            .into_iter()
            .filter(|pack| pack.risk_level == RiskLevel::HighRisk)
        {
            let error = try_plan_workflow_pack(
                OperationSource::Cli,
                DocumentId::new(),
                DocumentFingerprint::from_bytes(b"%PDF-1.7\n"),
                pack,
                WorkflowPlanOptions {
                    requested_write_mode: Some(WriteMode::NoWrite),
                },
            )
            .unwrap_err();
            assert_eq!(error.kind, FeErrorKind::PolicyDenied);
        }
    }

    #[test]
    fn workflow_pack_operation_count_is_bounded() {
        let mut pack = WorkflowPack::wave3_baseline_packs().remove(0);
        pack.operations = vec![PatchOperation::Noop; MAX_WORKFLOW_OPERATIONS + 1];

        let error = try_plan_workflow_pack(
            OperationSource::Cli,
            DocumentId::new(),
            DocumentFingerprint::from_bytes(b"%PDF-1.7\n"),
            pack,
            WorkflowPlanOptions::default(),
        )
        .unwrap_err();

        assert_eq!(error.kind, FeErrorKind::ResourceLimit);
    }
}
