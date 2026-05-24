//! Unified configuration and policy engine contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationRequest {
    pub subject: PolicySubject,
    pub action: String,
    pub resource: String,
    pub context: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicySubject {
    User,
    Cli,
    McpClient { client_id: String },
    Plugin { plugin_id: String },
    Automation { surface: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResult {
    pub decision: PolicyDecision,
    pub matched_rules: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyDecision { Allow, Deny, RequireApproval, RequireStrongerAuth }

pub trait ConfigPolicyEngine {
    fn evaluate(&self, request: PolicyEvaluationRequest) -> PolicyEvaluationResult;
    fn explain_effective_policy(&self) -> serde_json::Value;
}
