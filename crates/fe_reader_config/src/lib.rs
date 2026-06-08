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
}
