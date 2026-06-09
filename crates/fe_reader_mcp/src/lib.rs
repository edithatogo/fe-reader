//! Read-only and plan-only MCP contract surface.
//!
//! This crate intentionally exposes contract metadata and local planning behavior only.
//! It does not start a network server or expose mutation tools in Wave 5.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentId, FeError, FeErrorKind, OperationIntent, OperationKind, OperationSource, PatchPlan,
    RiskLevel,
};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// MCP tool risk declared in the public tool manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpToolRisk {
    /// Inspection-only tool.
    ReadOnly,
    /// Tool returns a reviewable plan but cannot apply it.
    PlanOnly,
    /// Approved mutation exists in the protocol but is disabled by default.
    ApprovedMutationDisabled,
}

/// MCP tool descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpToolDescriptor {
    /// Public MCP tool name.
    pub name: String,
    /// Risk class.
    pub risk: McpToolRisk,
    /// Short description.
    pub description: String,
}

impl McpToolDescriptor {
    /// Returns true when the tool may run without review.
    #[must_use]
    pub fn is_read_only(&self) -> bool {
        self.risk == McpToolRisk::ReadOnly
    }
}

/// Result of a contract-level MCP tool plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpToolPlan {
    /// Tool descriptor.
    pub tool: McpToolDescriptor,
    /// Operation intent created for policy evaluation.
    pub intent: OperationIntent,
    /// Optional plan for plan-only tools.
    pub patch_plan: Option<PatchPlan>,
    /// Whether execution is disabled by default.
    pub disabled_by_default: bool,
}

/// Returns the Wave 5 read-only/plan-only MCP tool manifest.
#[must_use]
pub fn read_only_tool_manifest() -> Vec<McpToolDescriptor> {
    vec![
        McpToolDescriptor {
            name: "fe.open_document".to_string(),
            risk: McpToolRisk::ReadOnly,
            description: "Open a document and return a local document id.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.read_page_text".to_string(),
            risk: McpToolRisk::ReadOnly,
            description: "Return text spans for a page.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.get_metadata".to_string(),
            risk: McpToolRisk::ReadOnly,
            description: "Return metadata snapshot.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.search".to_string(),
            risk: McpToolRisk::ReadOnly,
            description: "Search extracted text.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.plan_redaction".to_string(),
            risk: McpToolRisk::PlanOnly,
            description: "Return a reviewable redaction patch plan.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.plan_workflow".to_string(),
            risk: McpToolRisk::PlanOnly,
            description: "Return a reviewable workflow patch plan.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.plan_conversion".to_string(),
            risk: McpToolRisk::PlanOnly,
            description: "Return a reviewable conversion patch plan.".to_string(),
        },
        McpToolDescriptor {
            name: "fe.apply_approved_patch".to_string(),
            risk: McpToolRisk::ApprovedMutationDisabled,
            description:
                "Disabled by default; requires document hash, plan id, policy and approval token."
                    .to_string(),
        },
    ]
}

/// Plans an MCP tool invocation without performing document I/O or mutation.
///
/// # Errors
///
/// Returns an error for unknown tools.
pub fn plan_mcp_tool(
    name: &str,
    document_id: DocumentId,
    label: impl Into<String>,
) -> Result<McpToolPlan, FeError> {
    let tool = read_only_tool_manifest()
        .into_iter()
        .find(|tool| tool.name == name)
        .ok_or_else(|| {
            FeError::new(
                FeErrorKind::InvalidInput,
                format!("unknown MCP tool: {name}"),
            )
        })?;
    let label = label.into();
    let (intent, patch_plan, disabled_by_default) = match tool.risk {
        McpToolRisk::ReadOnly => (
            OperationIntent::read_only(OperationSource::Mcp, document_id, label),
            None,
            false,
        ),
        McpToolRisk::PlanOnly => {
            let intent = OperationIntent::new(
                OperationSource::Mcp,
                document_id,
                OperationKind::PlanMutation,
                label,
                RiskLevel::DocumentMutation,
            );
            let plan = PatchPlan::draft(&intent, "MCP plan-only request", Vec::new());
            (intent, Some(plan), false)
        }
        McpToolRisk::ApprovedMutationDisabled => {
            let intent = OperationIntent::high_risk(
                OperationSource::Mcp,
                document_id,
                OperationKind::ApplyPatch,
                label,
            );
            (intent, None, true)
        }
    };
    Ok(McpToolPlan {
        tool,
        intent,
        patch_plan,
        disabled_by_default,
    })
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
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn manifest_contains_read_only_tools() {
        let manifest = read_only_tool_manifest();
        for name in [
            "fe.open_document",
            "fe.read_page_text",
            "fe.get_metadata",
            "fe.search",
        ] {
            let tool = manifest
                .iter()
                .find(|tool| tool.name == name)
                .expect("tool");
            assert!(tool.is_read_only());
        }
    }

    #[test]
    fn plan_only_tools_return_unapproved_patch_plan() {
        let plan = plan_mcp_tool(
            "fe.plan_workflow",
            DocumentId("doc-mcp".to_string()),
            "plan_workflow",
        )
        .expect("MCP plan");
        assert_eq!(plan.intent.source, OperationSource::Mcp);
        assert!(plan.intent.requires_review);
        let patch_plan = plan.patch_plan.expect("patch plan");
        assert!(!patch_plan.approved_for_apply);
    }

    #[test]
    fn approved_apply_tool_is_disabled_by_default() {
        let plan = plan_mcp_tool(
            "fe.apply_approved_patch",
            DocumentId("doc-mcp".to_string()),
            "apply_approved_patch",
        )
        .expect("MCP disabled apply");
        assert!(plan.disabled_by_default);
        assert_eq!(plan.intent.risk_level, RiskLevel::HighRisk);
    }
}
