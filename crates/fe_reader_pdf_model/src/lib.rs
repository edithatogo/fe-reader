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

/// Parser-backed text extraction summary for a PDF document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfTextExtractionSummary {
    /// Parser adapter used for text extraction.
    pub adapter: String,
    /// Extracted spans. Geometry may be page-level when the adapter lacks glyph boxes.
    pub spans: Vec<TextSpan>,
    /// Whether span bounding boxes are precise glyph/span geometry.
    pub precise_geometry: bool,
    /// Deterministic diagnostics about extraction limitations.
    pub diagnostics: Vec<String>,
    /// Non-fatal parser error, if the parser could not extract text.
    pub error: Option<String>,
}

/// Read-only PDF Engineering Lab session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfLabSession {
    /// Deterministic local session id.
    pub session_id: String,
    /// SHA-256 of inspected bytes.
    pub document_sha256: String,
    /// Lab inspection mode.
    pub mode: String,
    /// Object/page findings.
    pub findings: Vec<PdfLabFinding>,
    /// Number of indirect objects observed by the parser.
    pub object_count: usize,
    /// Number of stream objects observed by the parser.
    pub stream_count: usize,
    /// Trailer dictionary keys.
    pub trailer_keys: Vec<String>,
    /// Page graph summary.
    pub pages: Vec<PdfLabPageSummary>,
    /// Non-fatal parser error.
    pub error: Option<String>,
}

/// Read-only PDF Engineering Lab finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfLabFinding {
    /// Finding severity.
    pub severity: String,
    /// Stable finding code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Optional object id location.
    pub object_id: Option<String>,
    /// Optional page location.
    pub page_index: Option<u32>,
}

/// Page graph summary for the PDF Engineering Lab.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfLabPageSummary {
    /// Zero-based page index.
    pub page_index: PageIndex,
    /// Page object id.
    pub object_id: String,
    /// MediaBox if present.
    pub media_box: Option<PdfRect>,
    /// CropBox if present.
    pub crop_box: Option<PdfRect>,
    /// Effective fallback box used by simple diagnostics.
    pub effective_box: PdfRect,
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

/// Extracts text spans from a PDF path without mutating it.
///
/// # Errors
///
/// Returns an error if the file cannot be read or does not begin with a PDF header.
pub fn extract_text_spans_path(
    path: impl AsRef<Path>,
) -> Result<PdfTextExtractionSummary, FeError> {
    let bytes = fs::read(path.as_ref())
        .map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    extract_text_spans_bytes(&bytes)
}

/// Extracts text spans from PDF bytes without mutating them.
///
/// # Errors
///
/// Returns an error if the byte stream does not begin with a `%PDF-` header.
pub fn extract_text_spans_bytes(bytes: &[u8]) -> Result<PdfTextExtractionSummary, FeError> {
    require_pdf_header(bytes)?;
    Ok(extract_text_with_lopdf(bytes))
}

/// Builds a read-only PDF Engineering Lab object/page graph summary from a path.
///
/// # Errors
///
/// Returns an error if the file cannot be read or does not begin with a PDF header.
pub fn inspect_lab_path(path: impl AsRef<Path>) -> Result<PdfLabSession, FeError> {
    let bytes = fs::read(path.as_ref())
        .map_err(|error| FeError::new(FeErrorKind::Io, error.to_string()))?;
    inspect_lab_bytes(&bytes)
}

/// Builds a read-only PDF Engineering Lab object/page graph summary from bytes.
///
/// # Errors
///
/// Returns an error if the byte stream does not begin with a `%PDF-` header.
pub fn inspect_lab_bytes(bytes: &[u8]) -> Result<PdfLabSession, FeError> {
    require_pdf_header(bytes)?;
    Ok(inspect_lab_with_lopdf(bytes))
}

/// Sniffs PDF bytes without full parsing.
///
/// # Errors
///
/// Returns an error if the byte stream does not begin with a `%PDF-` header.
pub fn sniff_pdf_bytes(bytes: &[u8]) -> Result<PdfDocumentSummary, FeError> {
    let (raw, version) = require_pdf_header(bytes)?;
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

fn require_pdf_header(bytes: &[u8]) -> Result<(String, String), FeError> {
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
    Ok((raw, version))
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

fn inspect_lab_with_lopdf(bytes: &[u8]) -> PdfLabSession {
    let fingerprint = DocumentFingerprint::from_bytes(bytes);
    let session_id = format!("lab-{}", &fingerprint.sha256_hex[..16]);
    match lopdf::Document::load_mem(bytes) {
        Ok(document) => {
            let mut findings = Vec::new();
            let pages = document
                .get_pages()
                .into_iter()
                .map(|(page_number, page_id)| {
                    let media_box = page_box_rect(&document, page_id, b"MediaBox");
                    let crop_box = page_box_rect(&document, page_id, b"CropBox");
                    let effective_box = crop_box
                        .or(media_box)
                        .unwrap_or_else(|| PdfRect::new(0.0, 0.0, 1.0, 1.0));
                    if media_box.is_none() {
                        findings.push(PdfLabFinding {
                            severity: "warning".to_string(),
                            code: "missing_media_box".to_string(),
                            message: "page has no direct MediaBox; inherited boxes are not resolved in this smoke path".to_string(),
                            object_id: Some(format_object_id(page_id)),
                            page_index: Some(page_number.saturating_sub(1)),
                        });
                    }
                    PdfLabPageSummary {
                        page_index: PageIndex(page_number.saturating_sub(1)),
                        object_id: format_object_id(page_id),
                        media_box,
                        crop_box,
                        effective_box,
                    }
                })
                .collect::<Vec<_>>();
            findings.push(PdfLabFinding {
                severity: "info".to_string(),
                code: "object_page_graph_smoke".to_string(),
                message: "read-only object and page graph inspection completed without executing active content".to_string(),
                object_id: None,
                page_index: None,
            });
            PdfLabSession {
                session_id,
                document_sha256: fingerprint.sha256_hex,
                mode: "object_page_graph".to_string(),
                findings,
                object_count: document.objects.len(),
                stream_count: document
                    .objects
                    .values()
                    .filter(|object| matches!(object, lopdf::Object::Stream(_)))
                    .count(),
                trailer_keys: document
                    .trailer
                    .iter()
                    .map(|(key, _)| String::from_utf8_lossy(key).to_string())
                    .collect(),
                pages,
                error: None,
            }
        }
        Err(error) => PdfLabSession {
            session_id,
            document_sha256: fingerprint.sha256_hex,
            mode: "object_page_graph".to_string(),
            findings: vec![PdfLabFinding {
                severity: "error".to_string(),
                code: "parser_error".to_string(),
                message: error.to_string(),
                object_id: None,
                page_index: None,
            }],
            object_count: 0,
            stream_count: 0,
            trailer_keys: Vec::new(),
            pages: Vec::new(),
            error: Some(error.to_string()),
        },
    }
}

fn extract_text_with_lopdf(bytes: &[u8]) -> PdfTextExtractionSummary {
    match lopdf::Document::load_mem(bytes) {
        Ok(document) => {
            let mut spans = Vec::new();
            let mut diagnostics =
                vec!["lopdf text extraction provides page-level fallback geometry".to_string()];
            for (page_number, page_id) in document.get_pages() {
                match document.extract_text(&[page_number]) {
                    Ok(text) if !text.trim().is_empty() => {
                        let bbox = page_fallback_rect(&document, page_id)
                            .unwrap_or_else(|| PdfRect::new(0.0, 0.0, 1.0, 1.0));
                        spans.push(TextSpan {
                            page_index: PageIndex(page_number.saturating_sub(1)),
                            text,
                            bbox,
                            reading_order: Some(spans.len() as u32),
                            font_name: None,
                        });
                    }
                    Ok(_) => {}
                    Err(error) => diagnostics.push(format!(
                        "page {page_number} text extraction failed: {error}"
                    )),
                }
            }
            PdfTextExtractionSummary {
                adapter: "lopdf".to_string(),
                spans,
                precise_geometry: false,
                diagnostics,
                error: None,
            }
        }
        Err(error) => PdfTextExtractionSummary {
            adapter: "lopdf".to_string(),
            spans: Vec::new(),
            precise_geometry: false,
            diagnostics: Vec::new(),
            error: Some(error.to_string()),
        },
    }
}

fn page_fallback_rect(document: &lopdf::Document, page_id: lopdf::ObjectId) -> Option<PdfRect> {
    page_box_rect(document, page_id, b"CropBox")
        .or_else(|| page_box_rect(document, page_id, b"MediaBox"))
}

fn page_box_rect(
    document: &lopdf::Document,
    page_id: lopdf::ObjectId,
    key: &[u8],
) -> Option<PdfRect> {
    let page = document.get_dictionary(page_id).ok()?;
    page.get(key)
        .ok()
        .and_then(pdf_box_to_rect)
        .filter(|rect| rect.is_non_empty())
}

fn pdf_box_to_rect(object: &lopdf::Object) -> Option<PdfRect> {
    let values = object.as_array().ok()?;
    if values.len() != 4 {
        return None;
    }
    let x0 = values[0].as_float().ok()?;
    let y0 = values[1].as_float().ok()?;
    let x1 = values[2].as_float().ok()?;
    let y1 = values[3].as_float().ok()?;
    Some(PdfRect::new(x0, y0, x1 - x0, y1 - y0))
}

fn format_object_id(object_id: lopdf::ObjectId) -> String {
    format!("{} {}", object_id.0, object_id.1)
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
    fn malformed_pdf_header_returns_non_fatal_parser_error() {
        let summary =
            sniff_pdf_bytes(b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R\n").unwrap();

        assert_eq!(summary.header.version, "1.7");
        assert!(!summary.eof_marker_hint);
        assert_eq!(summary.parser.adapter, "lopdf");
        assert_eq!(summary.parser.page_count, None);
        assert_eq!(summary.parser.version, None);
        assert_eq!(summary.parser.encrypted, None);
        assert!(summary.parser.error.is_some());
    }

    #[test]
    fn extracts_text_with_page_level_geometry() {
        let bytes = lopdf_text_fixture("Fe Reader Search");
        let summary = extract_text_spans_bytes(&bytes).unwrap();

        assert_eq!(summary.adapter, "lopdf");
        assert!(!summary.precise_geometry);
        assert_eq!(summary.error, None);
        assert_eq!(summary.spans.len(), 1);
        assert_eq!(summary.spans[0].page_index, PageIndex(0));
        assert!(summary.spans[0].text.contains("Fe Reader Search"));
        assert_eq!(summary.spans[0].bbox, PdfRect::new(0.0, 0.0, 612.0, 792.0));
        assert!(
            summary
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("page-level fallback geometry"))
        );
    }

    #[test]
    fn lab_inspection_reports_object_and_page_graph() {
        let bytes = minimal_xref_stream_pdf();
        let session = inspect_lab_bytes(&bytes).unwrap();

        assert_eq!(session.mode, "object_page_graph");
        assert_eq!(session.error, None);
        assert_eq!(session.pages.len(), 1);
        assert!(session.object_count > 0);
        assert!(session.stream_count > 0);
        assert_eq!(
            session.pages[0].effective_box,
            PdfRect::new(0.0, 0.0, 612.0, 792.0)
        );
        assert!(
            session
                .findings
                .iter()
                .any(|finding| finding.code == "object_page_graph_smoke")
        );
    }

    #[test]
    fn lab_inspection_reports_parser_error_without_mutation() {
        let session =
            inspect_lab_bytes(b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R\n").unwrap();

        assert_eq!(session.mode, "object_page_graph");
        assert!(session.error.is_some());
        assert!(session.pages.is_empty());
        assert_eq!(session.findings[0].code, "parser_error");
    }

    #[test]
    fn malformed_pdf_text_extraction_is_non_fatal_after_header() {
        let summary =
            extract_text_spans_bytes(b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R\n")
                .unwrap();

        assert_eq!(summary.adapter, "lopdf");
        assert!(summary.spans.is_empty());
        assert!(summary.error.is_some());
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

    fn lopdf_text_fixture(text: &str) -> Vec<u8> {
        use lopdf::content::{Content, Operation};
        use lopdf::{Document, Object, Stream, dictionary};

        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });
        let resources_id = document.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new(
                    "Tf",
                    vec![Object::Name(b"F1".to_vec()), Object::Integer(12)],
                ),
                Operation::new("Td", vec![Object::Integer(72), Object::Integer(720)]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = document.add_object(Stream::new(
            dictionary! {},
            content.encode().expect("content encodes"),
        ));
        let page_id = document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        document.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        document.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }
}
