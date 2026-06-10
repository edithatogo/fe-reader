//! Fe Reader transformation pass contract.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
pub struct TransformationPassRegistry {
    pub definitions: BTreeMap<String, TransformationPassDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPassDefinition {
    pub pass_type: String,
    pub version: String,
    pub maturity: PassMaturity,
    pub allowed_policy_risks: Vec<PolicyRisk>,
    pub required_inputs: Vec<String>,
    pub produced_outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationReport {
    pub graph_id: String,
    pub input_document_sha256: String,
    pub accepted_passes: Vec<CompiledPass>,
    pub expected_write_mode: WriteMode,
    pub mutation_policy: String,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledPass {
    pub pass_id: String,
    pub pass_type: String,
    pub definition_version: String,
    pub policy_risk: PolicyRisk,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
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
