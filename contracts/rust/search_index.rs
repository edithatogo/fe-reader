//! Deterministic search/index contract.
//! ML embeddings are explicitly not part of this contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedSpan {
    pub document_id: String,
    pub document_sha256: String,
    pub page_index: u32,
    pub span_id: String,
    pub text: String,
    pub bbox: [f32; 4],
    pub reading_order: Option<u32>,
    pub language_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub mode: SearchMode,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub max_results: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchMode {
    Literal,
    Phrase,
    Regex,
    Metadata,
    Annotation,
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub document_id: String,
    pub page_index: u32,
    pub span_id: Option<String>,
    pub bbox: Option<[f32; 4]>,
    pub snippet: String,
    pub score: f32,
}

pub trait SearchIndexProvider: Send + Sync {
    fn upsert_spans(&self, spans: &[IndexedSpan]) -> anyhow::Result<()>;
    fn delete_document(&self, document_id: &str) -> anyhow::Result<()>;
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchHit>>;
    fn purge_workspace(&self, workspace_id: &str) -> anyhow::Result<()>;
}
