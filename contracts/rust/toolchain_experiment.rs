//! Toolchain and experimental-lane governance contracts.

#[derive(Debug, Clone)]
pub struct ToolchainExperiment {
    pub experiment_id: String,
    pub owner: String,
    pub category: String,
    pub feature_flag: String,
    pub risk: String,
    pub fallback: String,
    pub benchmark_target: Option<String>,
    pub review_date: String,
}

#[derive(Debug, Clone)]
pub struct ExperimentDecision {
    pub experiment_id: String,
    pub decision: String,
    pub evidence: Vec<String>,
    pub promoted_to_default: bool,
}

pub trait ExperimentRegistry: Send + Sync {
    fn register(&self, experiment: ToolchainExperiment) -> Result<(), String>;
    fn decide(&self, decision: ExperimentDecision) -> Result<(), String>;
}
