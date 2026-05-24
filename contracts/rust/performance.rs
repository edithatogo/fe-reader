//! Fe Reader performance and telemetry contracts.
//! These types are intentionally core-safe: no platform profiler or UI types appear here.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfScenarioId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerfBudgetClass {
    InteractiveP0,
    WorkflowP1,
    BatchP2,
    FrontierP3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfBudget {
    pub scenario_id: PerfScenarioId,
    pub class: PerfBudgetClass,
    pub max_p50_ms: Option<f64>,
    pub max_p95_ms: Option<f64>,
    pub max_p99_ms: Option<f64>,
    pub max_peak_rss_mb: Option<f64>,
    pub max_output_bytes: Option<u64>,
    pub hard_gate_wave: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfSample {
    pub scenario_id: PerfScenarioId,
    pub git_rev: String,
    pub platform: String,
    pub target_triple: String,
    pub build_profile: String,
    pub fixture_id: String,
    pub p50_ms: Option<f64>,
    pub p95_ms: Option<f64>,
    pub p99_ms: Option<f64>,
    pub peak_rss_mb: Option<f64>,
    pub output_bytes: Option<u64>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfGateResult {
    pub scenario_id: PerfScenarioId,
    pub passed: bool,
    pub budget: PerfBudget,
    pub sample: PerfSample,
    pub regression_percent: Option<f64>,
    pub requires_human_review: bool,
}

pub trait PerfRecorder: Send + Sync {
    fn record_sample(&self, sample: PerfSample) -> anyhow::Result<()>;
    fn evaluate_budget(&self, sample: PerfSample, budget: PerfBudget) -> anyhow::Result<PerfGateResult>;
}
