//! Versioned sidecar/workspace migration contract.

use serde::{Deserialize, Serialize};

/// Identifies a versioned Fe sidecar format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatVersion {
    /// Format family, e.g. `feworkspace`, `fereceipt`, `fereview`, `fetemplate`.
    pub family: String,
    /// Semantic version of the format.
    pub version: String,
}

/// Describes a migration step between two format versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationStep {
    /// Source format.
    pub from: FormatVersion,
    /// Destination format.
    pub to: FormatVersion,
    /// Whether the migration is lossless.
    pub lossless: bool,
    /// Whether downgrade remains possible after migration.
    pub downgrade_supported: bool,
    /// Human-readable migration summary.
    pub summary: String,
}
