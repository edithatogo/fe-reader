//! Job, progress, cancellation and resource-limit contracts.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use uuid::Uuid;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Job id.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(pub String);

impl JobId {
    /// Creates a new job id.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

/// Job state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    /// Queued.
    Queued,
    /// Running.
    Running,
    /// Completed.
    Completed,
    /// Cancelled.
    Cancelled,
    /// Failed.
    Failed,
}

/// Progress event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressEvent {
    /// Job id.
    pub job_id: JobId,
    /// State.
    pub state: JobState,
    /// Completed work units.
    pub completed_units: u64,
    /// Total work units if known.
    pub total_units: Option<u64>,
    /// Human-readable message.
    pub message: String,
}

/// Schema-compatible job run state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobRunState {
    /// Queued.
    Queued,
    /// Running.
    Running,
    /// Paused.
    Paused,
    /// Cancellation requested.
    Cancelling,
    /// Cancelled.
    Cancelled,
    /// Completed.
    Completed,
    /// Failed.
    Failed,
}

/// Schema-compatible job progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobProgress {
    /// Completed work units.
    pub completed_units: u64,
    /// Total work units, if known.
    pub total_units: Option<u64>,
    /// Human-readable progress message.
    pub message: String,
}

impl JobProgress {
    /// Returns true when completed work does not exceed known total work.
    #[must_use]
    pub fn is_consistent(&self) -> bool {
        self.total_units
            .is_none_or(|total_units| self.completed_units <= total_units)
    }
}

/// Schema-compatible job run summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobRun {
    /// Job id.
    pub job_id: String,
    /// Job kind.
    pub kind: String,
    /// Job state.
    pub state: JobRunState,
    /// Progress.
    pub progress: JobProgress,
    /// Resource limits or policy notes.
    pub resource_limits: BTreeMap<String, serde_json::Value>,
}

impl JobRun {
    /// Creates a completed smoke job.
    #[must_use]
    pub fn smoke_completed() -> Self {
        Self {
            job_id: "job:wave0-smoke".to_string(),
            kind: "contract_smoke".to_string(),
            state: JobRunState::Completed,
            progress: JobProgress {
                completed_units: 1,
                total_units: Some(1),
                message: "contract smoke completed".to_string(),
            },
            resource_limits: BTreeMap::from([
                ("max_wall_time_ms".to_string(), serde_json::json!(30_000)),
                ("max_memory_mib".to_string(), serde_json::json!(1024)),
            ]),
        }
    }

    /// Validates job-run invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when ids/kinds are empty or progress is inconsistent.
    pub fn validate(&self) -> Result<(), JobError> {
        if self.job_id.trim().is_empty() {
            return Err(JobError::invalid("job_id must not be empty"));
        }
        if self.kind.trim().is_empty() {
            return Err(JobError::invalid("kind must not be empty"));
        }
        if !self.progress.is_consistent() {
            return Err(JobError::invalid(
                "completed_units must not exceed total_units",
            ));
        }
        Ok(())
    }
}

/// Job contract validation error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("job contract error: {message}")]
pub struct JobError {
    /// Human-readable validation message.
    pub message: String,
}

impl JobError {
    fn invalid(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Cooperative cancellation token.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Returns true if cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

/// Deterministic visible-page prefetch plan for scheduling and benchmark probes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisiblePagePrefetchPlan {
    /// Zero-based page index that is currently visible.
    pub visible_page: u32,
    /// Total page count available to the planner.
    pub page_count: u32,
    /// Prefetch radius used around the visible page.
    pub prefetch_radius: u32,
    /// Page indices selected for prefetch.
    pub prefetch_pages: Vec<u32>,
    /// Number of cancellation checks performed while building the plan.
    pub cancellation_checks: usize,
}

/// Plans a bounded set of pages to prefetch around a visible page.
#[must_use]
pub fn plan_visible_page_prefetch(
    visible_page: u32,
    page_count: u32,
    prefetch_radius: u32,
) -> VisiblePagePrefetchPlan {
    if page_count == 0 {
        return VisiblePagePrefetchPlan {
            visible_page,
            page_count,
            prefetch_radius,
            prefetch_pages: Vec::new(),
            cancellation_checks: 0,
        };
    }

    let visible_page = visible_page.min(page_count.saturating_sub(1));
    let start = visible_page.saturating_sub(prefetch_radius);
    let end = visible_page
        .saturating_add(prefetch_radius)
        .min(page_count.saturating_sub(1));
    let prefetch_pages = (start..=end)
        .filter(|page_index| *page_index != visible_page)
        .collect::<Vec<_>>();
    let cancellation_checks = prefetch_pages.len() + 1;

    VisiblePagePrefetchPlan {
        visible_page,
        page_count,
        prefetch_radius,
        prefetch_pages,
        cancellation_checks,
    }
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_token_is_cooperative() {
        let token = CancellationToken::default();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn smoke_job_run_validates() {
        let run = JobRun::smoke_completed();
        run.validate().unwrap();
        assert_eq!(run.state, JobRunState::Completed);
    }

    #[test]
    fn job_run_rejects_inconsistent_progress() {
        let mut run = JobRun::smoke_completed();
        run.progress.completed_units = 2;
        assert!(run.validate().is_err());
    }

    #[test]
    fn prefetch_plan_keeps_visible_page_out_of_prefetch_window() {
        let plan = plan_visible_page_prefetch(5, 12, 2);
        assert_eq!(plan.visible_page, 5);
        assert_eq!(plan.prefetch_pages, vec![3, 4, 6, 7]);
        assert_eq!(plan.cancellation_checks, 5);
    }

    #[test]
    fn prefetch_plan_handles_empty_document() {
        let plan = plan_visible_page_prefetch(0, 0, 3);
        assert!(plan.prefetch_pages.is_empty());
        assert_eq!(plan.cancellation_checks, 0);
    }
}
