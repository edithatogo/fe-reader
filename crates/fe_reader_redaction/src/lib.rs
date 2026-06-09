//! Redaction planning and verification contracts.
//!
//! Wave 3 remains plan/verify-only. This crate does not mutate PDF bytes or expose an apply path.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentFingerprint, DocumentId, FeError, FeErrorKind, OperationIntent, OperationReceipt,
    OperationSource, PatchOperation, PatchPlan, RiskLevel, TransactionJournal, TransactionState,
    VerificationStatus, WriteMode,
};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Redaction recipe security level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RedactionSecurityLevel {
    /// Markup-only redaction is never accepted as secure content removal.
    MarkupOnly,
    /// Content removal requires a full sanitizing rewrite in Fe Reader.
    ContentRemoval,
    /// Full sanitizing rewrite is required for secure redaction.
    SanitizedRewrite,
}

/// One redaction region planned by a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionRegion {
    /// Zero-based page index.
    pub page_index: u32,
    /// Opaque region label or bbox string.
    pub region: String,
}

/// Plan-only redaction recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionRecipe {
    /// Stable recipe id.
    pub recipe_id: String,
    /// Recipe version.
    pub version: String,
    /// User-visible recipe name.
    pub display_name: String,
    /// Security level requested by the recipe.
    pub security_level: RedactionSecurityLevel,
    /// Regions proposed for redaction.
    pub regions: Vec<RedactionRegion>,
    /// Whether human review is required before apply.
    pub requires_human_review: bool,
    /// Verification checks required after apply.
    pub verification: Vec<String>,
}

impl RedactionRecipe {
    /// Builds a deterministic Wave 3 smoke recipe.
    #[must_use]
    pub fn smoke_secure() -> Self {
        Self {
            recipe_id: "wave3-smoke-secure-redaction".to_string(),
            version: "0.1.0".to_string(),
            display_name: "Wave 3 secure redaction smoke".to_string(),
            security_level: RedactionSecurityLevel::SanitizedRewrite,
            regions: vec![RedactionRegion {
                page_index: 0,
                region: "bbox:72,72,144,24".to_string(),
            }],
            requires_human_review: true,
            verification: vec![
                "no_incremental_append".to_string(),
                "planned_regions_have_no_visible_text_leaks".to_string(),
            ],
        }
    }
}

/// Redaction verification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionVerificationReport {
    /// Whether the plan passed redaction verification preconditions.
    pub passed: bool,
    /// Deterministic verification checks that were evaluated.
    pub checks: Vec<String>,
    /// Verification findings.
    pub findings: Vec<String>,
    /// Audit receipt emitted for this verification.
    pub receipt: OperationReceipt,
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Plans secure redaction as a high-risk, unapproved patch plan.
///
/// # Errors
///
/// Returns an error when the recipe would be markup-only or lacks regions.
pub fn plan_secure_redaction(
    source: OperationSource,
    document_id: DocumentId,
    fingerprint: DocumentFingerprint,
    recipe: &RedactionRecipe,
) -> Result<(OperationIntent, PatchPlan), FeError> {
    if recipe.security_level == RedactionSecurityLevel::MarkupOnly {
        return Err(FeError::new(
            FeErrorKind::InvalidInput,
            "markup-only redaction is not secure redaction",
        ));
    }
    if recipe.regions.is_empty() {
        return Err(FeError::new(
            FeErrorKind::InvalidInput,
            "secure redaction requires at least one region",
        ));
    }
    let intent = OperationIntent::high_risk(
        source,
        document_id,
        fe_reader_core::OperationKind::PlanMutation,
        "plan_secure_redaction",
    )
    .with_document_fingerprint(fingerprint);
    let operations = recipe
        .regions
        .iter()
        .map(|region| PatchOperation::RedactRegion {
            page_index: region.page_index,
            region: region.region.clone(),
        })
        .collect::<Vec<_>>();
    let plan = PatchPlan::draft(&intent, recipe.display_name.clone(), operations);
    Ok((intent, plan))
}

/// Verifies secure redaction plan invariants and emits an audit receipt.
///
/// # Errors
///
/// Returns an error when the plan is not sanitizing-rewrite/high-risk or uses incremental append.
pub fn verify_secure_redaction_plan(
    intent: &OperationIntent,
    plan: &PatchPlan,
    after_fingerprint: DocumentFingerprint,
) -> Result<RedactionVerificationReport, FeError> {
    if plan.write_mode == WriteMode::IncrementalAppend {
        return Err(FeError::new(
            FeErrorKind::VerificationFailed,
            "secure redaction must not use incremental append",
        ));
    }
    if plan.write_mode != WriteMode::SanitizingRewrite {
        return Err(FeError::new(
            FeErrorKind::VerificationFailed,
            "secure redaction requires sanitizing rewrite",
        ));
    }
    if plan.risk_level != RiskLevel::HighRisk {
        return Err(FeError::new(
            FeErrorKind::VerificationFailed,
            "secure redaction must remain high risk",
        ));
    }
    let journal = TransactionJournal::planned(plan).transition(
        TransactionState::Verified,
        "redaction verification contract passed; no bytes applied by this crate",
    );
    let receipt = OperationReceipt::verified(
        intent,
        plan,
        &journal,
        VerificationStatus::Passed,
        after_fingerprint,
        "secure redaction verification passed",
    );
    Ok(RedactionVerificationReport {
        passed: true,
        checks: vec![
            "no_incremental_append".to_string(),
            "sanitizing_rewrite_required".to_string(),
            "high_risk_preserved".to_string(),
        ],
        findings: Vec::new(),
        receipt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn secure_redaction_plan_is_high_risk_sanitizing_and_unapproved() {
        let fingerprint = DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF");
        let (intent, plan) = plan_secure_redaction(
            OperationSource::Cli,
            DocumentId::new(),
            fingerprint,
            &RedactionRecipe::smoke_secure(),
        )
        .unwrap();

        assert_eq!(intent.risk_level, RiskLevel::HighRisk);
        assert_eq!(plan.risk_level, RiskLevel::HighRisk);
        assert_eq!(plan.write_mode, WriteMode::SanitizingRewrite);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn secure_redaction_rejects_markup_only_recipe() {
        let mut recipe = RedactionRecipe::smoke_secure();
        recipe.security_level = RedactionSecurityLevel::MarkupOnly;

        let error = plan_secure_redaction(
            OperationSource::Cli,
            DocumentId::new(),
            DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF"),
            &recipe,
        )
        .unwrap_err();

        assert_eq!(error.kind, FeErrorKind::InvalidInput);
    }

    #[test]
    fn secure_redaction_verification_emits_receipt() {
        let fingerprint = DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF");
        let (intent, plan) = plan_secure_redaction(
            OperationSource::Cli,
            DocumentId::new(),
            fingerprint.clone(),
            &RedactionRecipe::smoke_secure(),
        )
        .unwrap();
        let report = verify_secure_redaction_plan(&intent, &plan, fingerprint).unwrap();

        assert!(report.passed);
        assert_eq!(
            report.receipt.verification_status,
            VerificationStatus::Passed
        );
        assert_eq!(report.receipt.write_mode, WriteMode::SanitizingRewrite);
        assert_eq!(report.receipt.risk_level, RiskLevel::HighRisk);
        assert!(report.receipt.transaction_id.is_some());
    }

    #[test]
    fn secure_redaction_verification_rejects_incremental_append() {
        let intent = OperationIntent::high_risk(
            OperationSource::Cli,
            DocumentId::new(),
            fe_reader_core::OperationKind::PlanMutation,
            "bad_redaction",
        );
        let plan = PatchPlan::draft(
            &intent,
            "bad redaction",
            vec![PatchOperation::PlaceStamp {
                page_index: 0,
                stamp_ref: "redaction-overlay".to_string(),
            }],
        );

        let error = verify_secure_redaction_plan(
            &intent,
            &plan,
            DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF"),
        )
        .unwrap_err();

        assert_eq!(error.kind, FeErrorKind::VerificationFailed);
    }
}
