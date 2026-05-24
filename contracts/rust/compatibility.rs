//! Compatibility corpus and visual regression contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureDescriptor {
    pub fixture_id: String,
    pub path: String,
    pub license: FixtureLicense,
    pub redistribution: RedistributionStatus,
    pub pdf_version: Option<String>,
    pub expected_features: Vec<String>,
    pub risk_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixtureLicense {
    PublicDomain,
    Cc0,
    CcBy,
    ProjectGenerated,
    PrivateNotCommitted,
    UnknownRequiresReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RedistributionStatus {
    CommitAllowed,
    GeneratedOnly,
    PrivateLocalOnly,
    Forbidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualRegressionResult {
    pub fixture_id: String,
    pub page_index: u32,
    pub compared: bool,
    pub max_delta: f32,
    pub changed_pixels: u64,
    pub status: RegressionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionStatus {
    Pass,
    AcceptedDifference,
    FailNeedsReview,
}
