//! Offline collaboration and review-packet contracts.

#[derive(Debug, Clone)]
pub struct ReviewPacket {
    pub packet_id: String,
    pub base_document_sha256: String,
    pub author_label: String,
    pub created_at_utc: String,
    pub comments: Vec<ReviewComment>,
    pub proposed_patch_plans: Vec<String>,
    pub signature_hint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReviewComment {
    pub comment_id: String,
    pub page_index: u32,
    pub bbox: Option<[f64; 4]>,
    pub body_markdown: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReviewPacketImportResult {
    pub packet_id: String,
    pub base_hash_matches: bool,
    pub conflicts: Vec<String>,
    pub imported_comments: usize,
    pub proposed_patch_plan_ids: Vec<String>,
}

pub trait ReviewPacketStore: Send + Sync {
    fn export_packet(&self, packet: ReviewPacket, path: &str) -> Result<(), String>;
    fn import_packet(&self, path: &str, expected_document_sha256: &str) -> Result<ReviewPacketImportResult, String>;
}
