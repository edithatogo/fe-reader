//! Plan-only conversion provider contracts.
//!
//! Wave 4 establishes provider discovery and policy-compatible planning. It deliberately
//! does not execute Pandoc, LibreOffice, Typst, Quarto, LaTeX or other external tools.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentId, FeError, FeErrorKind, OperationIntent, OperationKind, OperationReceipt,
    OperationSource, PatchPlan, ResourceLimits,
};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Input kind accepted by a conversion provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionInputKind {
    /// PDF document input.
    Pdf,
    /// Markdown source input.
    Markdown,
    /// Quarto source input.
    Quarto,
    /// Typst source input.
    Typst,
    /// LaTeX source input.
    Latex,
    /// DOCX input.
    Docx,
    /// ODT input.
    Odt,
    /// One or more images.
    ImageSet,
}

/// Output kind produced by a conversion provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionOutputKind {
    /// PDF document output.
    Pdf,
    /// Plain text output.
    Text,
    /// Markdown output.
    Markdown,
    /// HTML output.
    Html,
    /// DOCX output.
    Docx,
    /// ODT output.
    Odt,
    /// JSON output.
    Json,
    /// PNG image output.
    Png,
    /// SVG output.
    Svg,
    /// Typst source output.
    Typst,
    /// LaTeX source output.
    Latex,
    /// Quarto source output.
    Quarto,
}

/// Stable conversion job shape matching `schemas/conversion-job.schema.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionJob {
    /// Stable job id from the caller.
    pub job_id: String,
    /// Input document or source kind.
    pub input_kind: ConversionInputKind,
    /// Requested output kind.
    pub output_kind: ConversionOutputKind,
    /// Local input URI or path-like reference.
    pub input_uri: String,
    /// Optional output URI. Providers may require this before apply.
    pub output_uri: Option<String>,
    /// Whether metadata should be preserved where the provider supports it.
    pub preserve_metadata: bool,
    /// Whether a source map should be created where supported.
    pub create_source_map: bool,
    /// Optional provider requested by the caller.
    pub provider_hint: Option<String>,
    /// Provider-specific options. They remain inert until policy approves execution.
    #[serde(default)]
    pub options: serde_json::Value,
}

impl ConversionJob {
    /// Returns true when the job has the minimum fields needed for plan generation.
    #[must_use]
    pub fn is_planable(&self) -> bool {
        !self.job_id.trim().is_empty() && !self.input_uri.trim().is_empty()
    }
}

/// Provider execution model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionProviderKind {
    /// Internal deterministic provider that does not spawn tools.
    Internal,
    /// External local command-line provider, gated by policy before execution.
    ExternalTool,
}

/// One supported input/output pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionPair {
    /// Supported input kind.
    pub input: ConversionInputKind,
    /// Supported output kind.
    pub output: ConversionOutputKind,
}

impl ConversionPair {
    /// Creates a supported pair.
    #[must_use]
    pub const fn new(input: ConversionInputKind, output: ConversionOutputKind) -> Self {
        Self { input, output }
    }
}

/// Advertised provider capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionProviderCapability {
    /// Stable provider id.
    pub provider_id: String,
    /// Human-readable provider name.
    pub display_name: String,
    /// Provider execution model.
    pub provider_kind: ConversionProviderKind,
    /// Supported input/output pairs.
    pub supported_pairs: Vec<ConversionPair>,
    /// Whether the provider can preserve metadata.
    pub preserves_metadata: bool,
    /// Whether the provider can produce a source map.
    pub creates_source_map: bool,
    /// Whether the provider is currently executable in this environment.
    pub executable: bool,
    /// Deterministic reason when the provider cannot execute.
    pub unavailable_reason: Option<String>,
}

impl ConversionProviderCapability {
    /// Returns whether this provider supports the requested job.
    #[must_use]
    pub fn supports(&self, job: &ConversionJob) -> bool {
        self.supported_pairs
            .iter()
            .any(|pair| pair.input == job.input_kind && pair.output == job.output_kind)
    }
}

/// Provider plan produced before any conversion process may run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionProviderPlan {
    /// Planned provider capability.
    pub capability: ConversionProviderCapability,
    /// Planned job.
    pub job: ConversionJob,
    /// Resource limits that would be applied if execution is later approved.
    pub resource_limits: ResourceLimits,
    /// Whether policy approval is required before execution.
    pub requires_policy_approval: bool,
    /// Whether interactive confirmation or an approval token is required before execution.
    pub requires_user_approval: bool,
    /// Deterministic warnings surfaced to users and automation clients.
    pub warnings: Vec<String>,
}

/// Full mutation-pipeline-compatible conversion plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlannedConversion {
    /// High-risk export/external-tool intent.
    pub intent: OperationIntent,
    /// Empty patch plan used as the reviewable policy object for conversion execution.
    pub patch_plan: PatchPlan,
    /// Plan-stage audit receipt.
    pub audit_receipt: OperationReceipt,
    /// Provider-specific plan.
    pub provider_plan: ConversionProviderPlan,
}

/// Conversion provider contract.
pub trait ConversionProvider {
    /// Returns advertised capability without executing the provider.
    fn capability(&self) -> ConversionProviderCapability;

    /// Plans the conversion without reading or writing document bytes.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the job is invalid or unsupported.
    fn plan(&self, job: ConversionJob) -> Result<ConversionProviderPlan, FeError> {
        let capability = self.capability();
        if !job.is_planable() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "conversion job id and input_uri are required",
            ));
        }
        if let Some(hint) = &job.provider_hint
            && hint != &capability.provider_id
        {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                format!(
                    "conversion job requested provider '{hint}', not '{}'",
                    capability.provider_id
                ),
            ));
        }
        if !capability.supports(&job) {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                format!(
                    "provider '{}' does not support {:?} to {:?}",
                    capability.provider_id, job.input_kind, job.output_kind
                ),
            ));
        }

        let mut warnings = Vec::new();
        if job.preserve_metadata && !capability.preserves_metadata {
            warnings.push("provider cannot preserve metadata".to_string());
        }
        if job.create_source_map && !capability.creates_source_map {
            warnings.push("provider cannot create a source map".to_string());
        }
        if !capability.executable {
            warnings.push(
                capability
                    .unavailable_reason
                    .clone()
                    .unwrap_or_else(|| "provider execution is unavailable".to_string()),
            );
        }

        Ok(ConversionProviderPlan {
            capability,
            job,
            resource_limits: ResourceLimits::default(),
            requires_policy_approval: true,
            requires_user_approval: true,
            warnings,
        })
    }
}

/// Built-in deterministic contract provider for PDF extraction-style exports.
#[derive(Debug, Clone, Copy, Default)]
pub struct BuiltInPdfExportProvider;

impl ConversionProvider for BuiltInPdfExportProvider {
    fn capability(&self) -> ConversionProviderCapability {
        ConversionProviderCapability {
            provider_id: "builtin_pdf_export_contract".to_string(),
            display_name: "Built-in PDF export contract".to_string(),
            provider_kind: ConversionProviderKind::Internal,
            supported_pairs: vec![
                ConversionPair::new(ConversionInputKind::Pdf, ConversionOutputKind::Text),
                ConversionPair::new(ConversionInputKind::Pdf, ConversionOutputKind::Markdown),
                ConversionPair::new(ConversionInputKind::Pdf, ConversionOutputKind::Html),
                ConversionPair::new(ConversionInputKind::Pdf, ConversionOutputKind::Json),
            ],
            preserves_metadata: false,
            creates_source_map: false,
            executable: true,
            unavailable_reason: None,
        }
    }
}

/// Contract-only provider for source pipeline tools such as Pandoc, Quarto, Typst and LaTeX.
#[derive(Debug, Clone, Copy, Default)]
pub struct SourcePipelineContractProvider;

impl ConversionProvider for SourcePipelineContractProvider {
    fn capability(&self) -> ConversionProviderCapability {
        ConversionProviderCapability {
            provider_id: "source_pipeline_contract".to_string(),
            display_name: "Source pipeline contract".to_string(),
            provider_kind: ConversionProviderKind::ExternalTool,
            supported_pairs: vec![
                ConversionPair::new(ConversionInputKind::Markdown, ConversionOutputKind::Pdf),
                ConversionPair::new(ConversionInputKind::Markdown, ConversionOutputKind::Html),
                ConversionPair::new(ConversionInputKind::Quarto, ConversionOutputKind::Pdf),
                ConversionPair::new(ConversionInputKind::Typst, ConversionOutputKind::Pdf),
                ConversionPair::new(ConversionInputKind::Latex, ConversionOutputKind::Pdf),
                ConversionPair::new(ConversionInputKind::Docx, ConversionOutputKind::Pdf),
                ConversionPair::new(ConversionInputKind::Odt, ConversionOutputKind::Pdf),
            ],
            preserves_metadata: true,
            creates_source_map: true,
            executable: false,
            unavailable_reason: Some(
                "external conversion providers require capability discovery and policy approval"
                    .to_string(),
            ),
        }
    }
}

/// Returns Wave 4 conversion providers available for planning.
#[must_use]
pub fn default_conversion_providers() -> Vec<ConversionProviderCapability> {
    vec![
        BuiltInPdfExportProvider.capability(),
        SourcePipelineContractProvider.capability(),
    ]
}

/// Creates a reviewable conversion plan without executing conversion.
///
/// # Errors
///
/// Returns a structured error when the provider rejects the job.
pub fn plan_conversion<P: ConversionProvider>(
    provider: &P,
    document_id: DocumentId,
    job: ConversionJob,
) -> Result<PlannedConversion, FeError> {
    let provider_plan = provider.plan(job)?;
    let intent = OperationIntent::high_risk(
        OperationSource::Cli,
        document_id,
        OperationKind::ExternalTool,
        "plan_conversion",
    );
    let patch_plan = PatchPlan::draft(
        &intent,
        format!(
            "plan conversion job '{}' with provider '{}'",
            provider_plan.job.job_id, provider_plan.capability.provider_id
        ),
        Vec::new(),
    );
    let audit_receipt = OperationReceipt::planned(
        &intent,
        &patch_plan,
        "conversion planned; no document bytes converted or written",
    );

    Ok(PlannedConversion {
        intent,
        patch_plan,
        audit_receipt,
        provider_plan,
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

    fn pdf_text_job() -> ConversionJob {
        ConversionJob {
            job_id: "job-pdf-text".to_string(),
            input_kind: ConversionInputKind::Pdf,
            output_kind: ConversionOutputKind::Text,
            input_uri: "fixtures/minimal/minimal.pdf".to_string(),
            output_uri: Some("target/conversion/minimal.txt".to_string()),
            preserve_metadata: true,
            create_source_map: true,
            provider_hint: None,
            options: serde_json::json!({}),
        }
    }

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn default_providers_expose_supported_pairs() {
        let providers = default_conversion_providers();
        assert_eq!(providers.len(), 2);
        assert!(
            providers
                .iter()
                .any(|provider| provider.supports(&pdf_text_job()))
        );
    }

    #[test]
    fn builtin_pdf_export_plans_without_approval_or_write() {
        let planned = plan_conversion(
            &BuiltInPdfExportProvider,
            DocumentId("doc-conversion".to_string()),
            pdf_text_job(),
        )
        .expect("conversion plan");

        assert_eq!(
            planned.intent.risk_level,
            fe_reader_core::RiskLevel::HighRisk
        );
        assert!(planned.intent.requires_review);
        assert!(!planned.patch_plan.approved_for_apply);
        assert_eq!(
            planned.patch_plan.write_mode,
            fe_reader_core::WriteMode::NoWrite
        );
        assert_eq!(
            planned.audit_receipt.write_mode,
            fe_reader_core::WriteMode::NoWrite
        );
        assert!(planned.provider_plan.requires_policy_approval);
        assert!(planned.provider_plan.requires_user_approval);
        assert!(
            planned
                .provider_plan
                .warnings
                .iter()
                .any(|warning| warning.contains("metadata"))
        );
    }

    #[test]
    fn source_pipeline_provider_reports_external_tool_boundary() {
        let job = ConversionJob {
            job_id: "job-md-pdf".to_string(),
            input_kind: ConversionInputKind::Markdown,
            output_kind: ConversionOutputKind::Pdf,
            input_uri: "docs/example.md".to_string(),
            output_uri: Some("target/conversion/example.pdf".to_string()),
            preserve_metadata: true,
            create_source_map: true,
            provider_hint: Some("source_pipeline_contract".to_string()),
            options: serde_json::json!({"tool": "pandoc"}),
        };

        let plan = SourcePipelineContractProvider
            .plan(job)
            .expect("source pipeline plan");
        assert_eq!(
            plan.capability.provider_kind,
            ConversionProviderKind::ExternalTool
        );
        assert!(!plan.capability.executable);
        assert!(
            plan.warnings
                .iter()
                .any(|warning| warning.contains("policy approval"))
        );
    }

    #[test]
    fn unsupported_conversion_pair_fails_clearly() {
        let mut job = pdf_text_job();
        job.output_kind = ConversionOutputKind::Png;
        let error = BuiltInPdfExportProvider
            .plan(job)
            .expect_err("unsupported pair");
        assert_eq!(error.kind, FeErrorKind::InvalidInput);
        assert!(error.message.contains("does not support"));
    }
}
