//! Metadata planning contracts.
//!
//! Wave 0 does not mutate PDF bytes. It creates explicit plans for metadata editing/scrubbing.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{OperationIntent, PatchOperation, PatchPlan};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Basic document info dictionary fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentInfo {
    /// Title.
    pub title: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Subject.
    pub subject: Option<String>,
    /// Keywords.
    pub keywords: Vec<String>,
    /// Creator application.
    pub creator: Option<String>,
    /// Producer application.
    pub producer: Option<String>,
}

/// Metadata scrub strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataScrubMode {
    /// Preserve all metadata.
    Preserve,
    /// Remove common private fields but keep user-visible title/subject.
    CleanShare,
    /// Remove all non-essential metadata.
    Aggressive,
}

/// Metadata operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum MetadataOperation {
    /// Set a document-info field.
    SetInfoField {
        /// Field name.
        field: String,
        /// Field value.
        value: String,
    },
    /// Scrub metadata according to a mode.
    Scrub {
        /// Scrub mode.
        mode: MetadataScrubMode,
    },
}

/// Plans metadata operations as a core patch plan.
#[must_use]
pub fn plan_metadata_operations(
    intent: &OperationIntent,
    operations: &[MetadataOperation],
) -> PatchPlan {
    let patch_ops = operations
        .iter()
        .map(|operation| match operation {
            MetadataOperation::SetInfoField { field, value } => PatchOperation::SetMetadata {
                key: field.clone(),
                value: value.clone(),
            },
            MetadataOperation::Scrub { mode } => PatchOperation::SetMetadata {
                key: "metadata_scrub_mode".to_string(),
                value: format!("{mode:?}"),
            },
        })
        .collect();
    PatchPlan::draft(intent, "metadata operations", patch_ops)
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fe_reader_core::{DocumentId, OperationKind, OperationSource};

    #[test]
    fn metadata_plan_is_not_auto_approved() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "metadata",
        );
        let plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::SetInfoField {
                field: "title".into(),
                value: "Fe".into(),
            }],
        );
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.operations.len(), 1);
    }
}
