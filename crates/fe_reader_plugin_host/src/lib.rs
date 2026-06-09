//! Proposal-only plugin host contract.
//!
//! Wave 5 plugins may declare capabilities and propose reviewable plans. The host does
//! not load WASM, expose audit receipts, or apply document mutations in this crate.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentId, FeError, FeErrorKind, OperationIntent, OperationKind, OperationSource, PatchPlan,
};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin manifest matching `schemas/plugin-manifest.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Stable plugin id.
    pub plugin_id: String,
    /// Plugin semantic version.
    pub plugin_version: String,
    /// Fe Reader plugin API version.
    pub fe_plugin_api: String,
    /// Publisher name.
    pub publisher: String,
    /// License expression.
    pub license: String,
    /// Requested capability names.
    pub capabilities: Vec<String>,
    /// Whether the plugin requests network access.
    pub network_access: bool,
    /// Whether the plugin requests filesystem access outside approved document handles.
    pub filesystem_access: bool,
    /// Optional plugin artifact digest.
    pub sha256: Option<String>,
    /// Optional plugin signature.
    pub signature: Option<String>,
}

impl PluginManifest {
    /// Validates proposal-only policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the manifest is empty or requests risky access.
    pub fn validate_proposal_only(&self) -> Result<(), FeError> {
        for (field, value) in [
            ("plugin_id", &self.plugin_id),
            ("plugin_version", &self.plugin_version),
            ("fe_plugin_api", &self.fe_plugin_api),
            ("publisher", &self.publisher),
            ("license", &self.license),
        ] {
            if value.trim().is_empty() {
                return Err(FeError::new(
                    FeErrorKind::InvalidInput,
                    format!("plugin manifest field {field} is required"),
                ));
            }
        }
        if self.network_access || self.filesystem_access {
            return Err(FeError::new(
                FeErrorKind::PolicyDenied,
                "Wave 5 proposal-only plugins cannot request network or filesystem access",
            ));
        }
        Ok(())
    }
}

/// Plugin proposal request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginProposalRequest {
    /// Plugin id.
    pub plugin_id: String,
    /// Target document id.
    pub document_id: DocumentId,
    /// Caller-known document hash.
    pub document_sha256: String,
    /// Requested operation.
    pub operation: String,
    /// Operation input.
    #[serde(default)]
    pub input_json: serde_json::Value,
}

/// Proposal-only plugin response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginProposal {
    /// Plugin id.
    pub plugin_id: String,
    /// Intent created by the plugin host.
    pub intent: OperationIntent,
    /// Reviewable patch plan.
    pub patch_plan: PatchPlan,
    /// Non-mutating annotations or diagnostics proposed by the plugin.
    pub annotations_json: serde_json::Value,
    /// Warnings to show before review.
    pub warnings: Vec<String>,
}

/// Proposal-only plugin host.
#[derive(Debug, Clone)]
pub struct ProposalOnlyPluginHost {
    manifest: PluginManifest,
}

impl ProposalOnlyPluginHost {
    /// Creates a proposal-only host from a manifest.
    ///
    /// # Errors
    ///
    /// Returns an error when the manifest violates proposal-only policy.
    pub fn new(manifest: PluginManifest) -> Result<Self, FeError> {
        manifest.validate_proposal_only()?;
        Ok(Self { manifest })
    }

    /// Produces a reviewable proposal without loading or executing plugin code.
    ///
    /// # Errors
    ///
    /// Returns an error when the request targets a different plugin or omits a document hash.
    pub fn propose(&self, request: PluginProposalRequest) -> Result<PluginProposal, FeError> {
        if request.plugin_id != self.manifest.plugin_id {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "plugin proposal request does not match loaded manifest",
            ));
        }
        if request.document_sha256.trim().is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "plugin proposal requires document hash match evidence",
            ));
        }
        let intent = OperationIntent::mutation(
            OperationSource::Plugin,
            request.document_id,
            OperationKind::PluginProposal,
            format!("plugin_proposal:{}", request.operation),
        );
        let patch_plan = PatchPlan::draft(
            &intent,
            format!("proposal-only plugin plan from {}", self.manifest.plugin_id),
            Vec::new(),
        );
        Ok(PluginProposal {
            plugin_id: self.manifest.plugin_id.clone(),
            intent,
            patch_plan,
            annotations_json: serde_json::json!({ "proposal_only": true }),
            warnings: vec![
                "plugin host is proposal-only; user and policy review are required before apply"
                    .to_string(),
            ],
        })
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

    fn manifest() -> PluginManifest {
        PluginManifest {
            plugin_id: "org.fereader.sample".to_string(),
            plugin_version: "0.1.0".to_string(),
            fe_plugin_api: "0.1-preview".to_string(),
            publisher: "Fe Reader Project".to_string(),
            license: "Apache-2.0 OR MIT".to_string(),
            capabilities: vec!["propose_annotations".to_string()],
            network_access: false,
            filesystem_access: false,
            sha256: None,
            signature: None,
        }
    }

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn rejects_risky_plugin_manifest() {
        let mut manifest = manifest();
        manifest.network_access = true;
        let error = manifest
            .validate_proposal_only()
            .expect_err("policy denial");
        assert_eq!(error.kind, FeErrorKind::PolicyDenied);
    }

    #[test]
    fn plugin_host_returns_unapproved_proposal_only_plan() {
        let host = ProposalOnlyPluginHost::new(manifest()).expect("host");
        let proposal = host
            .propose(PluginProposalRequest {
                plugin_id: "org.fereader.sample".to_string(),
                document_id: DocumentId("doc-plugin".to_string()),
                document_sha256: "abc123".to_string(),
                operation: "annotate".to_string(),
                input_json: serde_json::json!({}),
            })
            .expect("proposal");
        assert_eq!(proposal.intent.source, OperationSource::Plugin);
        assert!(!proposal.patch_plan.approved_for_apply);
        assert!(proposal.warnings[0].contains("proposal-only"));
    }
}
