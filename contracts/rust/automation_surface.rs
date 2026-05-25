//! Automation surfaces map platform/app automation into FeOperationIntent.

use serde::{Deserialize, Serialize};
use crate::core_types::{FeApprovalToken, FeAuditReceipt, FeOperationIntent, FePatchPlan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationClientIdentity {
    pub surface: String,
    pub client_id: String,
    pub display_name: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationResult {
    ReadOnlyJson(serde_json::Value),
    PatchPlan(FePatchPlan),
    Denied { reason: String },
}

pub trait AutomationSurface: Send + Sync {
    fn identify_client(&self) -> anyhow::Result<AutomationClientIdentity>;
    fn submit_intent(&self, intent: FeOperationIntent) -> anyhow::Result<AutomationResult>;
    fn apply_approved_patch(
        &self,
        patch_plan_id: &str,
        document_sha256_before: &str,
        policy_allow_rule: &str,
        approval: FeApprovalToken,
    ) -> anyhow::Result<FeAuditReceipt>;
}
