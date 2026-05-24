//! Fe Reader CLI Wave 0 harness.

use anyhow::Result;
use clap::{Parser, Subcommand};
use fe_reader_core::{OperationIntent, OperationSource};
use fe_reader_security::{PolicyAction, SecurityPolicy, evaluate_policy};

/// Local-first PDF workflow platform CLI.
#[derive(Debug, Parser)]
#[command(name = "fe-reader", version, about = "Fe Reader Wave 0 CLI harness")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print build and contract information.
    Doctor,
    /// Inspect a PDF path without mutating it.
    Inspect {
        /// Path to a PDF file.
        path: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Validate local JSON schemas and contract scaffolding.
    ValidateSchemas,
    /// Explain the default security decision for a representative action.
    Policy {
        /// Action to evaluate: read, plan, apply, export, external-tool, automation, plugin, network.
        action: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => {
            println!("fe-reader cli: {}", env!("CARGO_PKG_VERSION"));
            println!("core: {}", fe_reader_core::crate_identity());
            println!("pdf_model: {}", fe_reader_pdf_model::crate_identity());
            println!("security: {}", fe_reader_security::crate_identity());
        }
        Command::Inspect { path, json } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                summary.document_id.clone(),
                "inspect",
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = fe_reader_core::PatchPlan::draft(
                &intent,
                format!("inspect {path}"),
                vec![fe_reader_core::PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("PDF {} detected", summary.header.version);
                println!("sha256={}", summary.fingerprint.sha256_hex);
                println!("bytes={}", summary.fingerprint.byte_len);
                println!(
                    "page_count={}",
                    format_page_count(summary.parser.page_count)
                );
                println!("encrypted_hint={}", summary.encrypted_hint);
                println!("linearized_hint={}", summary.linearized_hint);
                println!("parser={}", summary.parser.adapter);
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::ValidateSchemas => {
            println!("schema validation is delegated to scripts/validate_schemas.py in Wave 0");
        }
        Command::Policy { action } => {
            let action = parse_policy_action(&action);
            let decision = evaluate_policy(
                &SecurityPolicy::default(),
                OperationSource::Cli,
                action,
                fe_reader_core::RiskLevel::HighRisk,
            );
            println!("{}", serde_json::to_string_pretty(&decision)?);
        }
    }
    Ok(())
}

fn format_page_count(page_count: Option<u32>) -> String {
    page_count
        .map(|count| count.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn parse_policy_action(action: &str) -> PolicyAction {
    match action {
        "read" => PolicyAction::Read,
        "plan" => PolicyAction::PlanMutation,
        "apply" => PolicyAction::ApplyMutation,
        "export" => PolicyAction::Export,
        "external-tool" => PolicyAction::RunExternalTool,
        "automation" => PolicyAction::UseAutomation,
        "plugin" => PolicyAction::LoadPlugin,
        "network" => PolicyAction::NetworkAccess,
        _ => PolicyAction::Read,
    }
}
