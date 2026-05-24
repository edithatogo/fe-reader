//! Platform integration trait. Implemented by fe_reader_platform_windows, _macos, _linux, _android, _ios.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentDocument {
    pub document_id: String,
    pub uri: String,
    pub display_name: String,
    pub sha256: Option<String>,
    pub last_page_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHandle {
    pub handle_id: String,
    pub uri: String,
    pub display_name: String,
    pub persisted_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedAccessGrant {
    pub grant_id: String,
    pub platform: String,
    pub uri: String,
    pub encrypted_payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeNotification {
    pub title: String,
    pub body: String,
    pub deep_link: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexDocument {
    pub document_id: String,
    pub title: String,
    pub text_preview: String,
    pub tags: Vec<String>,
    pub deep_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretPurpose {
    SigningKeyReference,
    PluginTrustState,
    PersistedFileGrant,
    LocalIndexKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenDocumentOptions {
    pub allowed_mime_types: Vec<String>,
    pub allow_multiple: bool,
    pub persist_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDocumentOptions {
    pub suggested_name: Option<String>,
    pub mime_type: String,
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareDocumentRequest {
    pub document_id: String,
    pub uri: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintDocumentRequest {
    pub document_id: String,
    pub uri: String,
}

pub trait PlatformIntegration: Send + Sync {
    fn platform_name(&self) -> &'static str;
    fn register_recent_document(&self, document: RecentDocument) -> anyhow::Result<()>;
    fn request_open_document(&self, options: OpenDocumentOptions) -> anyhow::Result<Vec<DocumentHandle>>;
    fn request_save_document(&self, options: SaveDocumentOptions) -> anyhow::Result<DocumentHandle>;
    fn persist_file_access(&self, handle: &DocumentHandle) -> anyhow::Result<PersistedAccessGrant>;
    fn restore_file_access(&self, grant: &PersistedAccessGrant) -> anyhow::Result<DocumentHandle>;
    fn show_notification(&self, notification: FeNotification) -> anyhow::Result<()>;
    fn index_document_for_search(&self, document: SearchIndexDocument) -> anyhow::Result<()>;
    fn protect_secret(&self, secret: &[u8], purpose: SecretPurpose) -> anyhow::Result<Vec<u8>>;
    fn unprotect_secret(&self, protected: &[u8], purpose: SecretPurpose) -> anyhow::Result<Vec<u8>>;
    fn share_document(&self, request: ShareDocumentRequest) -> anyhow::Result<()>;
    fn print_document(&self, request: PrintDocumentRequest) -> anyhow::Result<()>;
}
