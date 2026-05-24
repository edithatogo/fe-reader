//! Minimal PDF model and sniffing contracts for Wave 0.
//!
//! This crate contains typed geometry and inspection primitives only. Full parsing/rendering stays
//! behind later adapters.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{DocumentFingerprint, DocumentId, FeError, FeErrorKind};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Zero-based page index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PageIndex(pub u32);

/// Axis-aligned rectangle in PDF user-space points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PdfRect {
    /// Left x coordinate.
    pub x: f32,
    /// Bottom y coordinate.
    pub y: f32,
    /// Width in points.
    pub width: f32,
    /// Height in points.
    pub height: f32,
}

impl PdfRect {
    /// Creates a rectangle.
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns true when the rectangle has positive dimensions.
    #[must_use]
    pub fn is_non_empty(self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}

/// Page rotation in degrees clockwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageRotation {
    /// 0 degrees.
    Deg0,
    /// 90 degrees.
    Deg90,
    /// 180 degrees.
    Deg180,
    /// 270 degrees.
    Deg270,
}

/// Known page box kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageBoxKind {
    /// MediaBox.
    Media,
    /// CropBox.
    Crop,
    /// BleedBox.
    Bleed,
    /// TrimBox.
    Trim,
    /// ArtBox.
    Art,
}

/// Text span with geometry and reading-order hints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSpan {
    /// Page index.
    pub page_index: PageIndex,
    /// Extracted text.
    pub text: String,
    /// Bounding box in PDF points.
    pub bbox: PdfRect,
    /// Reading-order index if known.
    pub reading_order: Option<u32>,
    /// Optional font name.
    pub font_name: Option<String>,
}

/// PDF header detected from a byte stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfHeader {
    /// Version string after `%PDF-`, for example `1.7` or `2.0`.
    pub version: String,
    /// Raw first line.
    pub raw: String,
}

/// Minimal non-mutating PDF document summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfDocumentSummary {
    /// Local document id assigned by Fe Reader.
    pub document_id: DocumentId,
    /// Fingerprint of the bytes inspected.
    pub fingerprint: DocumentFingerprint,
    /// Detected header.
    pub header: PdfHeader,
    /// Whether the byte stream contains an `/Encrypt` marker.
    pub encrypted_hint: bool,
    /// Whether the byte stream contains a `/Linearized` marker near the beginning.
    pub linearized_hint: bool,
    /// Whether an EOF marker was observed.
    pub eof_marker_hint: bool,
    /// Parser-backed inspection result when the document can be opened safely.
    pub parser: PdfParserSummary,
}

/// Parser-backed inspection details for a PDF document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfParserSummary {
    /// Parser adapter used for this inspection.
    pub adapter: String,
    /// PDF version reported by the parser.
    pub version: Option<String>,
    /// Number of pages discovered through the page tree.
    pub page_count: Option<u32>,
    /// Whether the parser found an encryption dictionary.
    pub encrypted: Option<bool>,
    /// Root trailer dictionary keys visible to the parser.
    pub trailer_keys: Vec<String>,
    /// Non-fatal parser error, if the parser could not open the document.
    pub error: Option<String>,
}

impl PdfParserSummary {
    fn lopdf_error(error: impl ToString) -> Self {
        Self {
            adapter: "lopdf".to_string(),
            version: None,
            page_count: None,
            encrypted: None,
            trailer_keys: Vec::new(),
            error: Some(error.to_string()),
        }
    }
}

/// Reads and sniffs a PDF file without mutating it.
///
/// # Errors
///
/// Returns an error if the file cannot be read or does not begin with a PDF header.
pub fn sniff_pdf_path(path: impl AsRef<Path>) -> Result<PdfDocumentSummary, FeError> {
    let bytes = fs::read(path.as_ref())
        .map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    sniff_pdf_bytes(&bytes)
}

/// Sniffs PDF bytes without full parsing.
///
/// # Errors
///
/// Returns an error if the byte stream does not begin with a `%PDF-` header.
pub fn sniff_pdf_bytes(bytes: &[u8]) -> Result<PdfDocumentSummary, FeError> {
    let first_line_end = bytes
        .iter()
        .position(|byte| *byte == b'\n' || *byte == b'\r')
        .unwrap_or(bytes.len());
    let first_line = &bytes[..first_line_end.min(bytes.len())];
    if !first_line.starts_with(b"%PDF-") {
        return Err(FeError::new(
            FeErrorKind::Parse,
            "file does not start with a PDF header",
        ));
    }
    let raw = String::from_utf8_lossy(first_line).to_string();
    let version = raw.strip_prefix("%PDF-").unwrap_or("").trim().to_string();
    let prefix_len = bytes.len().min(4096);
    let prefix = String::from_utf8_lossy(&bytes[..prefix_len]);
    let full = String::from_utf8_lossy(bytes);
    Ok(PdfDocumentSummary {
        document_id: DocumentId::new(),
        fingerprint: DocumentFingerprint::from_bytes(bytes),
        header: PdfHeader { version, raw },
        encrypted_hint: full.contains("/Encrypt"),
        linearized_hint: prefix.contains("/Linearized"),
        eof_marker_hint: full.contains("%%EOF"),
        parser: inspect_pdf_with_lopdf(bytes),
    })
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

fn inspect_pdf_with_lopdf(bytes: &[u8]) -> PdfParserSummary {
    match lopdf::Document::load_mem(bytes) {
        Ok(document) => PdfParserSummary {
            adapter: "lopdf".to_string(),
            version: Some(document.version.clone()),
            page_count: Some(document.get_pages().len() as u32),
            encrypted: Some(document.is_encrypted()),
            trailer_keys: document
                .trailer
                .iter()
                .map(|(key, _)| String::from_utf8_lossy(key).to_string())
                .collect(),
            error: None,
        },
        Err(error) => PdfParserSummary::lopdf_error(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniffs_minimal_pdf_header() {
        let summary = sniff_pdf_bytes(b"%PDF-1.7\n1 0 obj\n<<>>\nendobj\n%%EOF").unwrap();
        assert_eq!(summary.header.version, "1.7");
        assert!(summary.eof_marker_hint);
    }

    #[test]
    fn reports_lopdf_parser_page_count() {
        let bytes = minimal_xref_stream_pdf();
        let summary = sniff_pdf_bytes(&bytes).unwrap();
        assert_eq!(summary.parser.adapter, "lopdf");
        assert_eq!(summary.parser.page_count, Some(1));
        assert_eq!(summary.parser.version, Some("1.5".to_string()));
        assert_eq!(summary.parser.error, None);
    }

    #[test]
    fn rejects_non_pdf_header() {
        let error = sniff_pdf_bytes(b"hello").unwrap_err();
        assert_eq!(error.kind, FeErrorKind::Parse);
    }

    #[test]
    fn rect_requires_positive_dimensions() {
        assert!(PdfRect::new(0.0, 0.0, 1.0, 1.0).is_non_empty());
        assert!(!PdfRect::new(0.0, 0.0, 0.0, 1.0).is_non_empty());
    }

    fn minimal_xref_stream_pdf() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"%PDF-1.5\n%\x80\x80\x80\x80\n");
        let mut object_offsets = Vec::new();
        for object in [
            b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n".as_slice(),
            b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n".as_slice(),
            b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\n"
                .as_slice(),
        ] {
            object_offsets.push(bytes.len() as u32);
            bytes.extend_from_slice(object);
        }
        let xref_offset = bytes.len() as u32;
        let mut xref_entries = Vec::new();
        push_xref_entry(&mut xref_entries, 0, 0, 65_535);
        for object_offset in object_offsets.into_iter().chain([xref_offset]) {
            push_xref_entry(&mut xref_entries, 1, object_offset, 0);
        }
        bytes.extend_from_slice(
            format!(
                "4 0 obj\n<< /Type /XRef /Size 5 /Root 1 0 R /W [1 4 2] /Length {} >>\nstream\n",
                xref_entries.len()
            )
            .as_bytes(),
        );
        bytes.extend_from_slice(&xref_entries);
        bytes.extend_from_slice(b"\nendstream\nendobj\n");
        bytes.extend_from_slice(format!("startxref\n{xref_offset}\n%%EOF\n").as_bytes());
        bytes
    }

    fn push_xref_entry(entries: &mut Vec<u8>, kind: u8, field_2: u32, field_3: u16) {
        entries.push(kind);
        entries.extend_from_slice(&field_2.to_be_bytes());
        entries.extend_from_slice(&field_3.to_be_bytes());
    }
}
