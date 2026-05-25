//! Contract sketch for release evidence bundles.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseEvidenceBundle {
    pub release_id: String,
    pub channel: ReleaseChannel,
    pub source_commit: String,
    pub source_tag: Option<String>,
    pub toolchain: String,
    pub artifacts: Vec<ReleaseArtifactEvidence>,
    pub sbom_path: Option<String>,
    pub provenance_path: Option<String>,
    pub audit_reports: Vec<String>,
    pub compatibility_report: Option<String>,
    pub performance_report: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReleaseChannel { Dev, Nightly, Preview, Beta, Stable, Lts, StoreSubmission }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseArtifactEvidence {
    pub artifact_name: String,
    pub platform: String,
    pub sha256: String,
    pub signature_path: Option<String>,
    pub notarization_or_store_receipt: Option<String>,
}
