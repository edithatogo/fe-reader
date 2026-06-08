//! Deterministic search contracts and a small in-memory search implementation.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_pdf_model::{PageIndex, PdfRect, TextSpan};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Search query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Raw query string.
    pub text: String,
    /// Whether matching should be case-sensitive.
    pub case_sensitive: bool,
}

/// Search result with page and geometry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    /// Page index.
    pub page_index: PageIndex,
    /// Bounding box of the matched span.
    pub bbox: PdfRect,
    /// Matched text span.
    pub text: String,
    /// Character offset inside the span.
    pub char_offset: usize,
}

/// Schema-compatible deterministic search index record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexRecord {
    /// Stable document identifier.
    pub document_id: String,
    /// SHA-256 of indexed document bytes.
    pub document_sha256: String,
    /// Zero-based page index.
    pub page_index: u32,
    /// Stable span id within the document.
    pub span_id: String,
    /// Indexed text.
    pub text: String,
    /// Bounding box as `[x, y, width, height]`.
    pub bbox: [f32; 4],
    /// Reading order for deterministic sorting.
    pub reading_order: u32,
    /// Optional language hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_hint: Option<String>,
}

/// Builds deterministic search index records from extracted spans.
#[must_use]
pub fn build_search_index_records(
    document_id: &str,
    document_sha256: &str,
    spans: &[TextSpan],
    language_hint: Option<&str>,
) -> Vec<SearchIndexRecord> {
    spans
        .iter()
        .enumerate()
        .map(|(span_index, span)| {
            let reading_order = span.reading_order.unwrap_or(span_index as u32);
            SearchIndexRecord {
                document_id: document_id.to_string(),
                document_sha256: document_sha256.to_string(),
                page_index: span.page_index.0,
                span_id: format!("page-{}-span-{reading_order}", span.page_index.0),
                text: span.text.clone(),
                bbox: [span.bbox.x, span.bbox.y, span.bbox.width, span.bbox.height],
                reading_order,
                language_hint: language_hint.map(str::to_string),
            }
        })
        .collect()
}

/// Runs deterministic substring search over extracted spans.
#[must_use]
pub fn search_spans(spans: &[TextSpan], query: &SearchQuery) -> Vec<SearchHit> {
    if query.text.is_empty() {
        return Vec::new();
    }
    let needle = if query.case_sensitive {
        query.text.clone()
    } else {
        query.text.to_lowercase()
    };
    let mut hits = Vec::new();
    for span in spans {
        let haystack = if query.case_sensitive {
            span.text.clone()
        } else {
            span.text.to_lowercase()
        };
        let mut search_start = 0;
        while let Some(relative_byte_offset) = haystack[search_start..].find(&needle) {
            let byte_offset = search_start + relative_byte_offset;
            hits.push(SearchHit {
                page_index: span.page_index,
                bbox: span.bbox,
                text: span.text.clone(),
                char_offset: haystack[..byte_offset].chars().count(),
            });
            search_start = byte_offset + needle.len();
        }
    }
    hits
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(page: u32, text: &str, bbox: PdfRect, reading_order: Option<u32>) -> TextSpan {
        TextSpan {
            page_index: PageIndex(page),
            text: text.to_string(),
            bbox,
            reading_order,
            font_name: None,
        }
    }

    #[test]
    fn finds_case_insensitive_hits() {
        let spans = vec![span(
            2,
            "Fe Reader",
            PdfRect::new(1.0, 2.0, 3.0, 4.0),
            Some(0),
        )];
        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: "reader".to_string(),
                case_sensitive: false,
            },
        );
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].page_index, PageIndex(2));
    }

    #[test]
    fn returns_multiple_hits_inside_a_span() {
        let spans = vec![span(
            0,
            "Reader, reader, READER",
            PdfRect::new(10.0, 20.0, 30.0, 40.0),
            Some(0),
        )];

        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: "reader".to_string(),
                case_sensitive: false,
            },
        );

        assert_eq!(hits.len(), 3);
        assert_eq!(
            hits.iter().map(|hit| hit.char_offset).collect::<Vec<_>>(),
            vec![0, 8, 16]
        );
    }

    #[test]
    fn respects_case_sensitive_queries() {
        let spans = vec![span(
            0,
            "Reader reader",
            PdfRect::new(0.0, 0.0, 5.0, 5.0),
            Some(0),
        )];

        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: "reader".to_string(),
                case_sensitive: true,
            },
        );

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].char_offset, 7);
        assert_eq!(hits[0].text, "Reader reader");
    }

    #[test]
    fn returns_no_hits_for_empty_query() {
        let spans = vec![span(0, "Reader", PdfRect::new(0.0, 0.0, 5.0, 5.0), Some(0))];

        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: String::new(),
                case_sensitive: false,
            },
        );

        assert!(hits.is_empty());
    }

    #[test]
    fn preserves_span_bounding_boxes() {
        let bbox = PdfRect::new(12.0, 24.0, 36.0, 48.0);
        let spans = vec![span(3, "Reader", bbox, Some(5))];

        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: "reader".to_string(),
                case_sensitive: false,
            },
        );

        assert_eq!(hits[0].bbox, bbox);
        assert_eq!(hits[0].page_index, PageIndex(3));
    }

    #[test]
    fn builds_schema_shaped_index_records() {
        let spans = vec![span(
            0,
            "Fe Reader Search Fixture\n",
            PdfRect::new(0.0, 0.0, 612.0, 792.0),
            Some(0),
        )];

        let records =
            build_search_index_records("fixture:text-search-fixture", "abc123", &spans, Some("en"));

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].document_id, "fixture:text-search-fixture");
        assert_eq!(records[0].document_sha256, "abc123");
        assert_eq!(records[0].page_index, 0);
        assert_eq!(records[0].span_id, "page-0-span-0");
        assert_eq!(records[0].bbox, [0.0, 0.0, 612.0, 792.0]);
        assert_eq!(records[0].reading_order, 0);
        assert_eq!(records[0].language_hint.as_deref(), Some("en"));
    }

    #[test]
    fn keeps_input_span_and_match_order_stable() {
        let spans = vec![
            span(1, "b hit hit", PdfRect::new(1.0, 0.0, 1.0, 1.0), Some(1)),
            span(0, "a hit", PdfRect::new(0.0, 0.0, 1.0, 1.0), Some(0)),
        ];

        let hits = search_spans(
            &spans,
            &SearchQuery {
                text: "hit".to_string(),
                case_sensitive: true,
            },
        );

        assert_eq!(
            hits.iter()
                .map(|hit| (hit.page_index, hit.char_offset, hit.bbox.x))
                .collect::<Vec<_>>(),
            vec![
                (PageIndex(1), 2, 1.0),
                (PageIndex(1), 6, 1.0),
                (PageIndex(0), 2, 0.0)
            ]
        );
    }
}
