//! Configuration, feature flags and policy bundle contracts.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Named feature flag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FeatureFlag(pub String);

/// Feature flag set.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureSet {
    /// Enabled flags.
    pub enabled: BTreeSet<FeatureFlag>,
}

/// Diagnostics/telemetry mode controlled by policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryMode {
    /// Diagnostics export is disabled.
    Disabled,
    /// Diagnostics stay local.
    LocalOnly,
    /// Diagnostics can be bundled only after user approval.
    UserApprovedBundleOnly,
    /// Managed upload is allowed by enterprise policy.
    ManagedUploadAllowed,
}

impl TelemetryMode {
    /// Returns the relative restriction strength of the telemetry mode.
    #[must_use]
    fn restriction_score(self) -> u8 {
        match self {
            Self::Disabled => 0,
            Self::LocalOnly => 1,
            Self::UserApprovedBundleOnly => 2,
            Self::ManagedUploadAllowed => 3,
        }
    }

    /// Returns the more restrictive of two telemetry modes.
    #[must_use]
    fn more_restrictive(self, other: Self) -> Self {
        if self.restriction_score() <= other.restriction_score() {
            self
        } else {
            other
        }
    }
}

/// Enterprise policy matching `schemas/enterprise-policy.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterprisePolicy {
    /// Policy version.
    pub policy_version: String,
    /// Whether policy is managed by an administrator.
    pub managed: bool,
    /// Disabled integration surfaces.
    #[serde(default)]
    pub disabled_surfaces: Vec<String>,
    /// Disabled feature ids.
    #[serde(default)]
    pub disabled_features: Vec<String>,
    /// Whether plugins are allowed.
    #[serde(default)]
    pub allow_plugins: bool,
    /// Whether MCP is allowed.
    #[serde(default)]
    pub allow_mcp: bool,
    /// Whether external converters are allowed.
    #[serde(default)]
    pub allow_external_converters: bool,
    /// Whether redaction verification is mandatory.
    #[serde(default)]
    pub require_redaction_verification: bool,
    /// Whether share/export flows require metadata-clean prompts.
    #[serde(default)]
    pub require_metadata_clean_share_prompt: bool,
    /// Diagnostics policy.
    pub telemetry_mode: TelemetryMode,
    /// Allowed update channels.
    #[serde(default)]
    pub allowed_update_channels: Vec<String>,
}

/// Optional frontier feature category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontierFeatureCategory {
    /// Optional local intelligence such as NLP or RAG suggestions.
    LocalIntelligence,
    /// Optional GPU/vector acceleration.
    GpuAcceleration,
    /// Optional PGO/BOLT/toolchain optimization.
    ToolchainOptimization,
    /// Optional transformation pass beyond accepted baseline behavior.
    TransformationPass,
}

/// Evidence required before a frontier feature can be promoted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierEvidenceRequirement {
    /// Required evidence source.
    pub source: String,
    /// Human-readable acceptance criterion.
    pub acceptance_criterion: String,
}

/// Feature-gated frontier policy entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierFeaturePolicy {
    /// Feature flag id.
    pub flag_id: String,
    /// Feature category.
    pub category: FrontierFeatureCategory,
    /// Whether the feature is enabled by default.
    pub default_enabled: bool,
    /// Whether a policy check is required before enablement.
    pub requires_policy_check: bool,
    /// Whether suggestions from this feature must cite evidence.
    pub requires_evidence_citations: bool,
    /// Whether high-risk mutation is forbidden without the normal mutation pipeline.
    pub forbids_high_risk_auto_mutation: bool,
    /// Evidence required before promotion.
    pub evidence_requirements: Vec<FrontierEvidenceRequirement>,
}

impl FrontierFeaturePolicy {
    /// Returns true when the frontier feature is advisory and safe by default.
    #[must_use]
    pub fn is_safe_default(&self) -> bool {
        !self.default_enabled
            && self.requires_policy_check
            && self.requires_evidence_citations
            && self.forbids_high_risk_auto_mutation
            && !self.evidence_requirements.is_empty()
    }
}

/// Wave 6 optional frontier policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierPolicy {
    /// Policy version.
    pub policy_version: String,
    /// Frontier features.
    pub features: Vec<FrontierFeaturePolicy>,
}

impl FrontierPolicy {
    /// Returns the conservative Wave 6 frontier policy.
    #[must_use]
    pub fn wave6_default() -> Self {
        Self {
            policy_version: "0.1".to_string(),
            features: vec![
                FrontierFeaturePolicy {
                    flag_id: "frontier.local_intelligence.suggestions".to_string(),
                    category: FrontierFeatureCategory::LocalIntelligence,
                    default_enabled: false,
                    requires_policy_check: true,
                    requires_evidence_citations: true,
                    forbids_high_risk_auto_mutation: true,
                    evidence_requirements: vec![FrontierEvidenceRequirement {
                        source: "accepted compatibility corpus or user-selected local evidence"
                            .to_string(),
                        acceptance_criterion:
                            "suggestions cite extracted local evidence and never mutate automatically"
                                .to_string(),
                    }],
                },
                FrontierFeaturePolicy {
                    flag_id: "frontier.render.gpu_compositor".to_string(),
                    category: FrontierFeatureCategory::GpuAcceleration,
                    default_enabled: false,
                    requires_policy_check: true,
                    requires_evidence_citations: true,
                    forbids_high_risk_auto_mutation: true,
                    evidence_requirements: vec![FrontierEvidenceRequirement {
                        source: "visual regression and performance reports".to_string(),
                        acceptance_criterion:
                            "no accepted visual regressions and measured P0/P1 performance win"
                                .to_string(),
                    }],
                },
                FrontierFeaturePolicy {
                    flag_id: "frontier.toolchain.pgo_bolt".to_string(),
                    category: FrontierFeatureCategory::ToolchainOptimization,
                    default_enabled: false,
                    requires_policy_check: true,
                    requires_evidence_citations: true,
                    forbids_high_risk_auto_mutation: true,
                    evidence_requirements: vec![FrontierEvidenceRequirement {
                        source: "benchmark budget report".to_string(),
                        acceptance_criterion:
                            "startup, memory and binary-size budgets do not regress".to_string(),
                    }],
                },
            ],
        }
    }

    /// Returns true when every Wave 6 frontier feature is disabled and evidence-gated.
    #[must_use]
    pub fn all_features_are_safe_defaults(&self) -> bool {
        self.features
            .iter()
            .all(FrontierFeaturePolicy::is_safe_default)
    }
}

impl EnterprisePolicy {
    /// Returns a conservative managed policy that disables risky integrations.
    #[must_use]
    pub fn managed_lockdown() -> Self {
        Self {
            policy_version: "0.1".to_string(),
            managed: true,
            disabled_surfaces: vec![
                "mcp".to_string(),
                "plugins".to_string(),
                "external_converters".to_string(),
                "web_postmessage".to_string(),
                "native_automation_apply".to_string(),
            ],
            disabled_features: vec!["network".to_string(), "local_ml".to_string()],
            allow_plugins: false,
            allow_mcp: false,
            allow_external_converters: false,
            require_redaction_verification: true,
            require_metadata_clean_share_prompt: true,
            telemetry_mode: TelemetryMode::LocalOnly,
            allowed_update_channels: vec!["stable".to_string(), "lts-enterprise".to_string()],
        }
    }

    /// Applies a managed enterprise policy over a lower-precedence policy layer.
    ///
    /// Security-sensitive booleans are combined conservatively so that `false`
    /// in either layer remains `false`. Deny-lists are unioned, and managed
    /// allow-lists replace any lower-precedence allow-list.
    #[must_use]
    pub fn effective_overrides(&self, lower_precedence: &Self) -> Self {
        if !self.managed {
            return lower_precedence.clone();
        }

        let disabled_surfaces = lower_precedence
            .disabled_surfaces
            .iter()
            .chain(self.disabled_surfaces.iter())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let disabled_features = lower_precedence
            .disabled_features
            .iter()
            .chain(self.disabled_features.iter())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let allowed_update_channels = if self.allowed_update_channels.is_empty() {
            lower_precedence.allowed_update_channels.clone()
        } else {
            self.allowed_update_channels.clone()
        };

        Self {
            policy_version: self.policy_version.clone(),
            managed: true,
            disabled_surfaces,
            disabled_features,
            allow_plugins: lower_precedence.allow_plugins && self.allow_plugins,
            allow_mcp: lower_precedence.allow_mcp && self.allow_mcp,
            allow_external_converters: lower_precedence.allow_external_converters
                && self.allow_external_converters,
            require_redaction_verification: lower_precedence.require_redaction_verification
                || self.require_redaction_verification,
            require_metadata_clean_share_prompt: lower_precedence
                .require_metadata_clean_share_prompt
                || self.require_metadata_clean_share_prompt,
            telemetry_mode: lower_precedence
                .telemetry_mode
                .more_restrictive(self.telemetry_mode),
            allowed_update_channels,
        }
    }

    /// Returns true if the named integration surface is disabled.
    #[must_use]
    pub fn disables_surface(&self, surface: &str) -> bool {
        self.disabled_surfaces
            .iter()
            .any(|disabled| disabled == surface)
    }

    /// Returns true if risky integrations are disabled.
    #[must_use]
    pub fn disables_risky_integrations(&self) -> bool {
        !self.allow_plugins
            && !self.allow_mcp
            && !self.allow_external_converters
            && self.disables_surface("web_postmessage")
            && self.disables_surface("native_automation_apply")
    }
}

impl FeatureSet {
    /// Returns true if the flag is enabled.
    #[must_use]
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.enabled.contains(&FeatureFlag(flag.to_string()))
    }

    /// Enables a flag.
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.enabled.insert(FeatureFlag(flag.into()));
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
    fn feature_flags_are_deterministic() {
        let mut set = FeatureSet::default();
        set.enable("render.gpu.experimental");
        assert!(set.is_enabled("render.gpu.experimental"));
        assert!(!set.is_enabled("ml.local_ner"));
    }

    #[test]
    fn enterprise_policy_can_disable_risky_integrations() {
        let policy = EnterprisePolicy::managed_lockdown();
        assert!(policy.managed);
        assert!(policy.disables_risky_integrations());
        assert!(policy.require_redaction_verification);
        assert_eq!(policy.telemetry_mode, TelemetryMode::LocalOnly);
    }

    #[test]
    fn managed_enterprise_policy_overrides_permissive_lower_layers() {
        let lower = EnterprisePolicy {
            policy_version: "0.1".to_string(),
            managed: false,
            disabled_surfaces: vec!["legacy-websocket".to_string()],
            disabled_features: vec!["experimental-cache".to_string()],
            allow_plugins: true,
            allow_mcp: true,
            allow_external_converters: true,
            require_redaction_verification: false,
            require_metadata_clean_share_prompt: false,
            telemetry_mode: TelemetryMode::ManagedUploadAllowed,
            allowed_update_channels: vec!["beta".to_string(), "nightly".to_string()],
        };
        let effective = EnterprisePolicy::managed_lockdown().effective_overrides(&lower);

        assert!(effective.managed);
        assert!(!effective.allow_plugins);
        assert!(!effective.allow_mcp);
        assert!(!effective.allow_external_converters);
        assert!(effective.require_redaction_verification);
        assert!(effective.require_metadata_clean_share_prompt);
        assert_eq!(effective.telemetry_mode, TelemetryMode::LocalOnly);
        assert!(effective.disables_surface("mcp"));
        assert!(effective.disables_surface("plugins"));
        assert!(effective.disables_surface("legacy-websocket"));
        assert!(
            effective
                .disabled_features
                .iter()
                .any(|feature| feature == "local_ml")
        );
        assert!(
            effective
                .disabled_features
                .iter()
                .any(|feature| feature == "experimental-cache")
        );
        assert!(
            effective
                .allowed_update_channels
                .iter()
                .any(|channel| channel == "stable")
        );
        assert!(
            effective
                .allowed_update_channels
                .iter()
                .any(|channel| channel == "lts-enterprise")
        );
        assert!(
            !effective
                .allowed_update_channels
                .iter()
                .any(|channel| channel == "beta")
        );
        assert!(
            !effective
                .allowed_update_channels
                .iter()
                .any(|channel| channel == "nightly")
        );
    }

    #[test]
    fn wave6_frontier_features_are_optional_and_evidence_gated() {
        let policy = FrontierPolicy::wave6_default();
        assert!(policy.all_features_are_safe_defaults());
        assert!(
            policy.features.iter().any(|feature| matches!(
                feature.category,
                FrontierFeatureCategory::LocalIntelligence
            ))
        );
        assert!(
            policy
                .features
                .iter()
                .all(|feature| feature.forbids_high_risk_auto_mutation)
        );
    }
}
