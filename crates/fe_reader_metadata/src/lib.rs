//! Metadata planning contracts.
//!
//! Wave 0 does not mutate PDF bytes. It creates explicit plans for metadata editing/scrubbing.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{FeError, FeErrorKind, OperationIntent, PatchOperation, PatchPlan};
use lopdf::{Dictionary, Document, Object};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Basic document info dictionary fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentInfo {
    /// Title.
    pub title: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Subject.
    pub subject: Option<String>,
    /// Keywords.
    pub keywords: Vec<String>,
    /// Creator application.
    pub creator: Option<String>,
    /// Producer application.
    pub producer: Option<String>,
}

/// Read-only metadata inspection summary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSummary {
    /// Document info dictionary fields decoded from PDF text strings.
    pub document_info: DocumentInfo,
    /// Whether the document catalog appears to point at an XMP metadata stream.
    pub xmp_metadata_present: bool,
    /// Read-only diagnostics for discovered XMP metadata streams.
    pub xmp_streams: Vec<XmpMetadataStream>,
    /// Root trailer dictionary keys visible to the parser.
    pub trailer_keys: Vec<String>,
    /// Claimed or recognized conformance markers and standards cues.
    pub conformance_claims: Vec<String>,
    /// Non-fatal warnings produced by metadata inspection.
    pub warnings: Vec<String>,
    /// Non-fatal parser error, if the PDF could not be opened for metadata inspection.
    pub parser_error: Option<String>,
}

/// Read-only diagnostics for one XMP metadata stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmpMetadataStream {
    /// Indirect object id when known.
    pub object_id: String,
    /// Decoded or raw byte length of the XMP packet.
    pub byte_len: usize,
    /// SHA-256 of the decoded or raw XMP packet bytes.
    pub sha256_hex: String,
    /// UTF-8 lossy preview of the first bytes of the packet.
    pub preview: String,
    /// Non-fatal decode error, if the stream could not be decoded.
    pub decode_error: Option<String>,
}

/// Stable Wave 2 metadata snapshot payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSnapshot {
    /// Snapshot contract version.
    pub snapshot_version: u8,
    /// Read-only metadata summary captured from the input document.
    pub summary: MetadataSummary,
}

impl MetadataSnapshot {
    /// Builds a Wave 2 metadata snapshot from a read-only summary.
    #[must_use]
    pub const fn new(summary: MetadataSummary) -> Self {
        Self {
            snapshot_version: 1,
            summary,
        }
    }
}

/// One metadata snapshot difference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDiffEntry {
    /// Dot-separated metadata field path.
    pub field: String,
    /// Value in the before snapshot.
    pub before: serde_json::Value,
    /// Value in the after snapshot.
    pub after: serde_json::Value,
}

/// Stable Wave 2 metadata diff payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDiff {
    /// Before snapshot.
    pub before: MetadataSnapshot,
    /// After snapshot.
    pub after: MetadataSnapshot,
    /// Changed metadata fields.
    pub changes: Vec<MetadataDiffEntry>,
}

/// Metadata scrub strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataScrubMode {
    /// Preserve all metadata.
    Preserve,
    /// Preserve metadata but emit a no-write forensic review plan.
    ForensicPreserve,
    /// Remove common private fields but keep user-visible title/subject.
    CleanShare,
    /// Remove all non-essential metadata.
    Aggressive,
}

impl MetadataScrubMode {
    /// Parses a user-facing scrub profile.
    ///
    /// # Errors
    ///
    /// Returns an error when the profile is not a supported Wave 2 scrub mode.
    pub fn parse_profile(profile: &str) -> Result<Self, FeError> {
        match profile {
            "preserve" => Ok(Self::Preserve),
            "forensic-preserve" | "forensic_preserve" => Ok(Self::ForensicPreserve),
            "clean-share" | "clean_share" => Ok(Self::CleanShare),
            "aggressive" => Ok(Self::Aggressive),
            _ => Err(FeError::new(
                FeErrorKind::InvalidInput,
                format!("unknown metadata scrub profile: {profile}"),
            )),
        }
    }

    fn as_plan_value(self) -> &'static str {
        match self {
            Self::Preserve => "preserve",
            Self::ForensicPreserve => "forensic_preserve",
            Self::CleanShare => "clean_share",
            Self::Aggressive => "aggressive",
        }
    }
}

/// Metadata operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum MetadataOperation {
    /// Set a document-info field.
    SetInfoField {
        /// Field name.
        field: String,
        /// Field value.
        value: String,
    },
    /// Scrub metadata according to a mode.
    Scrub {
        /// Scrub mode.
        mode: MetadataScrubMode,
    },
}

/// Plans metadata operations as a core patch plan.
#[must_use]
pub fn plan_metadata_operations(
    intent: &OperationIntent,
    operations: &[MetadataOperation],
) -> PatchPlan {
    let patch_ops = operations
        .iter()
        .map(|operation| match operation {
            MetadataOperation::SetInfoField { field, value } => PatchOperation::SetMetadata {
                key: field.clone(),
                value: value.clone(),
            },
            MetadataOperation::Scrub { mode } => match mode {
                MetadataScrubMode::ForensicPreserve => PatchOperation::Noop,
                _ => PatchOperation::SetMetadata {
                    key: "metadata_scrub_mode".to_string(),
                    value: mode.as_plan_value().to_string(),
                },
            },
        })
        .collect();
    PatchPlan::draft(intent, "metadata operations", patch_ops)
}

/// Creates a stable Wave 2 metadata snapshot from PDF bytes.
#[must_use]
pub fn metadata_snapshot_bytes(bytes: &[u8]) -> MetadataSnapshot {
    MetadataSnapshot::new(inspect_metadata_bytes(bytes))
}

/// Creates a stable Wave 2 metadata snapshot from a path.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn metadata_snapshot_path(path: impl AsRef<Path>) -> anyhow::Result<MetadataSnapshot> {
    let bytes = fs::read(path)?;
    Ok(metadata_snapshot_bytes(&bytes))
}

/// Computes a deterministic diff between two metadata snapshots.
#[must_use]
pub fn diff_metadata_snapshots(before: MetadataSnapshot, after: MetadataSnapshot) -> MetadataDiff {
    let mut changes = Vec::new();
    push_diff(
        &mut changes,
        "document_info.title",
        &before.summary.document_info.title,
        &after.summary.document_info.title,
    );
    push_diff(
        &mut changes,
        "document_info.author",
        &before.summary.document_info.author,
        &after.summary.document_info.author,
    );
    push_diff(
        &mut changes,
        "document_info.subject",
        &before.summary.document_info.subject,
        &after.summary.document_info.subject,
    );
    push_diff(
        &mut changes,
        "document_info.keywords",
        &before.summary.document_info.keywords,
        &after.summary.document_info.keywords,
    );
    push_diff(
        &mut changes,
        "document_info.creator",
        &before.summary.document_info.creator,
        &after.summary.document_info.creator,
    );
    push_diff(
        &mut changes,
        "document_info.producer",
        &before.summary.document_info.producer,
        &after.summary.document_info.producer,
    );
    push_diff(
        &mut changes,
        "xmp_metadata_present",
        &before.summary.xmp_metadata_present,
        &after.summary.xmp_metadata_present,
    );
    push_diff(
        &mut changes,
        "xmp_streams",
        &before.summary.xmp_streams,
        &after.summary.xmp_streams,
    );
    push_diff(
        &mut changes,
        "trailer_keys",
        &before.summary.trailer_keys,
        &after.summary.trailer_keys,
    );
    push_diff(
        &mut changes,
        "parser_error",
        &before.summary.parser_error,
        &after.summary.parser_error,
    );
    MetadataDiff {
        before,
        after,
        changes,
    }
}

/// Computes a deterministic diff between two metadata snapshot paths.
///
/// # Errors
///
/// Returns an error if either file cannot be read.
pub fn diff_metadata_paths(
    before_path: impl AsRef<Path>,
    after_path: impl AsRef<Path>,
) -> anyhow::Result<MetadataDiff> {
    Ok(diff_metadata_snapshots(
        metadata_snapshot_path(before_path)?,
        metadata_snapshot_path(after_path)?,
    ))
}

/// Inspects PDF metadata bytes without mutating the document.
///
/// Parser failures are recorded as non-fatal metadata on the returned summary.
#[must_use]
pub fn inspect_metadata_bytes(bytes: &[u8]) -> MetadataSummary {
    match Document::load_mem(bytes) {
        Ok(document) => inspect_lopdf_document(&document),
        Err(error) => MetadataSummary {
            parser_error: Some(error.to_string()),
            ..MetadataSummary::default()
        },
    }
}

/// Inspects PDF metadata from a path without mutating the document.
///
/// # Errors
///
/// Returns an error if the file cannot be read. PDF parser failures are recorded
/// as non-fatal metadata on the returned summary.
pub fn inspect_metadata_path(path: impl AsRef<Path>) -> anyhow::Result<MetadataSummary> {
    let bytes = fs::read(path)?;
    Ok(inspect_metadata_bytes(&bytes))
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

fn inspect_lopdf_document(document: &Document) -> MetadataSummary {
    MetadataSummary {
        document_info: inspect_document_info(document),
        xmp_metadata_present: has_xmp_metadata(document),
        xmp_streams: inspect_xmp_streams(document),
        trailer_keys: sorted_dictionary_keys(&document.trailer),
        conformance_claims: inspect_conformance_claims(document),
        warnings: inspect_metadata_warnings(document),
        parser_error: None,
    }
}

fn inspect_document_info(document: &Document) -> DocumentInfo {
    let Some(info_dictionary) = document
        .trailer
        .get(b"Info")
        .ok()
        .and_then(|info| document.dereference(info).ok())
        .and_then(|(_, info)| info.as_dict().ok())
    else {
        return DocumentInfo::default();
    };

    DocumentInfo {
        title: decode_info_field(info_dictionary, b"Title"),
        author: decode_info_field(info_dictionary, b"Author"),
        subject: decode_info_field(info_dictionary, b"Subject"),
        keywords: decode_info_field(info_dictionary, b"Keywords")
            .map(|keywords| split_keywords(&keywords))
            .unwrap_or_default(),
        creator: decode_info_field(info_dictionary, b"Creator"),
        producer: decode_info_field(info_dictionary, b"Producer"),
    }
}

fn decode_info_field(dictionary: &Dictionary, key: &[u8]) -> Option<String> {
    dictionary
        .get(key)
        .ok()
        .and_then(|object| lopdf::decode_text_string(object).ok())
}

fn split_keywords(keywords: &str) -> Vec<String> {
    keywords
        .split([',', ';'])
        .map(str::trim)
        .filter(|keyword| !keyword.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn has_xmp_metadata(document: &Document) -> bool {
    catalog_has_metadata(document) || document.objects.values().any(is_xmp_metadata_stream)
}

fn catalog_has_metadata(document: &Document) -> bool {
    catalog_has_key(document, b"Metadata")
}

fn catalog_has_key(document: &Document, key: &[u8]) -> bool {
    document
        .trailer
        .get(b"Root")
        .ok()
        .and_then(|root| document.dereference(root).ok())
        .and_then(|(_, root)| root.as_dict().ok())
        .is_some_and(|catalog| catalog.has(key))
}

fn is_xmp_metadata_stream(object: &Object) -> bool {
    let Ok(stream) = object.as_stream() else {
        return false;
    };
    let is_metadata_type = stream
        .dict
        .get(b"Type")
        .and_then(Object::as_name)
        .is_ok_and(|name| name == b"Metadata");
    let is_xml_subtype = stream
        .dict
        .get(b"Subtype")
        .and_then(Object::as_name)
        .is_ok_and(|name| name == b"XML");
    is_metadata_type && is_xml_subtype
}

fn inspect_xmp_streams(document: &Document) -> Vec<XmpMetadataStream> {
    let mut streams = document
        .objects
        .iter()
        .filter_map(|(object_id, object)| {
            if !is_xmp_metadata_stream(object) {
                return None;
            }
            let stream = object.as_stream().ok()?;
            let object_id = format!("{} {}", object_id.0, object_id.1);
            Some(inspect_xmp_stream(object_id, stream))
        })
        .collect::<Vec<_>>();
    streams.sort_by(|left, right| left.object_id.cmp(&right.object_id));
    streams
}

fn inspect_xmp_stream(object_id: String, stream: &lopdf::Stream) -> XmpMetadataStream {
    let (bytes, decode_error) = match stream.get_plain_content() {
        Ok(bytes) => (bytes, None),
        Err(error) => (stream.content.clone(), Some(error.to_string())),
    };
    XmpMetadataStream {
        object_id,
        byte_len: bytes.len(),
        sha256_hex: sha256_hex(&bytes),
        preview: preview_utf8(&bytes),
        decode_error,
    }
}

fn inspect_conformance_claims(document: &Document) -> Vec<String> {
    let mut claims = Vec::new();
    if document.version.trim_start().starts_with('2') {
        claims.push("pdf-2.0".to_string());
    }
    if catalog_has_key(document, b"StructTreeRoot") {
        claims.push("tagged-pdf".to_string());
    }
    if catalog_has_key(document, b"OutputIntents") {
        claims.push("output-intents".to_string());
    }
    if catalog_has_key(document, b"AF") {
        claims.push("associated-files".to_string());
    }
    claims.sort();
    claims.dedup();
    claims
}

fn inspect_metadata_warnings(document: &Document) -> Vec<String> {
    let mut warnings = Vec::new();
    if document.version.trim_start().starts_with('2') && !catalog_has_metadata(document) {
        warnings.push("PDF 2.0 document does not expose an XMP metadata stream".to_string());
    }
    warnings
}

fn preview_utf8(bytes: &[u8]) -> String {
    let max = bytes.len().min(160);
    String::from_utf8_lossy(&bytes[..max])
        .chars()
        .map(|ch| if ch.is_control() { ' ' } else { ch })
        .collect::<String>()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn sorted_dictionary_keys(dictionary: &Dictionary) -> Vec<String> {
    let mut keys = dictionary
        .iter()
        .map(|(key, _)| String::from_utf8_lossy(key).to_string())
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

fn push_diff<T>(changes: &mut Vec<MetadataDiffEntry>, field: &str, before: &T, after: &T)
where
    T: PartialEq + Serialize,
{
    if before == after {
        return;
    }
    changes.push(MetadataDiffEntry {
        field: field.to_string(),
        before: serde_json::to_value(before).unwrap_or(serde_json::Value::Null),
        after: serde_json::to_value(after).unwrap_or(serde_json::Value::Null),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use fe_reader_core::{DocumentId, OperationKind, OperationSource, WriteMode};
    use lopdf::dictionary;

    #[test]
    fn metadata_plan_is_not_auto_approved() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "metadata",
        );
        let plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::SetInfoField {
                field: "title".into(),
                value: "Fe".into(),
            }],
        );
        assert!(!plan.approved_for_apply);
        assert_eq!(plan.operations.len(), 1);
    }

    #[test]
    fn metadata_write_mode_distinguishes_update_from_scrub() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "metadata",
        );
        let update_plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::SetInfoField {
                field: "title".into(),
                value: "Fe".into(),
            }],
        );
        let scrub_plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::Scrub {
                mode: MetadataScrubMode::Aggressive,
            }],
        );

        assert_eq!(update_plan.write_mode, WriteMode::IncrementalAppend);
        assert_eq!(scrub_plan.write_mode, WriteMode::SanitizingRewrite);
        assert!(!update_plan.approved_for_apply);
        assert!(!scrub_plan.approved_for_apply);
    }

    #[test]
    fn metadata_scrub_plan_uses_stable_profile_values() {
        let intent = OperationIntent::mutation(
            OperationSource::Cli,
            DocumentId::new(),
            OperationKind::PlanMutation,
            "metadata_scrub",
        );
        let plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::Scrub {
                mode: MetadataScrubMode::CleanShare,
            }],
        );

        assert_eq!(
            plan.operations,
            vec![PatchOperation::SetMetadata {
                key: "metadata_scrub_mode".into(),
                value: "clean_share".into()
            }]
        );
        assert_eq!(plan.write_mode, WriteMode::SanitizingRewrite);
        assert!(!plan.approved_for_apply);
    }

    #[test]
    fn metadata_diff_reports_changed_fields() {
        let before = metadata_snapshot_bytes(&minimal_pdf_bytes(Some(dictionary! {
            "Title" => lopdf::text_string("Before"),
            "Author" => lopdf::text_string("Local User"),
        })));
        let after = metadata_snapshot_bytes(&minimal_pdf_bytes(Some(dictionary! {
            "Title" => lopdf::text_string("After"),
            "Author" => lopdf::text_string("Local User"),
        })));

        let diff = diff_metadata_snapshots(before, after);

        assert!(
            diff.changes
                .iter()
                .any(|change| change.field == "document_info.title")
        );
        assert!(
            !diff
                .changes
                .iter()
                .any(|change| change.field == "document_info.author")
        );
    }

    #[test]
    fn metadata_profile_parser_rejects_unknown_values() {
        assert_eq!(
            MetadataScrubMode::parse_profile("clean-share").unwrap(),
            MetadataScrubMode::CleanShare
        );
        assert_eq!(
            MetadataScrubMode::parse_profile("forensic-preserve").unwrap(),
            MetadataScrubMode::ForensicPreserve
        );
        assert!(MetadataScrubMode::parse_profile("unknown").is_err());
    }

    #[test]
    fn forensic_preserve_mode_is_no_write_read_only() {
        let intent = OperationIntent::read_only(
            fe_reader_core::OperationSource::Cli,
            fe_reader_core::DocumentId::new(),
            "metadata_scrub_forensic_preserve",
        );
        let plan = plan_metadata_operations(
            &intent,
            &[MetadataOperation::Scrub {
                mode: MetadataScrubMode::ForensicPreserve,
            }],
        );
        assert_eq!(plan.write_mode, fe_reader_core::WriteMode::NoWrite);
        assert_eq!(plan.risk_level, fe_reader_core::RiskLevel::ReadOnly);
        assert_eq!(plan.operations, vec![fe_reader_core::PatchOperation::Noop]);
    }

    #[test]
    fn inspect_minimal_pdf_without_metadata() {
        let summary = inspect_metadata_bytes(&minimal_pdf_bytes(None));

        assert_eq!(summary.document_info, DocumentInfo::default());
        assert!(!summary.xmp_metadata_present);
        assert!(summary.xmp_streams.is_empty());
        assert!(summary.conformance_claims.is_empty());
        assert!(summary.warnings.is_empty());
        assert!(summary.parser_error.is_none());
        assert!(summary.trailer_keys.contains(&"Root".to_string()));
        assert!(!summary.trailer_keys.contains(&"Info".to_string()));
    }

    #[test]
    fn inspect_info_dictionary_fields() {
        let info = dictionary! {
            "Title" => lopdf::text_string("Fe Reader"),
            "Author" => lopdf::text_string("Local User"),
            "Subject" => lopdf::text_string("Metadata"),
            "Keywords" => lopdf::text_string("pdf, local; private"),
            "Creator" => lopdf::text_string("Fe Tests"),
            "Producer" => lopdf::text_string("lopdf fixture"),
        };
        let summary = inspect_metadata_bytes(&minimal_pdf_bytes(Some(info)));

        assert_eq!(summary.document_info.title.as_deref(), Some("Fe Reader"));
        assert_eq!(summary.document_info.author.as_deref(), Some("Local User"));
        assert_eq!(summary.document_info.subject.as_deref(), Some("Metadata"));
        assert_eq!(
            summary.document_info.keywords,
            vec!["pdf", "local", "private"]
        );
        assert_eq!(summary.document_info.creator.as_deref(), Some("Fe Tests"));
        assert_eq!(
            summary.document_info.producer.as_deref(),
            Some("lopdf fixture")
        );
        assert!(summary.trailer_keys.contains(&"Info".to_string()));
        assert!(summary.trailer_keys.contains(&"Root".to_string()));
        assert!(summary.conformance_claims.is_empty());
        assert!(summary.warnings.is_empty());
        assert!(summary.parser_error.is_none());
    }

    #[test]
    fn inspect_malformed_pdf_reports_non_fatal_parser_error() {
        let summary = inspect_metadata_bytes(b"%PDF-1.7\nnot enough structure\n%%EOF");

        assert_eq!(summary.document_info, DocumentInfo::default());
        assert!(!summary.xmp_metadata_present);
        assert!(summary.xmp_streams.is_empty());
        assert!(summary.trailer_keys.is_empty());
        assert!(summary.parser_error.is_some());
    }

    #[test]
    fn inspect_xmp_metadata_streams_without_mutating_bytes() {
        let xmp = br#"<?xpacket begin=""?><rdf:RDF><rdf:Description dc:title="Fe Reader"/></rdf:RDF><?xpacket end="w"?>"#;
        let summary = inspect_metadata_bytes(&minimal_pdf_bytes_with_xmp(xmp));

        assert!(summary.xmp_metadata_present);
        assert_eq!(summary.xmp_streams.len(), 1);
        assert_eq!(summary.xmp_streams[0].byte_len, xmp.len());
        assert_eq!(summary.xmp_streams[0].sha256_hex, sha256_hex(xmp));
        assert!(summary.xmp_streams[0].preview.contains("rdf:RDF"));
        assert!(summary.xmp_streams[0].decode_error.is_none());
    }

    #[test]
    fn inspect_pdf_2_0_features_reports_claims() {
        let mut document = Document::with_version("2.0");
        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
            "StructTreeRoot" => dictionary! {},
            "OutputIntents" => lopdf::Object::Array(vec![]),
            "AF" => lopdf::Object::Array(vec![]),
        });
        document.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();

        let summary = inspect_metadata_bytes(&bytes);

        assert_eq!(
            summary.conformance_claims,
            vec![
                "associated-files".to_string(),
                "output-intents".to_string(),
                "pdf-2.0".to_string(),
                "tagged-pdf".to_string(),
            ]
        );
        assert_eq!(
            summary.warnings,
            vec!["PDF 2.0 document does not expose an XMP metadata stream".to_string()]
        );
    }

    fn minimal_pdf_bytes(info: Option<Dictionary>) -> Vec<u8> {
        let mut document = Document::with_version("1.7");
        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
        });
        document.trailer.set("Root", catalog_id);

        if let Some(info) = info {
            let info_id = document.add_object(info);
            document.trailer.set("Info", info_id);
        }

        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }

    fn minimal_pdf_bytes_with_xmp(xmp: &[u8]) -> Vec<u8> {
        let mut document = Document::with_version("1.7");
        let metadata_id = document.add_object(lopdf::Stream::new(
            dictionary! {
                "Type" => "Metadata",
                "Subtype" => "XML",
            },
            xmp.to_vec(),
        ));
        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
            "Metadata" => metadata_id,
        });
        document.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }
}
