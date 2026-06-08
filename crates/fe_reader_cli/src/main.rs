//! Fe Reader CLI Wave 0 harness.

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use fe_reader_core::{
    OperationIntent, OperationKind, OperationSource, PatchOperation, TransactionId,
    TransactionJournalEntry, TransactionJournalSidecar, TransactionPhase,
    write_transaction_sidecar,
};
use fe_reader_search::{SearchQuery, search_spans};
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
    /// Inspect PDF metadata without mutating it.
    Metadata {
        /// Path to a PDF file.
        path: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Persist or inspect transaction journals.
    Journal {
        #[command(subcommand)]
        command: JournalCommand,
    },
    /// Search extracted text with deterministic literal matching.
    Search {
        /// Path to a PDF file.
        path: String,
        /// Literal query text.
        query: String,
        /// Match case exactly.
        #[arg(long)]
        case_sensitive: bool,
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
        /// Source surface: ui, cli, mcp, automation, web, plugin.
        #[arg(long, default_value = "cli")]
        source: String,
    },
}

#[derive(Debug, Subcommand)]
enum JournalCommand {
    /// Persist a no-op plan journal sidecar for a PDF.
    Plan {
        /// Path to a PDF file.
        path: String,
        /// Output journal sidecar path.
        #[arg(long)]
        out: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
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
        Command::Metadata { path, json } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let metadata = fe_reader_metadata::inspect_metadata_path(&path)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                summary.document_id.clone(),
                "metadata",
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = fe_reader_core::PatchPlan::draft(
                &intent,
                format!("metadata {path}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "metadata": metadata,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!(
                    "title={}",
                    metadata.document_info.title.as_deref().unwrap_or("")
                );
                println!(
                    "author={}",
                    metadata.document_info.author.as_deref().unwrap_or("")
                );
                println!("xmp_metadata_present={}", metadata.xmp_metadata_present);
                println!(
                    "parser_error={}",
                    metadata.parser_error.as_deref().unwrap_or("")
                );
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::Journal { command } => match command {
            JournalCommand::Plan { path, out, json } => {
                let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
                let intent = OperationIntent::new(
                    OperationSource::Cli,
                    summary.document_id.clone(),
                    OperationKind::PlanMutation,
                    "journal_plan",
                    fe_reader_core::RiskLevel::DocumentMutation,
                )
                .with_document_fingerprint(summary.fingerprint.clone());
                let plan = fe_reader_core::PatchPlan::draft(
                    &intent,
                    format!("journal plan {path}"),
                    vec![PatchOperation::Noop],
                );
                let transaction_id = TransactionId::new();
                let mut sidecar = TransactionJournalSidecar::new();
                sidecar.append(TransactionJournalEntry::intent_received(
                    &transaction_id,
                    &intent,
                    "intent received; no document bytes modified",
                ))?;
                sidecar.append(TransactionJournalEntry::for_plan(
                    &transaction_id,
                    &intent,
                    &plan,
                    TransactionPhase::PlanGenerated,
                    1,
                    "patch plan generated; no document bytes modified",
                ))?;
                write_transaction_sidecar(&out, &sidecar)?;
                if json {
                    let value = serde_json::json!({
                        "intent": intent,
                        "plan": plan,
                        "summary": summary,
                        "journal": sidecar,
                        "journal_path": out,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("journal_path={out}");
                    println!("transaction_id={}", transaction_id.0);
                    println!("plan_id={}", plan.plan_id.0);
                }
            }
        },
        Command::Search {
            path,
            query,
            case_sensitive,
            json,
        } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let extraction = fe_reader_pdf_model::extract_text_spans_path(&path)?;
            let text_summary = fe_reader_text::summarize_extracted_text(
                &extraction
                    .spans
                    .iter()
                    .map(|span| span.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            let search_query = SearchQuery {
                text: query,
                case_sensitive,
            };
            let hits = search_spans(&extraction.spans, &search_query);
            let intent = OperationIntent::new(
                OperationSource::Cli,
                summary.document_id.clone(),
                OperationKind::Search,
                "search",
                fe_reader_core::RiskLevel::ReadOnly,
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = fe_reader_core::PatchPlan::draft(
                &intent,
                format!("search {path}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "text": {
                        "extraction": extraction,
                        "summary": text_summary,
                    },
                    "query": search_query,
                    "hits": hits,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("hits={}", hits.len());
                println!("precise_geometry={}", extraction.precise_geometry);
                println!("parser_error={}", extraction.error.as_deref().unwrap_or(""));
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::ValidateSchemas => {
            println!("schema validation is delegated to scripts/validate_schemas.py in Wave 0");
        }
        Command::Policy { action, source } => {
            let action = parse_policy_action(&action)?;
            let source = parse_operation_source(&source)?;
            let decision = evaluate_policy(
                &SecurityPolicy::default(),
                source,
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

fn parse_policy_action(action: &str) -> Result<PolicyAction> {
    match action {
        "read" => Ok(PolicyAction::Read),
        "plan" => Ok(PolicyAction::PlanMutation),
        "apply" => Ok(PolicyAction::ApplyMutation),
        "export" => Ok(PolicyAction::Export),
        "external-tool" => Ok(PolicyAction::RunExternalTool),
        "automation" => Ok(PolicyAction::UseAutomation),
        "plugin" => Ok(PolicyAction::LoadPlugin),
        "network" => Ok(PolicyAction::NetworkAccess),
        _ => bail!("unknown policy action: {action}"),
    }
}

fn parse_operation_source(source: &str) -> Result<OperationSource> {
    match source {
        "ui" => Ok(OperationSource::Ui),
        "cli" => Ok(OperationSource::Cli),
        "mcp" => Ok(OperationSource::Mcp),
        "automation" => Ok(OperationSource::Automation),
        "web" => Ok(OperationSource::Web),
        "plugin" => Ok(OperationSource::Plugin),
        _ => bail!("unknown operation source: {source}"),
    }
}
