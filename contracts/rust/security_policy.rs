//! Security and policy contract.
//! This file is an interface specification; implementation belongs in fe_reader_security.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskClass {
    ReadOnly,
    LowMutation,
    HighMutation,
    SensitiveExport,
    SecureRedaction,
    ExternalExecution,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutomationSurface {
    Ui,
    Cli,
    Mcp,
    Com,
    AppleScript,
    DBus,
    AndroidIntent,
    IosAppIntent,
    WebPostMessage,
    BrowserExtension,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    RequireInteractiveApproval { reason: String },
    RequireManagedApproval { reason: String },
    Deny { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicyInput {
    pub operation_id: String,
    pub source: AutomationSurface,
    pub risk_class: RiskClass,
    pub document_sha256: Option<String>,
    pub patch_plan_id: Option<String>,
    pub approval_token_id: Option<String>,
    pub plugin_id: Option<String>,
    pub workflow_pack_id: Option<String>,
    pub target_path_class: Option<String>,
}

pub trait SecurityPolicyEngine: Send + Sync {
    fn evaluate(&self, input: &SecurityPolicyInput) -> PolicyDecision;
}
