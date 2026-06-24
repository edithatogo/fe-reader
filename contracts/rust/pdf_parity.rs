//! Exhaustive PDF parity taxonomy and contract registry.
//!
//! This contract keeps public capability claims, evidence requirements and
//! support levels machine-readable so marketing and release docs can be
//! validated against one source of truth.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PdfParitySupportLevel {
    Stable,
    Beta,
    Preview,
    PlanOnly,
    OracleOnly,
    Blocked,
    DocumentedLimitation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfParityFamily {
    pub id: String,
    pub title: String,
    pub description: String,
    pub default_support_level: PdfParitySupportLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfParityClaim {
    pub id: String,
    pub family: String,
    pub public_claim: String,
    pub support_level: PdfParitySupportLevel,
    pub evidence: Vec<String>,
    pub oracle_requirements: Vec<String>,
    pub fixtures: Vec<String>,
    pub limitations: Vec<String>,
    pub requires_mutation_pipeline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfParityRegistry {
    pub registry_version: String,
    pub feature_gate: String,
    pub mutation_pipeline: String,
    pub support_levels: Vec<PdfParitySupportLevel>,
    pub evidence_classes: Vec<String>,
    pub oracle_requirements: Vec<String>,
    pub families: Vec<PdfParityFamily>,
    pub claims: Vec<PdfParityClaim>,
}
