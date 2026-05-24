//! Long-running job scheduler contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSpec {
    pub job_id: String,
    pub idempotency_key: Option<String>,
    pub kind: JobKind,
    pub priority: JobPriority,
    pub resource_limits: ResourceLimits,
    pub cancellable: bool,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobKind {
    RenderTileBatch,
    BuildSearchIndex,
    ApplyPatchPlan,
    VerifyRedaction,
    RunPreflight,
    ConvertDocument,
    OcrPages,
    GenerateThumbnails,
    RunWorkflowPack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobPriority { Interactive, Normal, Background, Maintenance }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: Option<u64>,
    pub max_wall_time_ms: Option<u64>,
    pub max_threads: Option<u16>,
    pub allow_network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSnapshot {
    pub job_id: String,
    pub state: JobState,
    pub progress: Progress,
    pub current_stage: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobState { Queued, Running, Paused, Cancelling, Cancelled, Completed, Failed }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress { pub completed_units: u64, pub total_units: Option<u64>, pub label: Option<String> }

pub trait JobScheduler {
    fn submit(&self, job: JobSpec) -> Result<JobSnapshot, JobError>;
    fn snapshot(&self, job_id: &str) -> Result<JobSnapshot, JobError>;
    fn cancel(&self, job_id: &str) -> Result<JobSnapshot, JobError>;
    fn list(&self) -> Result<Vec<JobSnapshot>, JobError>;
}

#[derive(Debug)]
pub struct JobError { pub code: String, pub message: String }
