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

/// Normalises text for deterministic search.
#[must_use]
pub fn normalize_for_search(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
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
}
