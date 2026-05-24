//! Privacy-preserving diagnostics and quality signal contracts.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualitySignalKind {
    CrashSummary,
    PerformanceCounter,
    CompatibilityFailure,
    SecurityPolicyDenial,
    AccessibilityFinding,
    UserSubmittedSupportBundle,
}

#[derive(Debug, Clone)]
pub struct QualitySignal {
    pub signal_id: String,
    pub kind: QualitySignalKind,
    pub created_at_utc: String,
    pub app_version: String,
    pub platform: String,
    pub redacted_payload_json: String,
}

pub trait QualitySignalSink: Send + Sync {
    fn record_local(&self, signal: QualitySignal) -> Result<(), String>;
    fn export_bundle(&self, bundle_id: &str, out_path: &str) -> Result<(), String>;
}
