//! Contract sketch for colour, prepress and font fidelity reporting.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepressReport {
    pub document_id: String,
    pub output_intents: Vec<OutputIntentSummary>,
    pub colour_findings: Vec<ColourFinding>,
    pub font_findings: Vec<FontFinding>,
    pub page_box_findings: Vec<PageBoxFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputIntentSummary {
    pub object_id: Option<String>,
    pub subtype: Option<String>,
    pub profile_description: Option<String>,
    pub page_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColourFinding {
    pub page_index: Option<u32>,
    pub code: String,
    pub message: String,
    pub preservation_risk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFinding {
    pub page_index: Option<u32>,
    pub font_name: String,
    pub embedded: bool,
    pub subset: bool,
    pub has_to_unicode: bool,
    pub extraction_risk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBoxFinding {
    pub page_index: u32,
    pub media_box: [f64; 4],
    pub crop_box: Option<[f64; 4]>,
    pub bleed_box: Option<[f64; 4]>,
    pub trim_box: Option<[f64; 4]>,
    pub art_box: Option<[f64; 4]>,
}
