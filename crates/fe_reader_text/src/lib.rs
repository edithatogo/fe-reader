//! Deterministic text helpers used before optional ML or semantic features.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Text direction hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextDirection {
    /// Left-to-right text.
    LeftToRight,
    /// Right-to-left text.
    RightToLeft,
    /// Vertical writing.
    Vertical,
    /// Unknown or mixed direction.
    MixedOrUnknown,
}

/// Diagnostic category for caller-provided text content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextExtractionDiagnosticKind {
    /// No text was provided to summarize.
    EmptyInput,
    /// Text contains both left-to-right and right-to-left hints.
    MixedDirection,
    /// Text contains control characters other than common whitespace.
    ControlCharacters,
}

/// Deterministic diagnostic for caller-provided text content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextExtractionDiagnostic {
    /// Diagnostic category.
    pub kind: TextExtractionDiagnosticKind,
    /// Stable human-readable diagnostic message.
    pub message: String,
}

/// Deterministic summary for text supplied by an extraction adapter.
///
/// This type does not parse PDFs or claim extraction support. It records facts about text that
/// another layer has already supplied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextExtractionSummary {
    /// Number of Unicode scalar values in the original text.
    pub char_count: usize,
    /// Number of Unicode scalar values after search normalization.
    pub normalized_char_count: usize,
    /// Direction hint detected from the original text.
    pub direction: TextDirection,
    /// Deterministic diagnostics for the original text.
    pub diagnostics: Vec<TextExtractionDiagnostic>,
}

/// Normalises text for deterministic search.
#[must_use]
pub fn normalize_for_search(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Guesses whether a string contains strong right-to-left characters.
#[must_use]
pub fn detect_direction(input: &str) -> TextDirection {
    let has_rtl = input.chars().any(|ch| matches!(ch as u32, 0x0590..=0x08FF));
    let has_ltr = input.chars().any(|ch| ch.is_ascii_alphabetic());
    match (has_ltr, has_rtl) {
        (true, true) => TextDirection::MixedOrUnknown,
        (false, true) => TextDirection::RightToLeft,
        (true, false) => TextDirection::LeftToRight,
        (false, false) => TextDirection::MixedOrUnknown,
    }
}

/// Summarizes caller-provided text without parsing or repairing PDF content.
#[must_use]
pub fn summarize_extracted_text(input: &str) -> TextExtractionSummary {
    let normalized = normalize_for_search(input);
    let direction = detect_direction(input);
    let mut diagnostics = Vec::new();

    if input.is_empty() {
        diagnostics.push(TextExtractionDiagnostic {
            kind: TextExtractionDiagnosticKind::EmptyInput,
            message: "no text supplied".to_string(),
        });
    }
    if direction == TextDirection::MixedOrUnknown && has_ltr(input) && has_rtl(input) {
        diagnostics.push(TextExtractionDiagnostic {
            kind: TextExtractionDiagnosticKind::MixedDirection,
            message: "text contains left-to-right and right-to-left hints".to_string(),
        });
    }
    if input
        .chars()
        .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
    {
        diagnostics.push(TextExtractionDiagnostic {
            kind: TextExtractionDiagnosticKind::ControlCharacters,
            message: "text contains control characters".to_string(),
        });
    }

    TextExtractionSummary {
        char_count: input.chars().count(),
        normalized_char_count: normalized.chars().count(),
        direction,
        diagnostics,
    }
}

fn has_ltr(input: &str) -> bool {
    input.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn has_rtl(input: &str) -> bool {
    input.chars().any(|ch| matches!(ch as u32, 0x0590..=0x08FF))
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
    fn normalizes_whitespace_and_case() {
        assert_eq!(normalize_for_search("  Fe\nReader\tPDF  "), "fe reader pdf");
    }

    #[test]
    fn detects_rtl_hint() {
        assert_eq!(detect_direction("שלום"), TextDirection::RightToLeft);
    }

    #[test]
    fn summarizes_caller_supplied_text() {
        let summary = summarize_extracted_text("  Fe\nReader  ");

        assert_eq!(summary.char_count, 13);
        assert_eq!(summary.normalized_char_count, 9);
        assert_eq!(summary.direction, TextDirection::LeftToRight);
        assert!(summary.diagnostics.is_empty());
    }

    #[test]
    fn emits_deterministic_text_diagnostics() {
        let summary = summarize_extracted_text("Fe\u{0007} שלום");

        assert_eq!(summary.direction, TextDirection::MixedOrUnknown);
        assert_eq!(
            summary
                .diagnostics
                .iter()
                .map(|diagnostic| &diagnostic.kind)
                .collect::<Vec<_>>(),
            vec![
                &TextExtractionDiagnosticKind::MixedDirection,
                &TextExtractionDiagnosticKind::ControlCharacters
            ]
        );
    }

    #[test]
    fn summarizes_empty_text_without_claiming_extraction_failure() {
        let summary = summarize_extracted_text("");

        assert_eq!(summary.char_count, 0);
        assert_eq!(summary.normalized_char_count, 0);
        assert_eq!(summary.direction, TextDirection::MixedOrUnknown);
        assert_eq!(summary.diagnostics.len(), 1);
        assert_eq!(
            summary.diagnostics[0].kind,
            TextExtractionDiagnosticKind::EmptyInput
        );
    }
}
