//! Metadata operation contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataMode {
    View,
    Edit,
    Diff,
    CleanShare,
    ForensicPreserve,
    ProvenancePreview,
    ProvenanceAuthor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSnapshot {
    pub document_info: serde_json::Value,
    pub xmp_packet: Option<String>,
    pub embedded_files: Vec<serde_json::Value>,
    pub output_intents: Vec<serde_json::Value>,
    pub signatures: Vec<serde_json::Value>,
    pub conformance_claims: Vec<String>,
    pub warnings: Vec<String>,
}

pub trait MetadataProvider: Send + Sync {
    fn snapshot(&self, document_id: &str) -> anyhow::Result<MetadataSnapshot>;
    fn plan_metadata_operation(&self, document_id: &str, mode: MetadataMode, payload: serde_json::Value) -> anyhow::Result<serde_json::Value>;
}
