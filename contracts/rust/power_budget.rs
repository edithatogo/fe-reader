//! Power, thermal and responsiveness policy contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePowerPolicy {
    pub mode: PowerMode,
    pub max_parallel_jobs: u16,
    pub max_render_threads: u16,
    pub tile_prefetch_distance: u16,
    pub pause_background_jobs_on_battery: bool,
    pub reduce_animations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerMode {
    InteractiveFast,
    Balanced,
    BatterySaver,
    BackgroundBatch,
    Benchmark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerBudgetScenario {
    pub scenario_id: String,
    pub platform: String,
    pub mode: PowerMode,
    pub max_peak_memory_mb: Option<u64>,
    pub max_cpu_percent_avg: Option<f32>,
    pub max_gpu_percent_avg: Option<f32>,
    pub max_wall_time_ms: Option<u64>,
    pub notes: Vec<String>,
}
