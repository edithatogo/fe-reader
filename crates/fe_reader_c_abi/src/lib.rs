//! Wave 0 C ABI fallback for .NET and other native callers.
//!
//! This crate intentionally exposes only identity, capability and plan-only contract probes. It
//! does not expose document mutation, apply, file IO, rendering, platform automation or plugin
//! hooks.

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentId, OperationIntent, OperationSource, PatchOperation, PatchPlan, RiskLevel, WriteMode,
};
use std::ffi::{c_char, c_int};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// C ABI major version. Increment for breaking ABI layout changes.
pub const ABI_VERSION_MAJOR: u32 = 0;

/// C ABI minor version. Increment for additive ABI changes.
pub const ABI_VERSION_MINOR: u32 = 1;

/// C ABI patch version. Increment for compatible fixes.
pub const ABI_VERSION_PATCH: u32 = 0;

/// Status code returned by successful ABI probes.
pub const FE_READER_C_ABI_STATUS_OK: c_int = 0;

/// Status code reserved for unsupported operations.
pub const FE_READER_C_ABI_STATUS_UNSUPPORTED: c_int = 1;

/// Status code reserved for mutation/apply attempts that this fallback does not expose.
pub const FE_READER_C_ABI_STATUS_MUTATION_NOT_EXPOSED: c_int = 2;

/// Static JSON contract exposed by the C ABI.
pub const CONTRACT_JSON: &str = r#"{"surface":"c_abi","crate":"fe_reader_c_abi","version":"0.1.0","stability":"preview","mutation_policy":"read_only_or_plan_only","csharp_strategy":"p_invoke_fallback","exports":["fe_reader_c_abi_version_major","fe_reader_c_abi_version_minor","fe_reader_c_abi_version_patch","fe_reader_c_abi_contract_json","fe_reader_c_abi_supports_apply","fe_reader_c_abi_supports_plan_only","fe_reader_c_abi_plan_noop_contract"],"notes":["No apply path is exposed in Wave 0.","C# wrappers must preserve OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt."]}"#;
const CONTRACT_JSON_NUL: &str = concat!(
    r#"{"surface":"c_abi","crate":"fe_reader_c_abi","version":"0.1.0","stability":"preview","mutation_policy":"read_only_or_plan_only","csharp_strategy":"p_invoke_fallback","exports":["fe_reader_c_abi_version_major","fe_reader_c_abi_version_minor","fe_reader_c_abi_version_patch","fe_reader_c_abi_contract_json","fe_reader_c_abi_supports_apply","fe_reader_c_abi_supports_plan_only","fe_reader_c_abi_plan_noop_contract"],"notes":["No apply path is exposed in Wave 0.","C# wrappers must preserve OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt."]}"#,
    "\0"
);

/// C ABI risk level values.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeReaderCAbiRiskLevel {
    /// Inspection only.
    ReadOnly = 0,
    /// Reversible local state change.
    LocalState = 1,
    /// PDF mutation that needs review.
    DocumentMutation = 2,
    /// High-risk mutation such as redaction or signing.
    HighRisk = 3,
}

/// C ABI write mode values.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeReaderCAbiWriteMode {
    /// No document bytes are written.
    NoWrite = 0,
    /// Append-only revision.
    IncrementalAppend = 1,
    /// Full document rewrite.
    FullRewrite = 2,
    /// Full rewrite intended to remove old revisions or unreachable sensitive content.
    SanitizingRewrite = 3,
}

/// Plan-only C ABI contract probe.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeReaderCAbiPlanContract {
    /// ABI major version used by this struct layout.
    pub abi_version_major: u32,
    /// Risk level encoded as `FeReaderCAbiRiskLevel`.
    pub risk_level: u32,
    /// Write mode encoded as `FeReaderCAbiWriteMode`.
    pub write_mode: u32,
    /// Whether this plan may be applied without further review.
    pub approved_for_apply: u32,
    /// Number of planned operations.
    pub operation_count: u32,
    /// Status code for this probe.
    pub status: c_int,
}

/// Returns the crate identity for Rust-side diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{CRATE_NAME}@{CRATE_VERSION}")
}

/// Builds the Wave 0 no-op plan contract using the core mutation pipeline primitives.
#[must_use]
pub fn noop_plan_contract() -> FeReaderCAbiPlanContract {
    let intent = OperationIntent::read_only(
        OperationSource::Automation,
        DocumentId("c-abi-contract-probe".to_owned()),
        "c_abi_noop_contract",
    );
    let plan = PatchPlan::draft(&intent, "C ABI no-op contract", vec![PatchOperation::Noop]);

    FeReaderCAbiPlanContract {
        abi_version_major: ABI_VERSION_MAJOR,
        risk_level: c_abi_risk_level(plan.risk_level) as u32,
        write_mode: c_abi_write_mode(plan.write_mode) as u32,
        approved_for_apply: u32::from(plan.approved_for_apply),
        operation_count: plan.operations.len() as u32,
        status: FE_READER_C_ABI_STATUS_OK,
    }
}

/// Returns the C ABI major version.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_version_major() -> u32 {
    ABI_VERSION_MAJOR
}

/// Returns the C ABI minor version.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_version_minor() -> u32 {
    ABI_VERSION_MINOR
}

/// Returns the C ABI patch version.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_version_patch() -> u32 {
    ABI_VERSION_PATCH
}

/// Returns a null-terminated JSON contract string owned by the library for the process lifetime.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_contract_json() -> *const c_char {
    CONTRACT_JSON_NUL.as_ptr().cast()
}

/// Returns whether this fallback ABI exposes an apply path.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_supports_apply() -> u32 {
    0
}

/// Returns whether this fallback ABI exposes plan-only probes.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_supports_plan_only() -> u32 {
    1
}

/// Returns the no-op plan contract probe.
#[unsafe(no_mangle)]
pub extern "C" fn fe_reader_c_abi_plan_noop_contract() -> FeReaderCAbiPlanContract {
    noop_plan_contract()
}

fn c_abi_risk_level(risk_level: RiskLevel) -> FeReaderCAbiRiskLevel {
    match risk_level {
        RiskLevel::ReadOnly => FeReaderCAbiRiskLevel::ReadOnly,
        RiskLevel::LocalState => FeReaderCAbiRiskLevel::LocalState,
        RiskLevel::DocumentMutation => FeReaderCAbiRiskLevel::DocumentMutation,
        RiskLevel::HighRisk => FeReaderCAbiRiskLevel::HighRisk,
    }
}

fn c_abi_write_mode(write_mode: WriteMode) -> FeReaderCAbiWriteMode {
    match write_mode {
        WriteMode::NoWrite => FeReaderCAbiWriteMode::NoWrite,
        WriteMode::IncrementalAppend => FeReaderCAbiWriteMode::IncrementalAppend,
        WriteMode::FullRewrite => FeReaderCAbiWriteMode::FullRewrite,
        WriteMode::SanitizingRewrite => FeReaderCAbiWriteMode::SanitizingRewrite,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn contract_json_declares_plan_only_policy() {
        assert!(CONTRACT_JSON.contains("\"surface\":\"c_abi\""));
        assert!(CONTRACT_JSON.contains("\"mutation_policy\":\"read_only_or_plan_only\""));
        assert!(CONTRACT_JSON.contains("No apply path"));
    }

    #[test]
    fn exported_contract_json_pointer_is_present() {
        assert!(!fe_reader_c_abi_contract_json().is_null());
    }

    #[test]
    fn abi_versions_match_preview_contract() {
        assert_eq!(fe_reader_c_abi_version_major(), ABI_VERSION_MAJOR);
        assert_eq!(fe_reader_c_abi_version_minor(), ABI_VERSION_MINOR);
        assert_eq!(fe_reader_c_abi_version_patch(), ABI_VERSION_PATCH);
    }

    #[test]
    fn noop_plan_contract_is_read_only_and_not_approved() {
        let contract = fe_reader_c_abi_plan_noop_contract();

        assert_eq!(contract.abi_version_major, ABI_VERSION_MAJOR);
        assert_eq!(contract.risk_level, FeReaderCAbiRiskLevel::ReadOnly as u32);
        assert_eq!(contract.write_mode, FeReaderCAbiWriteMode::NoWrite as u32);
        assert_eq!(contract.approved_for_apply, 0);
        assert_eq!(contract.operation_count, 1);
        assert_eq!(contract.status, FE_READER_C_ABI_STATUS_OK);
        assert_eq!(fe_reader_c_abi_supports_apply(), 0);
        assert_eq!(fe_reader_c_abi_supports_plan_only(), 1);
    }
}
