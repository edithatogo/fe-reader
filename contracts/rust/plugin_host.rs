//! WASM plugin host contract. Plugins propose actions; they do not mutate documents directly.

use serde::{Deserialize, Serialize};
use crate::core_types::{FeApprovalToken, FePatchPlan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FePluginManifest {
    pub plugin_id: String,
    pub plugin_version: String,
    pub fe_plugin_api: String,
    pub publisher: String,
    pub license: String,
    pub capabilities: Vec<String>,
    pub network_access: bool,
    pub filesystem_access: bool,
    pub sha256: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProposalRequest {
    pub plugin_id: String,
    pub document_id: String,
    pub document_sha256: String,
    pub operation: String,
    pub input_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProposal {
    pub plugin_id: String,
    pub patch_plan: Option<FePatchPlan>,
    pub annotations_json: serde_json::Value,
    pub warnings: Vec<String>,
}

pub trait PluginHost: Send + Sync {
    fn load_plugin(
        &self,
        manifest: FePluginManifest,
        wasm_bytes: &[u8],
        policy_approval: FeApprovalToken,
    ) -> anyhow::Result<()>;
    fn propose(&self, request: PluginProposalRequest) -> anyhow::Result<PluginProposal>;
}
