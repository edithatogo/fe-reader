//! Signed update manifest contracts.
//!
//! Wave 4 validates manifest structure, artifact digests and release-channel invariants.
//! Cryptographic signature verification is a later release-signing integration.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Release channel for update manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateChannel {
    /// Developer/nightly channel.
    Nightly,
    /// Alpha channel.
    Alpha,
    /// Beta channel.
    Beta,
    /// Stable public channel.
    Stable,
    /// Long-term support enterprise channel.
    LtsEnterprise,
}

impl UpdateChannel {
    fn requires_release_evidence(self) -> bool {
        matches!(self, Self::Beta | Self::Stable | Self::LtsEnterprise)
    }
}

/// One downloadable update artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateArtifact {
    /// Target platform.
    pub platform: String,
    /// Target CPU architecture.
    pub arch: String,
    /// Installer/package kind.
    pub installer_kind: String,
    /// Download URL.
    pub url: String,
    /// Lowercase SHA-256 digest of the artifact.
    pub sha256: String,
    /// Optional artifact size in bytes.
    pub size_bytes: Option<u64>,
    /// Detached artifact signature reference or encoded signature.
    pub signature: String,
    /// Path or URI for provenance evidence.
    pub provenance_path: Option<String>,
    /// Path or URI for signing readiness/receipt evidence.
    pub signing_receipt_path: Option<String>,
}

impl UpdateArtifact {
    fn validate(&self, index: usize, failures: &mut Vec<String>) {
        for (field, value) in [
            ("platform", &self.platform),
            ("arch", &self.arch),
            ("installer_kind", &self.installer_kind),
            ("url", &self.url),
            ("signature", &self.signature),
        ] {
            if value.trim().is_empty() {
                failures.push(format!("artifacts[{index}].{field} is required"));
            }
        }
        if !is_lower_hex_64(&self.sha256) {
            failures.push(format!(
                "artifacts[{index}].sha256 must be a 64-character lowercase hex digest"
            ));
        }
        if self.size_bytes == Some(0) {
            failures.push(format!(
                "artifacts[{index}].size_bytes must be greater than zero"
            ));
        }
    }
}

/// Signed update manifest matching `schemas/update-manifest.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateManifest {
    /// Manifest format version.
    pub manifest_version: String,
    /// Application version being published.
    pub app_version: String,
    /// Release channel.
    pub channel: UpdateChannel,
    /// Oldest app version that can consume this manifest.
    pub min_supported_version: Option<String>,
    /// Whether the manifest represents an approved rollback.
    pub rollback: Option<bool>,
    /// Update artifacts.
    pub artifacts: Vec<UpdateArtifact>,
    /// Signature over the manifest payload.
    pub manifest_signature: String,
    /// Release provenance path.
    pub provenance_path: Option<String>,
    /// Signing readiness evidence path.
    pub signing_readiness_path: Option<String>,
}

impl UpdateManifest {
    /// Parses a manifest from JSON bytes and validates contract invariants.
    ///
    /// # Errors
    ///
    /// Returns an error when JSON is invalid or manifest invariants fail.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, UpdateManifestError> {
        let manifest: Self = serde_json::from_slice(bytes)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validates the update manifest contract.
    ///
    /// # Errors
    ///
    /// Returns all validation failures as a single structured error.
    pub fn validate(&self) -> Result<(), UpdateManifestError> {
        let mut failures = Vec::new();
        for (field, value) in [
            ("manifest_version", &self.manifest_version),
            ("app_version", &self.app_version),
            ("manifest_signature", &self.manifest_signature),
        ] {
            if value.trim().is_empty() {
                failures.push(format!("{field} is required"));
            }
        }
        if self.artifacts.is_empty() {
            failures.push("artifacts must contain at least one artifact".to_string());
        }
        for (index, artifact) in self.artifacts.iter().enumerate() {
            artifact.validate(index, &mut failures);
        }
        if self.channel.requires_release_evidence() {
            if self
                .provenance_path
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                failures.push("release channel requires provenance_path".to_string());
            }
            if self
                .signing_readiness_path
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                failures.push("release channel requires signing_readiness_path".to_string());
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(UpdateManifestError::Validation(failures))
        }
    }

    /// Computes a deterministic SHA-256 digest for canonical JSON serialization.
    ///
    /// # Errors
    ///
    /// Returns a serialization error when the manifest cannot be encoded.
    pub fn canonical_sha256(&self) -> Result<String, UpdateManifestError> {
        let bytes = serde_json::to_vec(self)?;
        let digest = Sha256::digest(&bytes);
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}

/// Update manifest validation error.
#[derive(Debug, Error)]
pub enum UpdateManifestError {
    /// JSON parsing or serialization failed.
    #[error("update manifest JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Manifest contract validation failed.
    #[error("update manifest validation failed: {0:?}")]
    Validation(Vec<String>),
}

/// Returns true when `value` is a lowercase 64-character hex digest.
#[must_use]
pub fn is_lower_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> UpdateArtifact {
        UpdateArtifact {
            platform: "macos".to_string(),
            arch: "arm64".to_string(),
            installer_kind: "dmg".to_string(),
            url: "https://example.invalid/fe-reader.dmg".to_string(),
            sha256: "a".repeat(64),
            size_bytes: Some(42),
            signature: "dev-placeholder-signature".to_string(),
            provenance_path: Some("target/release-evidence/provenance.json".to_string()),
            signing_receipt_path: Some(
                "target/release-evidence/signing-readiness.json".to_string(),
            ),
        }
    }

    fn manifest() -> UpdateManifest {
        UpdateManifest {
            manifest_version: "1".to_string(),
            app_version: "0.1.0".to_string(),
            channel: UpdateChannel::Stable,
            min_supported_version: Some("0.1.0".to_string()),
            rollback: Some(false),
            artifacts: vec![artifact()],
            manifest_signature: "dev-placeholder-manifest-signature".to_string(),
            provenance_path: Some("target/release-evidence/provenance.json".to_string()),
            signing_readiness_path: Some(
                "target/release-evidence/signing-readiness.json".to_string(),
            ),
        }
    }

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn valid_manifest_passes_contract() {
        let manifest = manifest();
        manifest.validate().expect("valid manifest");
        let digest = manifest.canonical_sha256().expect("digest");
        assert!(is_lower_hex_64(&digest));
    }

    #[test]
    fn stable_manifest_requires_release_evidence_paths() {
        let mut manifest = manifest();
        manifest.provenance_path = None;
        manifest.signing_readiness_path = None;
        let error = manifest.validate().expect_err("missing evidence paths");
        assert!(format!("{error}").contains("provenance_path"));
    }

    #[test]
    fn rejects_invalid_artifact_digest() {
        let mut manifest = manifest();
        manifest.artifacts[0].sha256 = "PLACEHOLDER".to_string();
        let error = manifest.validate().expect_err("invalid digest");
        assert!(format!("{error}").contains("sha256"));
    }

    #[test]
    fn parses_schema_shaped_json() {
        let bytes = serde_json::to_vec(&manifest()).expect("json");
        let parsed = UpdateManifest::from_json_bytes(&bytes).expect("parsed");
        assert_eq!(parsed.channel, UpdateChannel::Stable);
    }
}
