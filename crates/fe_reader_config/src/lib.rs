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
}
