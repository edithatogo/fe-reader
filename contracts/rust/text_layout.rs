//! Text extraction, shaping and font diagnostics contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WritingMode {
    HorizontalLtr,
    HorizontalRtl,
    VerticalTtb,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSpanDiagnostic {
    pub span_id: String,
    pub decoded_text: String,
    pub original_glyph_ids: Vec<u32>,
    pub bbox: [f32; 4],
    pub writing_mode: WritingMode,
    pub font_name: Option<String>,
    pub has_tounicode: bool,
    pub extraction_confidence: f32,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSubstitutionReport {
    pub requested_font: String,
    pub substituted_font: Option<String>,
    pub reason: String,
}

pub trait TextDiagnosticsProvider: Send + Sync {
    fn page_text_diagnostics(&self, document_id: &str, page_index: u32) -> anyhow::Result<Vec<TextSpanDiagnostic>>;
    fn font_substitutions(&self, document_id: &str) -> anyhow::Result<Vec<FontSubstitutionReport>>;
}
