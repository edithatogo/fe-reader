//! Content-addressed cache and workspace contracts.

#[derive(Debug, Clone)]
pub struct CacheKey {
    pub namespace: String,
    pub document_sha256: String,
    pub operation_version: String,
    pub settings_hash: String,
}

#[derive(Debug, Clone)]
pub struct CacheEntryMeta {
    pub key: CacheKey,
    pub bytes: u64,
    pub created_at_utc: String,
    pub expires_at_utc: Option<String>,
    pub privacy_class: String,
}

#[derive(Debug, Clone)]
pub struct WorkspaceRecord {
    pub workspace_id: String,
    pub document_sha256: String,
    pub display_name: String,
    pub source_uri_hint: Option<String>,
    pub sidecars: Vec<String>,
    pub cache_entries: Vec<CacheKey>,
}

pub trait CacheStore: Send + Sync {
    fn get(&self, key: &CacheKey) -> Result<Option<Vec<u8>>, String>;
    fn put(&self, key: CacheKey, value: &[u8], privacy_class: &str) -> Result<CacheEntryMeta, String>;
    fn remove(&self, key: &CacheKey) -> Result<(), String>;
}

pub trait WorkspaceCatalog: Send + Sync {
    fn upsert(&self, record: WorkspaceRecord) -> Result<(), String>;
    fn forget(&self, document_sha256: &str) -> Result<(), String>;
    fn list_recent(&self, limit: usize) -> Result<Vec<WorkspaceRecord>, String>;
}
