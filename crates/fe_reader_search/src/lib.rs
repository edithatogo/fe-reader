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
    spans
        .iter()
        .filter_map(|span| {
            let haystack = if query.case_sensitive {
                span.text.clone()
            } else {
                span.text.to_lowercase()
            };
            haystack.find(&needle).map(|char_offset| SearchHit {
                page_index: span.page_index,
                bbox: span.bbox,
                text: span.text.clone(),
                char_offset,
            })
        })
        .collect()
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_case_insensitive_hits() {
        let spans = vec![TextSpan {
            page_index: PageIndex(2),
            text: "Fe Reader".to_string(),
            bbox: PdfRect::new(1.0, 2.0, 3.0, 4.0),
            reading_order: Some(0),
            font_name: None,
        }];
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
}
