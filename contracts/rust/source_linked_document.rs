//! Source-linked document contracts for Typst, Quarto, LaTeX/Tectonic and Pandoc workflows.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLinkedProject {
    pub project_id: String,
    pub root_dir: String,
    pub provider: BuildProviderKind,
    pub entrypoint: String,
    pub output_pdf: String,
    pub source_map: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildProviderKind {
    Typst,
    Quarto,
    Pandoc,
    Tectonic,
    LatexToolchain,
    CustomExternal,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub heading: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PdfSourceMapQuery {
    pub page_index: u32,
    pub bbox: Option<[f64; 4]>,
}

pub trait SourceLinkedProvider: Send + Sync {
    fn detect(&self, root_dir: &str) -> Result<Option<SourceLinkedProject>, String>;
    fn build(&self, project: &SourceLinkedProject) -> Result<String, String>;
    fn locate_source(&self, project: &SourceLinkedProject, query: PdfSourceMapQuery) -> Result<Option<SourceLocation>, String>;
}
