//! Plan-only AcroForm contracts for `fe_reader_forms`.
//!
//! The crate may inspect or plan form work, but it must not apply PDF writes directly.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_core::{
    DocumentFingerprint, DocumentId, FeError, FeErrorKind, OperationIntent, OperationKind,
    OperationSource, PatchOperation, PatchPlan, RiskLevel,
};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A plan-only form fill value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum FormFieldValue {
    /// Text field value.
    Text(String),
    /// Checkbox or radio boolean value.
    Boolean(bool),
    /// Choice field selected export value.
    Choice(String),
}

impl FormFieldValue {
    fn into_plan_value(self) -> String {
        match self {
            Self::Text(value) | Self::Choice(value) => value,
            Self::Boolean(value) => value.to_string(),
        }
    }
}

/// One field-fill request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormFill {
    /// Fully qualified field name.
    pub field_name: String,
    /// Field value.
    pub value: FormFieldValue,
}

/// Options for plan-only form filling.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormFillOptions {
    /// Flatten fields after filling. This stays plan-only until apply/rewrite verification exists.
    pub flatten_after_fill: bool,
}

/// Planned form fill output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedFormFill {
    /// Operation intent.
    pub intent: OperationIntent,
    /// Draft patch plan.
    pub plan: PatchPlan,
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

/// Plans AcroForm fills without applying document writes.
///
/// # Errors
///
/// Returns an error when no fields are provided or a field name/value is blank.
pub fn plan_form_fill(
    source: OperationSource,
    document_id: DocumentId,
    fingerprint: DocumentFingerprint,
    fills: Vec<FormFill>,
    options: FormFillOptions,
) -> Result<PlannedFormFill, FeError> {
    validate_fills(&fills)?;
    let intent = OperationIntent::new(
        source,
        document_id,
        OperationKind::PlanMutation,
        "form_fill",
        RiskLevel::DocumentMutation,
    )
    .with_document_fingerprint(fingerprint);
    let mut operations = Vec::with_capacity(fills.len() + usize::from(options.flatten_after_fill));
    let field_names = fills
        .iter()
        .map(|fill| fill.field_name.clone())
        .collect::<Vec<_>>();
    for fill in fills {
        operations.push(PatchOperation::FillFormField {
            field_name: fill.field_name,
            value: fill.value.into_plan_value(),
        });
    }
    if options.flatten_after_fill {
        operations.push(PatchOperation::FlattenFormFields { field_names });
    }
    let plan = PatchPlan::draft(&intent, "plan form fill", operations);
    Ok(PlannedFormFill { intent, plan })
}

fn validate_fills(fills: &[FormFill]) -> Result<(), FeError> {
    if fills.is_empty() {
        return Err(FeError::new(
            FeErrorKind::InvalidInput,
            "form fill requires at least one field",
        ));
    }
    for fill in fills {
        if fill.field_name.trim().is_empty() {
            return Err(FeError::new(
                FeErrorKind::InvalidInput,
                "form field name must not be blank",
            ));
        }
        match &fill.value {
            FormFieldValue::Text(value) | FormFieldValue::Choice(value)
                if value.trim().is_empty() =>
            {
                return Err(FeError::new(
                    FeErrorKind::InvalidInput,
                    "form field value must not be blank",
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fe_reader_core::{DocumentFingerprint, OperationSource, WriteMode};

    #[test]
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn form_fill_is_plan_only_incremental_append() {
        let planned = plan_form_fill(
            OperationSource::Cli,
            DocumentId::new(),
            DocumentFingerprint::from_bytes(b"%PDF-forms-smoke"),
            vec![FormFill {
                field_name: "patient.name".to_string(),
                value: FormFieldValue::Text("Ada Lovelace".to_string()),
            }],
            FormFillOptions::default(),
        )
        .unwrap();

        assert_eq!(planned.intent.risk_level, RiskLevel::DocumentMutation);
        assert_eq!(planned.plan.write_mode, WriteMode::IncrementalAppend);
        assert!(!planned.plan.approved_for_apply);
        assert_eq!(planned.plan.operations.len(), 1);
    }

    #[test]
    fn flatten_after_fill_escalates_to_full_rewrite() {
        let planned = plan_form_fill(
            OperationSource::Cli,
            DocumentId::new(),
            DocumentFingerprint::from_bytes(b"%PDF-forms-smoke"),
            vec![FormFill {
                field_name: "agree".to_string(),
                value: FormFieldValue::Boolean(true),
            }],
            FormFillOptions {
                flatten_after_fill: true,
            },
        )
        .unwrap();

        assert_eq!(planned.plan.write_mode, WriteMode::FullRewrite);
        assert_eq!(planned.plan.operations.len(), 2);
        assert!(matches!(
            planned.plan.operations.last(),
            Some(PatchOperation::FlattenFormFields { field_names }) if field_names == &vec!["agree".to_string()]
        ));
    }

    #[test]
    fn form_fill_rejects_empty_fields() {
        let err = plan_form_fill(
            OperationSource::Cli,
            DocumentId::new(),
            DocumentFingerprint::from_bytes(b"%PDF-forms-smoke"),
            Vec::new(),
            FormFillOptions::default(),
        )
        .unwrap_err();

        assert_eq!(err.kind, FeErrorKind::InvalidInput);
    }
}
