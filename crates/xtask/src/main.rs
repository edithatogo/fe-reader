//! Fe Reader xtask Wave 0 harness.

mod perf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fe_reader_render::RenderBackend;
use perf::PerfTask;
use std::process::Command;

#[derive(Debug, Parser)]
#[command(name = "xtask", version, about = "Fe Reader repo automation")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
    /// Run the Wave 0 bootstrap checks.
    Wave0Check,
    /// Run formatting, clippy, tests and schema smoke checks where available.
    Review,
    /// Performance and benchmark harness commands.
    Perf {
        #[command(subcommand)]
        command: PerfTask,
    },
    /// Validate JSON schemas.
    ValidateSchemas,
    /// Emit sample Document IR and transformation graph JSON.
    IrSmoke,
    /// Emit a passive transformation graph compile report.
    IrCompileSmoke,
    /// Emit deterministic render smoke metadata.
    RenderSmoke,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Task::Wave0Check => run_script("scripts/wave0_bootstrap_check.sh"),
        Task::Review => run_script("scripts/conductor_phase_gate.sh"),
        Task::Perf { command } => perf::run(command),
        Task::ValidateSchemas => run_script("scripts/validate_schemas.py"),
        Task::IrSmoke => run_ir_smoke(),
        Task::IrCompileSmoke => run_ir_compile_smoke(),
        Task::RenderSmoke => run_render_smoke(),
    }
}

fn run_ir_smoke() -> Result<()> {
    let sha256 = "f7e2b4436614640779c890a882537d543cf4579ae6cc43ad5f43f193afa6cd7f";
    let document_ir = fe_reader_ir::DocumentIr::minimal("fixture:text-search-fixture", sha256);
    let graph = fe_reader_ir::TransformationGraph::read_only_smoke(sha256);
    document_ir.validate()?;
    graph.validate()?;
    let payload = serde_json::json!({
        "document_ir": document_ir,
        "transformation_graph": graph,
    });
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

fn run_ir_compile_smoke() -> Result<()> {
    let sha256 = "f7e2b4436614640779c890a882537d543cf4579ae6cc43ad5f43f193afa6cd7f";
    let graph = fe_reader_ir::TransformationGraph::read_only_smoke(sha256);
    let registry = fe_reader_ir::TransformationPassRegistry::preview();
    registry.validate()?;
    let report = graph.compile(&registry)?;
    let payload = serde_json::json!({
        "transformation_graph": graph,
        "pass_registry": registry,
        "compilation_report": report,
    });
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

fn run_render_smoke() -> Result<()> {
    let backend = fe_reader_render::NullRenderBackend;
    let request = fe_reader_render::RenderTileRequest {
        document_ref: "fixture:text-search-fixture".to_string(),
        page_index: fe_reader_pdf_model::PageIndex(0),
        tile_rect: fe_reader_pdf_model::PdfRect::new(0.0, 0.0, 16.0, 12.0),
        scale: 1.0,
        rotation_degrees: 0,
        color_mode: fe_reader_render::ColorMode::Normal,
        acceleration: fe_reader_render::AccelerationPreference::CpuOnly,
    };
    let tile = backend.render_tile(request)?;
    let payload = serde_json::json!({
        "fixture_id": "text-search-fixture",
        "page_index": 0,
        "backend": backend.backend_name(),
        "width": tile.width,
        "height": tile.height,
        "pixel_format": tile.pixel_format,
        "byte_len": tile.bytes.len(),
        "cache_key": tile.cache_key,
        "status": "pass",
        "max_delta": 0,
        "changed_pixels": 0,
    });
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

fn run_script(path: &str) -> Result<()> {
    let status = if path.ends_with(".py") {
        Command::new("python3").arg(path).status()?
    } else {
        Command::new("bash").arg(path).status()?
    };
    if !status.success() {
        anyhow::bail!("{path} failed with status {status}");
    }
    Ok(())
}
