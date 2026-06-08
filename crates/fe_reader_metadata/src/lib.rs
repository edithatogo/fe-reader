//! Metadata planning contracts.
//!
//! Wave 0 does not mutate PDF bytes. It creates explicit plans for metadata editing/scrubbing.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{OperationIntent, PatchOperation, PatchPlan};
use lopdf::{Dictionary, Document, Object};
use serde::{Deserialize, Serialize};
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
    /// Root trailer dictionary keys visible to the parser.
    pub trailer_keys: Vec<String>,
    /// Non-fatal parser error, if the PDF could not be opened for metadata inspection.
    pub parser_error: Option<String>,
}

/// Metadata scrub strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataScrubMode {
    /// Preserve all metadata.
    Preserve,
    /// Remove common private fields but keep user-visible title/subject.
    CleanShare,
    /// Remove all non-essential metadata.
    Aggressive,
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
            MetadataOperation::Scrub { mode } => PatchOperation::SetMetadata {
                key: "metadata_scrub_mode".to_string(),
                value: format!("{mode:?}"),
            },
        })
        .collect();
    PatchPlan::draft(intent, "metadata operations", patch_ops)
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
        trailer_keys: sorted_dictionary_keys(&document.trailer),
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
    document
        .trailer
        .get(b"Root")
        .ok()
        .and_then(|root| document.dereference(root).ok())
        .and_then(|(_, root)| root.as_dict().ok())
        .is_some_and(|catalog| catalog.has(b"Metadata"))
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

fn sorted_dictionary_keys(dictionary: &Dictionary) -> Vec<String> {
    let mut keys = dictionary
        .iter()
        .map(|(key, _)| String::from_utf8_lossy(key).to_string())
        .collect::<Vec<_>>();
    keys.sort();
    keys
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
    fn inspect_minimal_pdf_without_metadata() {
        let summary = inspect_metadata_bytes(&minimal_pdf_bytes(None));

        assert_eq!(summary.document_info, DocumentInfo::default());
        assert!(!summary.xmp_metadata_present);
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
        assert!(summary.parser_error.is_none());
    }

    #[test]
    fn inspect_malformed_pdf_reports_non_fatal_parser_error() {
        let summary = inspect_metadata_bytes(b"%PDF-1.7\nnot enough structure\n%%EOF");

        assert_eq!(summary.document_info, DocumentInfo::default());
        assert!(!summary.xmp_metadata_present);
        assert!(summary.trailer_keys.is_empty());
        assert!(summary.parser_error.is_some());
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
}
