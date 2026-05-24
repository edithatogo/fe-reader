//! Workflow pack engine contract.

use serde::{Deserialize, Serialize};
use crate::core_types::{FeOperationIntent, FePatchPlan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPackDescriptor {
    pub workflow_id: String,
    pub version: String,
    pub display_name: String,
    pub workflow_family: String,
    pub risk_level: String,
    pub requires_human_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunRequest {
    pub workflow_id: String,
    pub document_id: String,
    pub parameters: serde_json::Value,
}

pub trait WorkflowEngine: Send + Sync {
    fn list_workflows(&self) -> anyhow::Result<Vec<WorkflowPackDescriptor>>;
    fn plan_workflow(&self, intent: FeOperationIntent, request: WorkflowRunRequest) -> anyhow::Result<FePatchPlan>;
}
