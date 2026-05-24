//! External application integration contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationDirection {
    Import,
    Export,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationIntegrationDescriptor {
    pub integration_id: String,
    pub display_name: String,
    pub version: String,
    pub direction: IntegrationDirection,
    pub capabilities: Vec<String>,
    pub requires_network: bool,
    pub requires_user_grant: bool,
    pub risk_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRequest {
    pub request_id: String,
    pub integration_id: String,
    pub document_id: Option<String>,
    pub operation: String,
    pub payload_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub request_id: String,
    pub result_json: serde_json::Value,
    pub warnings: Vec<String>,
}

pub trait ApplicationIntegration: Send + Sync {
    fn descriptor(&self) -> ApplicationIntegrationDescriptor;
    fn handle(&self, request: IntegrationRequest) -> anyhow::Result<IntegrationResponse>;
}
