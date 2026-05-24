//! Contract sketch for external oracle comparison.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialOracleRun {
    pub run_id: String,
    pub fixture_id: String,
    pub operation: OracleOperation,
    pub tools: Vec<OracleToolResult>,
    pub comparison: OracleComparison,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleOperation {
    SyntaxValidity,
    RenderVisualSimilarity,
    TextExtractionSimilarity,
    MetadataRoundtrip,
    RedactionLeakAbsence,
    RepairSemanticDelta,
    ConversionOutputQuality,
    AccessibilityValidation,
    PrepressValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleToolResult {
    pub tool: String,
    pub version: Option<String>,
    pub exit_code: Option<i32>,
    pub normalized_output_sha256: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleComparison {
    pub status: OracleStatus,
    pub disagreement_class: Option<DisagreementClass>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleStatus { Match, AcceptableDelta, Disagreement, OracleUnavailable }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisagreementClass {
    FeReaderBug,
    OracleBugOrLimitation,
    SpecAmbiguous,
    FixtureInvalid,
    KnownFeatureGap,
    SecurityPolicyDifference,
}
