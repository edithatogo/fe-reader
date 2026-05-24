//! Contract sketch for API and ABI compatibility governance.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilityLevel { Internal, Experimental, Preview, Stable, Lts }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicSurface {
    RustCrate { crate_name: String },
    UniFfiBinding { language: String },
    CAbi,
    Cli,
    Mcp,
    WasmPluginAbi,
    ComAutomation,
    AppleScript,
    DBus,
    AndroidIntent,
    IosAppIntent,
    WebPostMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCompatibilityReport {
    pub surface: PublicSurface,
    pub previous_version: Option<String>,
    pub current_version: String,
    pub stability: StabilityLevel,
    pub breaking_changes: Vec<ApiChange>,
    pub migration_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChange {
    pub change_id: String,
    pub description: String,
    pub is_breaking: bool,
    pub requires_major_version: bool,
}
