//! Active content firewall contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveContentPolicy {
    pub javascript: ActiveContentDecision,
    pub launch_actions: ActiveContentDecision,
    pub rich_media: ActiveContentDecision,
    pub remote_uri: ActiveContentDecision,
    pub embedded_executables: ActiveContentDecision,
    pub submit_form: ActiveContentDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveContentDecision { Allow, Prompt, Disable, Block }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveContentAuditEvent {
    pub event_id: String,
    pub document_sha256: String,
    pub object_ref: Option<String>,
    pub content_type: String,
    pub decision: ActiveContentDecision,
    pub user_visible_message: String,
}
