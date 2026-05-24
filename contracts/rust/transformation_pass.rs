//! Fe Reader transformation pass contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationGraph {
    pub graph_id: String,
    pub input_document_sha256: String,
    pub passes: Vec<TransformationPassSpec>,
    pub expected_write_mode: WriteMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPassSpec {
    pub pass_id: String,
    pub pass_type: String,
    pub maturity: PassMaturity,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub policy_risk: PolicyRisk,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassMaturity { Experimental, Preview, Stable }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyRisk { ReadOnly, LowMutation, HighRiskMutation, SecurityCritical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriteMode { IncrementalAppend, FullRewrite, FullSanitizingRewrite }

pub trait TransformationPass {
    fn spec(&self) -> TransformationPassSpec;
    fn validate(&self, context: &PassValidationContext) -> Result<(), PassError>;
    fn plan(&self, context: &PassPlanningContext) -> Result<PassPlanOutput, PassError>;
}

#[derive(Debug)]
pub struct PassValidationContext { pub document_sha256: String }
#[derive(Debug)]
pub struct PassPlanningContext { pub document_sha256: String }
#[derive(Debug)]
pub struct PassPlanOutput { pub produced_nodes: Vec<String>, pub warnings: Vec<String> }
#[derive(Debug)]
pub struct PassError { pub code: String, pub message: String }
