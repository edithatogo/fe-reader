//! Security policy contracts for Fe Reader automation and high-risk operations.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{OperationSource, RiskLevel};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Security-relevant action category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    /// Read or inspect only.
    Read,
    /// Plan a mutation without applying it.
    PlanMutation,
    /// Apply a document mutation.
    ApplyMutation,
    /// Export, share, print, or otherwise release document-derived bytes.
    Export,
    /// Execute an external executable such as Pandoc, Typst, Quarto or qpdf.
    RunExternalTool,
    /// Execute or accept automation from COM, AppleScript, D-Bus, intents, MCP or local API.
    UseAutomation,
    /// Load a WASM plugin or workflow package.
    LoadPlugin,
    /// Network access.
    NetworkAccess,
}

/// Security policy configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Whether mutation via automation may skip explicit review.
    pub allow_automation_mutation_without_review: bool,
    /// Whether external tools may be run.
    pub allow_external_tools: bool,
    /// Whether plugins may be loaded.
    pub allow_plugins: bool,
    /// Whether network access is allowed.
    pub allow_network: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            allow_automation_mutation_without_review: false,
            allow_external_tools: false,
            allow_plugins: false,
            allow_network: false,
        }
    }
}

/// Policy evaluation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    /// Whether the action is allowed at all.
    pub allowed: bool,
    /// Whether explicit human approval is required before apply.
    pub requires_review: bool,
    /// Human-readable reason.
    pub reason: String,
}

impl PolicyDecision {
    /// Allows the operation.
    #[must_use]
    pub fn allow(reason: impl Into<String>, requires_review: bool) -> Self {
        Self { allowed: true, requires_review, reason: reason.into() }
    }

    /// Denies the operation.
    #[must_use]
    pub fn deny(reason: impl Into<String>) -> Self {
        Self { allowed: false, requires_review: true, reason: reason.into() }
    }
}

/// Evaluates a security policy for a source/action/risk tuple.
#[must_use]
pub fn evaluate_policy(
    policy: &SecurityPolicy,
    source: OperationSource,
    action: PolicyAction,
    risk: RiskLevel,
) -> PolicyDecision {
    match action {
        PolicyAction::Read => PolicyDecision::allow("read-only action", false),
        PolicyAction::PlanMutation => PolicyDecision::allow("planning is allowed; apply remains gated", true),
        PolicyAction::ApplyMutation => {
            if matches!(source, OperationSource::Automation | OperationSource::Mcp | OperationSource::Plugin | OperationSource::Web)
                && !policy.allow_automation_mutation_without_review
            {
                return PolicyDecision::allow("automation mutation requires review", true);
            }
            PolicyDecision::allow("mutation allowed by policy", risk.normally_requires_review())
        }
        PolicyAction::Export => PolicyDecision::allow("export requires review", true),
        PolicyAction::RunExternalTool => {
            if policy.allow_external_tools {
                PolicyDecision::allow("external tools allowed by policy", true)
            } else {
                PolicyDecision::deny("external tools are disabled")
            }
        }
        PolicyAction::UseAutomation => PolicyDecision::allow("automation is read-only by default", risk.normally_requires_review()),
        PolicyAction::LoadPlugin => {
            if policy.allow_plugins {
                PolicyDecision::allow("plugins allowed by policy", true)
            } else {
                PolicyDecision::deny("plugins are disabled")
            }
        }
        PolicyAction::NetworkAccess => {
            if policy.allow_network {
                PolicyDecision::allow("network access allowed by policy", true)
            } else {
                PolicyDecision::deny("network access is disabled")
            }
        }
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
    fn read_is_allowed_without_review() {
        let decision = evaluate_policy(&SecurityPolicy::default(), OperationSource::Cli, PolicyAction::Read, RiskLevel::ReadOnly);
        assert!(decision.allowed);
        assert!(!decision.requires_review);
    }

    #[test]
    fn plugins_are_denied_by_default() {
        let decision = evaluate_policy(&SecurityPolicy::default(), OperationSource::Plugin, PolicyAction::LoadPlugin, RiskLevel::HighRisk);
        assert!(!decision.allowed);
    }

    #[test]
    fn automation_mutation_requires_review_by_default() {
        let decision = evaluate_policy(&SecurityPolicy::default(), OperationSource::Mcp, PolicyAction::ApplyMutation, RiskLevel::HighRisk);
        assert!(decision.allowed);
        assert!(decision.requires_review);
    }
}
