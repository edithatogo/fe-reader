//! Contract sketch for compile-time and runtime capability governance.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCapability {
    pub feature_id: String,
    pub owner: String,
    pub maturity: FeatureMaturity,
    pub compile_time_flag: Option<String>,
    pub runtime_available: bool,
    pub policy_rule: Option<String>,
    pub disabled_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureMaturity {
    CoreDefault,
    PlatformOptional,
    EnterprisePolicyControlled,
    FrontierExperimental,
    SecuritySensitive,
    StoreRestricted,
}

pub trait CapabilityRegistry {
    fn get(&self, feature_id: &str) -> Option<FeatureCapability>;
    fn list(&self) -> Vec<FeatureCapability>;
    fn require(&self, feature_id: &str) -> Result<FeatureCapability, CapabilityError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CapabilityError {
    #[error("feature is unavailable: {0}")]
    Unavailable(String),
    #[error("feature is blocked by policy: {0}")]
    PolicyBlocked(String),
}
