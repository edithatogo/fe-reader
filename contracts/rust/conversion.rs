//! Conversion provider contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionInputKind {
    Pdf,
    Markdown,
    Quarto,
    Typst,
    Latex,
    Docx,
    Odt,
    ImageSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionOutputKind {
    Pdf,
    Text,
    Markdown,
    Html,
    Docx,
    Odt,
    Json,
    Png,
    Svg,
    Typst,
    Latex,
    Quarto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionJob {
    pub job_id: String,
    pub input_kind: ConversionInputKind,
    pub output_kind: ConversionOutputKind,
    pub input_uri: String,
    pub output_uri: Option<String>,
    pub preserve_metadata: bool,
    pub create_source_map: bool,
    pub provider_hint: Option<String>,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub job_id: String,
    pub output_uri: String,
    pub warnings: Vec<String>,
    pub source_map_uri: Option<String>,
}

pub trait ConversionProvider: Send + Sync {
    fn provider_name(&self) -> &'static str;
    fn supports(&self, input: &ConversionInputKind, output: &ConversionOutputKind) -> bool;
    fn convert(&self, job: ConversionJob) -> anyhow::Result<ConversionResult>;
}
