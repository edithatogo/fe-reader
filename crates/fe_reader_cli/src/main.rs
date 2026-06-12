//! Fe Reader CLI Wave 0 harness.

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use fe_reader_accessibility::AccessibilityAuditReport;
use fe_reader_core::{
    OperationIntent, OperationKind, OperationSource, PatchOperation, PatchPlan, RiskLevel,
    TransactionId, TransactionJournalEntry, TransactionJournalSidecar, TransactionPhase,
    list_recovery_required_transaction_sidecars, read_transaction_sidecar,
    write_transaction_sidecar,
};
use fe_reader_metadata::{MetadataOperation, MetadataScrubMode};
use fe_reader_prepress::PrepressReport;
use fe_reader_render::RenderBackend;
use fe_reader_search::{SearchQuery, build_search_index_records, search_spans};
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
    /// Diff two PDF metadata snapshots without mutating either document.
    MetadataDiff {
        /// Path to the before PDF file.
        before: String,
        /// Path to the after PDF file.
        after: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Plan a metadata scrub without applying document writes.
    MetadataScrub {
        /// Path to a PDF file.
        path: String,
        /// Scrub profile: preserve, clean-share, aggressive.
        #[arg(long, default_value = "clean-share")]
        profile: String,
        /// Require plan-only output. Apply is intentionally unavailable in Wave 2.
        #[arg(long)]
        plan_only: bool,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Generate a prepress report without mutating the document.
    Prepress {
        /// Path to a PDF file.
        path: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Generate an accessibility audit report without mutating the document.
    Accessibility {
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
    /// Inspect platform adapter contracts without calling native platform APIs.
    Platform {
        #[command(subcommand)]
        command: PlatformCommand,
    },
    /// Inspect PDF internals without mutating or executing active content.
    Lab {
        #[command(subcommand)]
        command: LabCommand,
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
    /// Render one deterministic tile through the render adapter boundary.
    RenderTile {
        /// Path to a PDF file.
        path: String,
        /// Zero-based page index.
        #[arg(long, default_value_t = 0)]
        page: u32,
        /// Tile left x coordinate in PDF points.
        #[arg(long, default_value_t = 0.0)]
        x: f32,
        /// Tile bottom y coordinate in PDF points.
        #[arg(long, default_value_t = 0.0)]
        y: f32,
        /// Tile width in PDF points.
        #[arg(long, default_value_t = 16.0)]
        width: f32,
        /// Tile height in PDF points.
        #[arg(long, default_value_t = 12.0)]
        height: f32,
        /// Scale multiplier.
        #[arg(long, default_value_t = 1.0)]
        scale: f32,
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
        /// Source surface: ui, cli, mcp, automation, com, applescript, dbus,
        /// android-intent, ios-app-intent, web, browser-extension, local-api, plugin.
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
    /// Inspect a persisted journal sidecar without mutating it.
    Inspect {
        /// Journal sidecar path.
        path: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// List journal sidecars in a directory that require recovery inspection.
    Recoveries {
        /// Directory containing journal sidecars.
        dir: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum PlatformCommand {
    /// Run the recent-document default-deny smoke contract for all target platforms.
    RecentSmoke {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Run the native automation read-only/default-deny smoke contract.
    AutomationSmoke {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum LabCommand {
    /// Inspect object and page graph summary.
    Inspect {
        /// Path to a PDF file.
        path: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Inspect content-stream and text-map diagnostics.
    TextMap {
        /// Path to a PDF file.
        path: String,
        /// Zero-based page index.
        #[arg(long, default_value_t = 0)]
        page: u32,
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
            let metadata = fe_reader_metadata::metadata_snapshot_path(&path)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                summary.document_id.clone(),
                "metadata",
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = PatchPlan::draft(
                &intent,
                format!("metadata {path}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "metadata": metadata.summary,
                    "snapshot": metadata,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!(
                    "title={}",
                    metadata
                        .summary
                        .document_info
                        .title
                        .as_deref()
                        .unwrap_or("")
                );
                println!(
                    "author={}",
                    metadata
                        .summary
                        .document_info
                        .author
                        .as_deref()
                        .unwrap_or("")
                );
                println!(
                    "xmp_metadata_present={}",
                    metadata.summary.xmp_metadata_present
                );
                println!("xmp_stream_count={}", metadata.summary.xmp_streams.len());
                println!(
                    "parser_error={}",
                    metadata.summary.parser_error.as_deref().unwrap_or("")
                );
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::MetadataDiff {
            before,
            after,
            json,
        } => {
            let before_summary = fe_reader_pdf_model::sniff_pdf_path(&before)?;
            let after_summary = fe_reader_pdf_model::sniff_pdf_path(&after)?;
            let diff = fe_reader_metadata::diff_metadata_paths(&before, &after)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                before_summary.document_id.clone(),
                "metadata_diff",
            )
            .with_document_fingerprint(before_summary.fingerprint.clone());
            let plan = PatchPlan::draft(
                &intent,
                format!("metadata diff {before} {after}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "before_summary": before_summary,
                    "after_summary": after_summary,
                    "diff": diff,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("changes={}", diff.changes.len());
                println!("before_sha256={}", before_summary.fingerprint.sha256_hex);
                println!("after_sha256={}", after_summary.fingerprint.sha256_hex);
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::MetadataScrub {
            path,
            profile,
            plan_only,
            json,
        } => {
            if !plan_only {
                bail!("metadata-scrub is plan-only in Wave 2; pass --plan-only");
            }
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let metadata = fe_reader_metadata::metadata_snapshot_path(&path)?;
            let scrub_mode = MetadataScrubMode::parse_profile(&profile)?;
            let intent = if matches!(scrub_mode, MetadataScrubMode::ForensicPreserve) {
                OperationIntent::read_only(
                    OperationSource::Cli,
                    summary.document_id.clone(),
                    "metadata_scrub_forensic_preserve",
                )
            } else {
                OperationIntent::mutation(
                    OperationSource::Cli,
                    summary.document_id.clone(),
                    OperationKind::PlanMutation,
                    "metadata_scrub",
                )
            }
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = fe_reader_metadata::plan_metadata_operations(
                &intent,
                &[MetadataOperation::Scrub { mode: scrub_mode }],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "metadata": metadata.summary,
                    "profile": profile,
                    "plan_only": true,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("profile={profile}");
                println!("write_mode={:?}", plan.write_mode);
                println!("approved_for_apply={}", plan.approved_for_apply);
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::Prepress { path, json } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let lab = fe_reader_pdf_model::inspect_lab_path(&path)?;
            let report = PrepressReport::from_pdf_path(&path)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                summary.document_id.clone(),
                "prepress",
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = PatchPlan::draft(
                &intent,
                format!("prepress {path}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "lab": lab,
                    "report": report,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("document_id={}", report.document_id);
                println!("output_intents={}", report.output_intents.len());
                println!("colour_findings={}", report.colour_findings.len());
                println!("font_findings={}", report.font_findings.len());
                println!("page_box_findings={}", report.page_box_findings.len());
                println!("plan_id={}", plan.plan_id.0);
            }
        }
        Command::Accessibility { path, json } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let mut report = AccessibilityAuditReport::from_pdf_path(&path)?;
            report.surface_id = summary.document_id.to_string();
            let extraction = fe_reader_pdf_model::extract_text_spans_path(&path)?;
            let intent = OperationIntent::read_only(
                OperationSource::Cli,
                summary.document_id.clone(),
                "accessibility",
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = PatchPlan::draft(
                &intent,
                format!("accessibility {path}"),
                vec![PatchOperation::Noop],
            );
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "extraction": extraction,
                    "report": report,
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("surface_id={}", report.surface_id);
                println!("targets={}", report.targets.len());
                println!("findings={}", report.findings.len());
                println!(
                    "wcag_target={}",
                    report.wcag_target.as_deref().unwrap_or("")
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
            JournalCommand::Inspect { path, json } => {
                let sidecar = read_transaction_sidecar(&path)?;
                let latest = sidecar.latest().cloned();
                let recovery_required = sidecar.recovery_required();
                if json {
                    let value = serde_json::json!({
                        "journal": sidecar,
                        "latest": latest,
                        "recovery_required": recovery_required,
                        "journal_path": path,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("journal_path={path}");
                    println!("recovery_required={recovery_required}");
                    if let Some(entry) = latest {
                        println!("transaction_id={}", entry.transaction_id);
                        println!("phase={:?}", entry.phase);
                        println!("sequence={}", entry.sequence);
                    }
                }
            }
            JournalCommand::Recoveries { dir, json } => {
                let sidecars = list_recovery_required_transaction_sidecars(&dir)?;
                let recovery_count = sidecars.len();
                if json {
                    let value = serde_json::json!({
                        "directory": dir,
                        "recovery_required_count": recovery_count,
                        "journals": sidecars,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("directory={dir}");
                    println!("recovery_required_count={recovery_count}");
                    for sidecar in sidecars {
                        if let Some(entry) = sidecar.latest() {
                            println!(
                                "transaction_id={} phase={:?} sequence={}",
                                entry.transaction_id, entry.phase, entry.sequence
                            );
                        }
                    }
                }
            }
        },
        Command::Platform { command } => match command {
            PlatformCommand::RecentSmoke { json } => {
                let receipts = fe_reader_platform::wave1_recent_document_smoke()?;
                if json {
                    let value = serde_json::json!({
                        "contract": "platform_recent_document",
                        "status": "pass",
                        "mutation_applied": receipts.iter().any(|receipt| receipt.applied),
                        "receipt_count": receipts.len(),
                        "receipts": receipts,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("contract=platform_recent_document");
                    println!("status=pass");
                    println!("receipt_count={}", receipts.len());
                    println!(
                        "mutation_applied={}",
                        receipts.iter().any(|receipt| receipt.applied)
                    );
                }
            }
            PlatformCommand::AutomationSmoke { json } => {
                let receipts = fe_reader_platform::wave5_native_automation_smoke()?;
                if json {
                    let value = serde_json::json!({
                        "contract": "native_automation",
                        "status": "pass",
                        "mutation_applied": receipts.iter().any(|receipt| receipt.applied),
                        "receipt_count": receipts.len(),
                        "receipts": receipts,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("contract=native_automation");
                    println!("status=pass");
                    println!("receipt_count={}", receipts.len());
                    println!(
                        "mutation_applied={}",
                        receipts.iter().any(|receipt| receipt.applied)
                    );
                }
            }
        },
        Command::Lab { command } => match command {
            LabCommand::Inspect { path, json } => {
                let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
                let lab = fe_reader_pdf_model::inspect_lab_path(&path)?;
                let intent = OperationIntent::new(
                    OperationSource::Cli,
                    summary.document_id.clone(),
                    OperationKind::Custom("lab_inspect".to_string()),
                    "lab_inspect",
                    RiskLevel::ReadOnly,
                )
                .with_document_fingerprint(summary.fingerprint.clone());
                let plan = fe_reader_core::PatchPlan::draft(
                    &intent,
                    format!("lab inspect {path}"),
                    vec![PatchOperation::Noop],
                );
                if json {
                    let value = serde_json::json!({
                        "intent": intent,
                        "plan": plan,
                        "summary": summary,
                        "lab": lab,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("mode={}", lab.mode);
                    println!("objects={}", lab.object_count);
                    println!("streams={}", lab.stream_count);
                    println!("pages={}", lab.pages.len());
                    println!("findings={}", lab.findings.len());
                    println!("parser_error={}", lab.error.as_deref().unwrap_or(""));
                    println!("plan_id={}", plan.plan_id.0);
                }
            }
            LabCommand::TextMap { path, page, json } => {
                let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
                let text_map = fe_reader_pdf_model::inspect_text_map_path(
                    &path,
                    fe_reader_pdf_model::PageIndex(page),
                )?;
                let intent = OperationIntent::new(
                    OperationSource::Cli,
                    summary.document_id.clone(),
                    OperationKind::Custom("lab_text_map".to_string()),
                    "lab_text_map",
                    RiskLevel::ReadOnly,
                )
                .with_document_fingerprint(summary.fingerprint.clone());
                let plan = PatchPlan::draft(
                    &intent,
                    format!("lab text-map {path} page={page}"),
                    vec![PatchOperation::Noop],
                );
                if json {
                    let value = serde_json::json!({
                        "intent": intent,
                        "plan": plan,
                        "summary": summary,
                        "text_map": text_map,
                    });
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("mode={}", text_map.mode);
                    println!("page_index={}", text_map.page_index.0);
                    println!("content_streams={}", text_map.content_streams.len());
                    println!("text_map_entries={}", text_map.text_map.len());
                    println!("findings={}", text_map.findings.len());
                    println!("parser_error={}", text_map.error.as_deref().unwrap_or(""));
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
            let index_records = build_search_index_records(
                &format!("sha256:{}", summary.fingerprint.sha256_hex),
                &summary.fingerprint.sha256_hex,
                &extraction.spans,
                None,
            );
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
                    "index_records": index_records,
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
        Command::RenderTile {
            path,
            page,
            x,
            y,
            width,
            height,
            scale,
            json,
        } => {
            let summary = fe_reader_pdf_model::sniff_pdf_path(&path)?;
            let intent = OperationIntent::new(
                OperationSource::Cli,
                summary.document_id.clone(),
                OperationKind::Render,
                "render_tile",
                RiskLevel::ReadOnly,
            )
            .with_document_fingerprint(summary.fingerprint.clone());
            let plan = fe_reader_core::PatchPlan::draft(
                &intent,
                format!("render tile {path} page={page}"),
                vec![PatchOperation::Noop],
            );
            let backend = fe_reader_render::NullRenderBackend;
            let request = fe_reader_render::RenderTileRequest {
                document_ref: format!("sha256:{}", summary.fingerprint.sha256_hex),
                page_index: fe_reader_pdf_model::PageIndex(page),
                tile_rect: fe_reader_pdf_model::PdfRect::new(x, y, width, height),
                scale,
                rotation_degrees: 0,
                color_mode: fe_reader_render::ColorMode::Normal,
                acceleration: fe_reader_render::AccelerationPreference::CpuOnly,
            };
            let tile = backend.render_tile(request.clone())?;
            if json {
                let value = serde_json::json!({
                    "intent": intent,
                    "plan": plan,
                    "summary": summary,
                    "render": {
                        "backend": backend.backend_name(),
                        "request": request,
                        "tile": {
                            "cache_key": tile.cache_key,
                            "width": tile.width,
                            "height": tile.height,
                            "pixel_format": tile.pixel_format,
                            "byte_len": tile.bytes.len(),
                            "all_zero": tile.bytes.iter().all(|byte| *byte == 0),
                        },
                        "status": "pass",
                    },
                });
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("backend={}", backend.backend_name());
                println!("page_index={page}");
                println!("width={}", tile.width);
                println!("height={}", tile.height);
                println!("pixel_format={:?}", tile.pixel_format);
                println!("byte_len={}", tile.bytes.len());
                println!("cache_key={}", tile.cache_key);
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
        "automation" | "com" | "windows-com" | "applescript" | "apple-script" | "dbus"
        | "d-bus" | "android-intent" | "android-intents" | "ios-app-intent" | "ios-app-intents" => {
            Ok(OperationSource::Automation)
        }
        "web" | "browser-extension" | "local-api" => Ok(OperationSource::Web),
        "plugin" => Ok(OperationSource::Plugin),
        _ => bail!("unknown operation source: {source}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_source_aliases_cover_integration_surfaces() {
        for source in [
            "com",
            "windows-com",
            "applescript",
            "dbus",
            "d-bus",
            "android-intent",
            "android-intents",
            "ios-app-intent",
            "ios-app-intents",
        ] {
            assert_eq!(
                parse_operation_source(source).unwrap(),
                OperationSource::Automation
            );
        }

        for source in ["web", "browser-extension", "local-api"] {
            assert_eq!(
                parse_operation_source(source).unwrap(),
                OperationSource::Web
            );
        }
    }
}
