//! Wave 0 scaffold for `fe_reader_redaction`.
//!
//! This crate is intentionally minimal in v7. Keep core contracts compiling before adding
//! PDF parsing, rendering, platform, plugin, or automation dependencies.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a stable identity string for diagnostics.
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }
}
