//! Enterprise/managed policy contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterprisePolicy {
    pub policy_version: String,
    pub managed: bool,
    pub allowed_update_channels: Vec<String>,
    pub disabled_surfaces: Vec<String>,
    pub disabled_features: Vec<String>,
    pub require_redaction_verification: bool,
    pub require_metadata_clean_share_prompt: bool,
    pub allow_plugins: bool,
    pub allow_mcp: bool,
    pub allow_external_converters: bool,
    pub telemetry_mode: DiagnosticsMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticsMode {
    Disabled,
    LocalOnly,
    UserApprovedBundleOnly,
    ManagedUploadAllowed,
}

pub trait EnterprisePolicyProvider: Send + Sync {
    fn effective_policy(&self, workspace_id: Option<&str>) -> anyhow::Result<EnterprisePolicy>;
}
