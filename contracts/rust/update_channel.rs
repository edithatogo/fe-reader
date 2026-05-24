//! Release channel and signed update manifest contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReleaseChannel {
    Nightly,
    Alpha,
    Beta,
    Stable,
    LtsEnterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUpdateArtifact {
    pub platform: String,
    pub arch: String,
    pub installer_kind: String,
    pub url: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUpdateManifest {
    pub manifest_version: String,
    pub app_version: String,
    pub channel: ReleaseChannel,
    pub min_supported_version: Option<String>,
    pub artifacts: Vec<SignedUpdateArtifact>,
    pub release_notes_url: Option<String>,
    pub rollback: bool,
    pub manifest_signature: String,
}

pub trait UpdateVerifier: Send + Sync {
    fn verify_manifest(&self, manifest: &SignedUpdateManifest) -> anyhow::Result<()>;
    fn verify_artifact_digest(&self, artifact: &SignedUpdateArtifact, bytes: &[u8]) -> anyhow::Result<()>;
}
