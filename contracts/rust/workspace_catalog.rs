//! Local workspace/catalog contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecord {
    pub workspace_id: String,
    pub display_name: String,
    pub local_path_hint: Option<String>,
    pub indexing_enabled: bool,
    pub policy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCatalogRecord {
    pub document_id: String,
    pub workspace_id: String,
    pub display_title: String,
    pub document_sha256: String,
    pub last_opened_utc: Option<String>,
    pub tags: Vec<String>,
    pub metadata_profile: Option<String>,
    pub indexing_state: String,
}

pub trait WorkspaceCatalog: Send + Sync {
    fn upsert_workspace(&self, workspace: WorkspaceRecord) -> anyhow::Result<()>;
    fn upsert_document(&self, document: DocumentCatalogRecord) -> anyhow::Result<()>;
    fn remove_document(&self, document_id: &str) -> anyhow::Result<()>;
    fn list_recent_documents(&self, workspace_id: Option<&str>, limit: u32) -> anyhow::Result<Vec<DocumentCatalogRecord>>;
}
