//! Privacy-preserving diagnostics contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSensitivity {
    SafeOperational,
    ContainsPath,
    ContainsDocumentTitle,
    ContainsDocumentText,
    ContainsRenderedImage,
    ContainsSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticEvent {
    pub event_id: String,
    pub event_kind: String,
    pub timestamp_utc: String,
    pub sensitivities: Vec<DiagnosticSensitivity>,
    pub payload_json: serde_json::Value,
}

pub trait DiagnosticsSink: Send + Sync {
    fn record(&self, event: DiagnosticEvent) -> anyhow::Result<()>;
    fn build_support_bundle(&self, request_json: serde_json::Value) -> anyhow::Result<Vec<u8>>;
}
