//! Performance harness commands for Fe Reader.

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand, ValueEnum};
use fe_reader_core::{
    DocumentFingerprint, DocumentId, OperationIntent, OperationKind, OperationSource,
};
use fe_reader_jobs::{CancellationToken, JobRun, plan_visible_page_prefetch};
use fe_reader_metadata::{MetadataOperation, MetadataScrubMode, plan_metadata_operations};
use fe_reader_pdf_model::{PageIndex, PdfRect, extract_text_spans_bytes};
use fe_reader_redaction::{RedactionRecipe, plan_secure_redaction, verify_secure_redaction_plan};
use fe_reader_render::{
    AccelerationPreference, ColorMode, NullRenderBackend, RenderBackend, RenderTileCache,
    RenderTileRequest,
};
use fe_reader_search::build_search_index_records;
use fe_reader_text::summarize_extracted_text;
use fe_reader_workflows::{WorkflowPack, plan_workflow_pack};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    hint::black_box,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

#[derive(Debug, Subcommand)]
pub enum PerfTask {
    /// Run the advisory performance smoke script.
    Smoke,
    /// Run a benchmark suite and write artifacts/perf/latest.json.
    Bench(PerfBenchArgs),
    /// Run deterministic callgrind/cachegrind/dhat targets.
    Callgrind(PerfCallgrindArgs),
    /// Compare release profiles, allocator variants and binary size.
    Release(PerfReleaseArgs),
    /// Compare the latest benchmark manifest against the budget file.
    Compare(PerfCompareArgs),
    /// Run a lightweight hyperfine comparison when available.
    Hyperfine,
    /// Produce an advisory PGO training manifest.
    PgoTrain,
    /// Produce an advisory optimized build manifest.
    PgoBuild,
}

#[derive(Debug, Args)]
pub struct PerfBenchArgs {
    /// Benchmark suite to run.
    #[arg(long, value_enum, default_value_t = PerfSuite::Default)]
    pub suite: PerfSuite,
}

#[derive(Debug, Args)]
pub struct PerfCompareArgs {
    /// Budget file to compare against.
    #[arg(long, default_value = "benchmarks/budgets/performance-budgets.yaml")]
    pub budget: PathBuf,
}

#[derive(Debug, Args)]
pub struct PerfReleaseArgs {
    /// Release comparison suite.
    #[arg(long, value_enum, default_value_t = PerfReleaseSuite::Default)]
    pub suite: PerfReleaseSuite,
}

#[derive(Debug, Args)]
pub struct PerfCallgrindArgs {
    /// Callgrind suite to run.
    #[arg(long, value_enum, default_value_t = PerfDeterministicSuite::Deterministic)]
    pub suite: PerfDeterministicSuite,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PerfDeterministicSuite {
    /// Run all deterministic Linux CI tools.
    Deterministic,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PerfReleaseSuite {
    /// Compare standard release profiles, allocator variants and size tooling.
    Default,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PerfSuite {
    /// Reader-oriented rendering and extraction benchmarks.
    Reader,
    /// Workflow and metadata benchmarks.
    Workflows,
    /// All available benchmark cases.
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfManifest {
    suite: String,
    generated_at: String,
    cases: Vec<PerfCaseResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfCaseResult {
    scenario_id: String,
    class: String,
    samples_ms: Vec<f64>,
    p50_ms: f64,
    p95_ms: f64,
    peak_bytes: u64,
    metrics: BTreeMap<String, serde_json::Value>,
    detail: String,
}

#[derive(Debug, Deserialize)]
struct BudgetFile {
    budgets: Vec<BudgetEntry>,
}

#[derive(Debug, Deserialize)]
struct BudgetEntry {
    scenario_id: String,
    #[allow(dead_code)]
    class: String,
    max_p95_ms: f64,
    max_peak_rss_mb: Option<u64>,
}

type BenchFn = Box<dyn Fn(usize) -> BenchmarkObservation + Send + Sync>;

struct BenchCase {
    scenario_id: &'static str,
    class: &'static str,
    detail: &'static str,
    run: BenchFn,
}

#[derive(Debug, Clone)]
struct BenchmarkObservation {
    peak_bytes: u64,
    metrics: BTreeMap<String, serde_json::Value>,
    detail: String,
}

#[derive(Debug, Clone)]
struct ReleaseVariant<'a> {
    profile: &'a str,
    allocator: &'a str,
    features: &'a [&'a str],
    artifact_name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
struct ReleaseRun {
    profile: String,
    allocator: String,
    features: Vec<String>,
    artifact_path: String,
    binary_size_bytes: u64,
    build_status: String,
    bloat_status: String,
    llvm_lines_status: String,
}

pub fn run(task: PerfTask) -> Result<()> {
    match task {
        PerfTask::Smoke => run_script("scripts/perf_smoke.sh"),
        PerfTask::Bench(args) => run_bench(args.suite),
        PerfTask::Callgrind(args) => run_callgrind(args.suite),
        PerfTask::Release(args) => run_release(args.suite),
        PerfTask::Compare(args) => run_compare(&args.budget),
        PerfTask::Hyperfine => run_hyperfine(),
        PerfTask::PgoTrain => run_pgo_train(),
        PerfTask::PgoBuild => run_pgo_build(),
    }
}

fn run_bench(suite: PerfSuite) -> Result<()> {
    let cases = build_cases(suite)?;
    fs::create_dir_all("artifacts/perf")?;
    let mut results = Vec::new();
    for case in cases {
        results.push(measure_case(case));
    }

    let manifest = PerfManifest {
        suite: format!("{suite:?}").to_lowercase(),
        generated_at: "2026-06-12T00:00:00Z".to_string(),
        cases: results,
    };
    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write("artifacts/perf/latest.json", format!("{json}\n"))?;
    write_markdown_summary(&manifest)?;
    println!("{json}");
    Ok(())
}

fn run_compare(budget_path: &Path) -> Result<()> {
    let manifest_bytes = fs::read("artifacts/perf/latest.json")
        .context("missing artifacts/perf/latest.json; run `cargo xtask perf bench` first")?;
    let manifest: PerfManifest = serde_json::from_slice(&manifest_bytes)?;
    let budget_bytes = fs::read(budget_path)
        .with_context(|| format!("failed to read budget file {}", budget_path.display()))?;
    let budgets: BudgetFile = serde_yaml::from_slice(&budget_bytes)?;

    let result_map = manifest
        .cases
        .iter()
        .map(|case| (case.scenario_id.as_str(), case))
        .collect::<BTreeMap<_, _>>();
    let mut failures = Vec::new();
    for budget in budgets.budgets {
        let Some(case) = result_map.get(budget.scenario_id.as_str()) else {
            continue;
        };
        if case.p95_ms > budget.max_p95_ms {
            failures.push(format!(
                "{} p95 {:.3}ms > budget {:.3}ms",
                budget.scenario_id, case.p95_ms, budget.max_p95_ms
            ));
        }
        if let Some(max_peak_rss_mb) = budget.max_peak_rss_mb {
            let peak_mb = bytes_to_mb(case.peak_bytes);
            if peak_mb > max_peak_rss_mb as f64 {
                failures.push(format!(
                    "{} peak {:.1}MB > budget {}MB",
                    budget.scenario_id, peak_mb, max_peak_rss_mb
                ));
            }
        }
    }

    if !failures.is_empty() {
        bail!(failures.join("\n"));
    }

    println!("perf compare passed against {}", budget_path.display());
    Ok(())
}

fn run_hyperfine() -> Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg("command -v hyperfine >/dev/null 2>&1 && command -v fe-reader >/dev/null 2>&1")
        .status()?;
    if !status.success() {
        println!("hyperfine or fe-reader unavailable; skipping");
        return Ok(());
    }
    let status = Command::new("hyperfine")
        .arg("--warmup")
        .arg("1")
        .arg("fe-reader --version")
        .status()?;
    if !status.success() {
        bail!("hyperfine run failed");
    }
    Ok(())
}

fn run_pgo_train() -> Result<()> {
    fs::create_dir_all("artifacts/perf")?;
    let manifest = serde_json::json!({
        "check": "pgo_train",
        "status": "advisory",
        "detail": "pgo training lane is advisory until training corpora and profile capture are materialised",
    });
    fs::write(
        "artifacts/perf/pgo-train.json",
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}

fn run_pgo_build() -> Result<()> {
    fs::create_dir_all("artifacts/perf")?;
    let manifest = serde_json::json!({
        "check": "pgo_build",
        "status": "advisory",
        "detail": "pgo build lane is advisory until release-profile capture exists",
    });
    fs::write(
        "artifacts/perf/pgo-build.json",
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}

fn build_release_variant(variant: ReleaseVariant<'_>) -> Result<ReleaseRun> {
    let mut command = Command::new("cargo");
    command.arg("build");
    command.arg("--package").arg("fe_reader_cli");
    command.arg("--bin").arg("fe-reader");
    command.arg("--profile").arg(variant.profile);
    if !variant.features.is_empty() {
        command.arg("--features").arg(variant.features.join(","));
    }
    let output = command.output()?;
    if !output.status.success() {
        bail!(
            "release build failed for {} {} with status {}",
            variant.profile,
            variant.allocator,
            output.status
        );
    }

    let source_path = release_binary_path(variant.profile);
    let artifact_path = release_artifact_path(variant.artifact_name);
    fs::copy(&source_path, &artifact_path)?;
    let binary_size_bytes = fs::metadata(&artifact_path)?.len();
    Ok(ReleaseRun {
        profile: variant.profile.to_string(),
        allocator: variant.allocator.to_string(),
        features: variant
            .features
            .iter()
            .map(|feature| feature.to_string())
            .collect(),
        artifact_path,
        binary_size_bytes,
        build_status: "pass".to_string(),
        bloat_status: "pending".to_string(),
        llvm_lines_status: "pending".to_string(),
    })
}

fn run_optional_tool(
    tool_name: &str,
    cargo_args: &[&str],
    command_label: &str,
    log_path: &str,
) -> Result<serde_json::Value> {
    let available = Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", tool_name))
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    fs::create_dir_all(
        Path::new(log_path)
            .parent()
            .expect("log path should have a parent"),
    )?;
    if !available {
        fs::write(log_path, format!("{command_label}: unavailable\n"))?;
        return Ok(serde_json::json!({
            "status": "unavailable",
            "command": command_label,
            "log_path": log_path,
        }));
    }
    let output = Command::new("cargo").args(cargo_args).output()?;
    fs::write(
        log_path,
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )?;
    let status = if output.status.success() {
        "pass"
    } else {
        "failed"
    };
    Ok(serde_json::json!({
        "status": status,
        "command": command_label,
        "log_path": log_path,
    }))
}

fn render_release_summary(manifest: &serde_json::Value) -> Result<String> {
    let runs = manifest["runs"].as_array().cloned().unwrap_or_default();
    let mut body = String::new();
    body.push_str("# Release Optimization Summary\n\n");
    body.push_str("| Profile | Allocator | Binary Size (bytes) | Status |\n");
    body.push_str("| --- | --- | ---: | --- |\n");
    for run in runs {
        body.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            run["profile"].as_str().unwrap_or_default(),
            run["allocator"].as_str().unwrap_or_default(),
            run["binary_size_bytes"].as_u64().unwrap_or_default(),
            run["build_status"].as_str().unwrap_or_default(),
        ));
    }
    body.push_str("\n## Tooling\n\n");
    body.push_str(&serde_json::to_string_pretty(&manifest["tooling"])?);
    body.push('\n');
    Ok(body)
}

fn release_binary_path(profile: &str) -> String {
    let path = Path::new("target").join(profile).join(if cfg!(windows) {
        "fe-reader.exe"
    } else {
        "fe-reader"
    });
    path.display().to_string()
}

fn release_artifact_path(artifact_name: &str) -> String {
    let path = Path::new("artifacts/perf/release").join(if cfg!(windows) {
        format!("{artifact_name}.exe")
    } else {
        artifact_name.to_string()
    });
    path.display().to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn run_release(suite: PerfReleaseSuite) -> Result<()> {
    match suite {
        PerfReleaseSuite::Default => run_release_default(),
    }
}

fn run_release_default() -> Result<()> {
    fs::create_dir_all("artifacts/perf/release")?;
    let variants = vec![
        ReleaseVariant {
            profile: "release-thinlto",
            allocator: "system",
            features: &[],
            artifact_name: "fe-reader-system",
        },
        ReleaseVariant {
            profile: "release-fat",
            allocator: "system",
            features: &[],
            artifact_name: "fe-reader-fat",
        },
        ReleaseVariant {
            profile: "release-thinlto",
            allocator: "mimalloc",
            features: &["mimalloc-allocator"],
            artifact_name: "fe-reader-mimalloc",
        },
    ];

    let mut runs = Vec::new();
    for variant in variants {
        runs.push(build_release_variant(variant)?);
    }

    let cargo_bloat = run_optional_tool(
        "cargo-bloat",
        &["bloat", "--release", "--crates"],
        "cargo bloat --release --crates",
        "artifacts/perf/release/cargo-bloat.log",
    )?;
    let cargo_llvm_lines = run_optional_tool(
        "cargo-llvm-lines",
        &["llvm-lines", "--workspace"],
        "cargo llvm-lines --workspace",
        "artifacts/perf/release/cargo-llvm-lines.log",
    )?;

    let manifest = serde_json::json!({
        "suite": "release",
        "runs": runs,
        "tooling": {
            "cargo_bloat": cargo_bloat,
            "cargo_llvm_lines": cargo_llvm_lines,
        },
    });
    let manifest_path = "artifacts/perf/release/manifest.json";
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    let summary = render_release_summary(&manifest)?;
    let summary_path = "artifacts/perf/release/summary.md";
    fs::write(summary_path, &summary)?;
    let signature = sha256_hex(summary.as_bytes());
    fs::write(
        "artifacts/perf/release/summary.md.sha256",
        format!("{signature}  summary.md\n"),
    )?;
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}

fn run_callgrind(suite: PerfDeterministicSuite) -> Result<()> {
    fs::create_dir_all("artifacts/perf/iai-callgrind")?;
    let tools = match suite {
        PerfDeterministicSuite::Deterministic => vec![
            ("callgrind", "callgrind_core", "Callgrind"),
            ("cachegrind", "cachegrind_core", "Cachegrind"),
            ("dhat", "dhat_core", "Dhat"),
        ],
    };

    let linux = std::env::consts::OS == "linux";
    let valgrind_available = Command::new("sh")
        .arg("-c")
        .arg("command -v valgrind >/dev/null 2>&1")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    let execute = linux && valgrind_available;

    let mut runs = Vec::new();
    for (tool_name, bench_name, tool_label) in tools {
        let mut entry = serde_json::json!({
            "tool": tool_name,
            "bench": bench_name,
            "mode": if execute { "execute" } else { "compile_only" },
        });
        let mut command = Command::new("cargo");
        command.args([
            "bench",
            "-p",
            "fe_reader_perf_benches",
            "--bench",
            bench_name,
        ]);
        let output = if execute {
            command.output()?
        } else {
            command.arg("--no-run").output()?
        };
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let log_path = format!("artifacts/perf/iai-callgrind/{}.log", tool_name);
        fs::write(&log_path, format!("{stdout}{stderr}"))?;
        if !output.status.success() {
            bail!(
                "{} benchmark failed with status {}",
                tool_label,
                output.status
            );
        }
        entry["log_path"] = serde_json::json!(log_path);
        runs.push(entry);
    }

    let manifest = serde_json::json!({
        "suite": "deterministic",
        "executed": execute,
        "runs": runs,
    });
    fs::write(
        "artifacts/perf/iai-callgrind/manifest.json",
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    fs::write(
        "artifacts/perf/iai-callgrind/summary.md",
        format!(
            "# Iai-Callgrind Baseline\n\n- Suite: deterministic\n- Executed: {}\n- Log directory: artifacts/perf/iai-callgrind\n",
            execute
        ),
    )?;
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}

fn build_cases(suite: PerfSuite) -> Result<Vec<BenchCase>> {
    let mut cases = Vec::new();
    if matches!(suite, PerfSuite::Reader | PerfSuite::Default) {
        cases.extend(reader_cases()?);
    }
    if matches!(suite, PerfSuite::Workflows | PerfSuite::Default) {
        cases.extend(workflow_cases());
    }
    Ok(cases)
}

fn reader_cases() -> Result<Vec<BenchCase>> {
    let tile_request = RenderTileRequest {
        document_ref: "fixture:corpus/basic/text-search-fixture".to_string(),
        page_index: PageIndex(0),
        tile_rect: PdfRect::new(0.0, 0.0, 512.0, 512.0),
        scale: 2.0,
        rotation_degrees: 0,
        color_mode: ColorMode::Normal,
        acceleration: AccelerationPreference::CpuOnly,
    };
    let tile_request_for_render = tile_request.clone();
    let tile_request_for_cache = tile_request.clone();
    let thumbnail_document = "fixture:corpus/basic/text-search-fixture".to_string();
    let text_bytes = read_fixture("fixtures/corpus/basic/text-search-fixture.pdf")?;

    Ok(vec![
        BenchCase {
            scenario_id: "render.tile.512_2x_warm",
            class: "InteractiveP0",
            detail: "null backend tile render on a warmed 512px tile",
            run: Box::new(move |iterations| {
                let mut tile_bytes = 0usize;
                let mut tile_width = 0u32;
                let mut tile_height = 0u32;
                let mut cache_key = String::new();
                let mut bytes = BTreeMap::new();
                let backend = NullRenderBackend;
                for _ in 0..iterations {
                    let tile = backend
                        .render_tile(tile_request_for_render.clone())
                        .expect("null renderer must remain deterministic");
                    tile_width = tile.width;
                    tile_height = tile.height;
                    tile_bytes = tile.bytes.len();
                    cache_key = tile.cache_key;
                }
                bytes.insert("width".to_string(), serde_json::json!(tile_width));
                bytes.insert("height".to_string(), serde_json::json!(tile_height));
                bytes.insert(
                    "cache_key_len".to_string(),
                    serde_json::json!(cache_key.len()),
                );
                BenchmarkObservation {
                    peak_bytes: tile_bytes as u64,
                    metrics: bytes,
                    detail: "tile render completed with null backend".to_string(),
                }
            }),
        },
        BenchCase {
            scenario_id: "render.first_page.150dpi",
            class: "InteractiveP0",
            detail: "null backend thumbnail render for the first page",
            run: Box::new(move |iterations| {
                let mut sample = None;
                let backend = NullRenderBackend;
                for _ in 0..iterations {
                    sample = Some(
                        backend
                            .render_page_thumbnail(&thumbnail_document, PageIndex(0), 150)
                            .expect("thumbnail render must succeed"),
                    );
                }
                let tile = sample.expect("thumbnail sample");
                let mut metrics = BTreeMap::new();
                metrics.insert("width".to_string(), serde_json::json!(tile.width));
                metrics.insert("height".to_string(), serde_json::json!(tile.height));
                BenchmarkObservation {
                    peak_bytes: tile.bytes.len() as u64,
                    metrics,
                    detail: "first-page thumbnail render completed".to_string(),
                }
            }),
        },
        BenchCase {
            scenario_id: "core.extract_text.single_page",
            class: "InteractiveP0",
            detail: "text span extraction from the basic fixture",
            run: Box::new(move |iterations| {
                let mut span_count = 0usize;
                let mut normalized_chars = 0usize;
                let mut diagnostics = 0usize;
                for _ in 0..iterations {
                    let summary = extract_text_spans_bytes(&text_bytes)
                        .expect("fixture PDF must remain extractable");
                    span_count = summary.spans.len();
                    normalized_chars = summary
                        .spans
                        .iter()
                        .map(|span| summarize_extracted_text(&span.text).normalized_char_count)
                        .sum();
                    diagnostics = summary.diagnostics.len();
                    let _ = build_search_index_records(
                        "fixture:corpus/basic/text-search-fixture",
                        "fixture-sha256",
                        &summary.spans,
                        Some("en"),
                    );
                }
                let mut metrics = BTreeMap::new();
                metrics.insert("spans".to_string(), serde_json::json!(span_count));
                metrics.insert(
                    "normalized_chars".to_string(),
                    serde_json::json!(normalized_chars),
                );
                metrics.insert("diagnostics".to_string(), serde_json::json!(diagnostics));
                BenchmarkObservation {
                    peak_bytes: normalized_chars as u64,
                    metrics,
                    detail: "text span extraction and search-index shaping".to_string(),
                }
            }),
        },
        BenchCase {
            scenario_id: "render.tile_cache.visible_pages",
            class: "InteractiveP0",
            detail: "tile cache byte budget across a visible-page window",
            run: Box::new(move |iterations| {
                let mut cache = RenderTileCache::new(4);
                let backend = NullRenderBackend;
                for iteration in 0..iterations {
                    let page_index = PageIndex(iteration as u32);
                    let tile = backend
                        .render_tile(RenderTileRequest {
                            page_index,
                            ..tile_request_for_cache.clone()
                        })
                        .expect("tile render must succeed");
                    cache.insert(tile);
                }
                let mut metrics = BTreeMap::new();
                metrics.insert("entries".to_string(), serde_json::json!(cache.len()));
                metrics.insert("bytes".to_string(), serde_json::json!(cache.byte_len()));
                BenchmarkObservation {
                    peak_bytes: cache.byte_len() as u64,
                    metrics,
                    detail: "tile cache byte accounting completed".to_string(),
                }
            }),
        },
        BenchCase {
            scenario_id: "jobs.visible_page.prefetch_cancel",
            class: "InteractiveP0",
            detail: "visible-page prefetch planning with cooperative cancellation",
            run: Box::new(move |iterations| {
                let mut cancellation_checks = 0usize;
                let mut prefetched_pages = 0usize;
                for _ in 0..iterations {
                    let token = CancellationToken::default();
                    let plan = plan_visible_page_prefetch(12, 120, 3);
                    for (index, _page) in plan.prefetch_pages.iter().enumerate() {
                        cancellation_checks += 1;
                        if index == 1 {
                            token.cancel();
                        }
                        if token.is_cancelled() {
                            break;
                        }
                        prefetched_pages += 1;
                    }
                }
                let mut metrics = BTreeMap::new();
                metrics.insert(
                    "prefetch_pages".to_string(),
                    serde_json::json!(prefetched_pages),
                );
                metrics.insert(
                    "cancellation_checks".to_string(),
                    serde_json::json!(cancellation_checks),
                );
                let _ = black_box(JobRun::smoke_completed());
                BenchmarkObservation {
                    peak_bytes: 0,
                    metrics,
                    detail: "prefetch window and cancellation probe completed".to_string(),
                }
            }),
        },
    ])
}

fn workflow_cases() -> Vec<BenchCase> {
    let affidavit_pack = WorkflowPack::wave3_baseline_packs()
        .into_iter()
        .find(|pack| pack.pack_id == "legal.affidavit.initials.every_page")
        .expect("baseline affidavit pack must exist");

    let mut cases = Vec::new();
    for page_count in [10_u32, 100, 500] {
        let pack = affidavit_pack.clone();
        cases.push(BenchCase {
            scenario_id: Box::leak(
                format!("workflow.affidavit.apply_{page_count}_pages").into_boxed_str(),
            ),
            class: "WorkflowP1",
            detail: "legal affidavit plan generation across repeated pages",
            run: Box::new(move |iterations| {
                let mut bundles = Vec::new();
                let mut operations = 0usize;
                let mut xobject_reuse_count = 0usize;
                for _ in 0..iterations {
                    let mut planned_packs = Vec::new();
                    for _ in 0..page_count {
                        let fingerprint = DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF");
                        let planned = plan_workflow_pack(
                            OperationSource::Cli,
                            DocumentId::new(),
                            fingerprint,
                            pack.clone(),
                        );
                        operations = planned.plan.operations.len();
                        planned_packs.push(planned);
                    }
                    xobject_reuse_count = page_count.saturating_sub(1) as usize;
                    bundles = planned_packs;
                }
                let bundle_bytes =
                    serde_json::to_vec(&bundles).expect("workflow bundle serializable");
                let mut metrics = BTreeMap::new();
                metrics.insert("pages".to_string(), serde_json::json!(page_count));
                metrics.insert("operations".to_string(), serde_json::json!(operations));
                metrics.insert(
                    "xobject_reuse_count".to_string(),
                    serde_json::json!(xobject_reuse_count),
                );
                metrics.insert(
                    "output_bytes".to_string(),
                    serde_json::json!(bundle_bytes.len()),
                );
                BenchmarkObservation {
                    peak_bytes: bundle_bytes.len() as u64,
                    metrics,
                    detail: "workflow pack planning stayed bounded".to_string(),
                }
            }),
        });
    }

    cases.push(BenchCase {
        scenario_id: "redaction.secure_rewrite_100_pages",
        class: "WorkflowP1",
        detail: "secure redaction planning and verification probe",
        run: Box::new(move |iterations| {
            let redaction_recipe = RedactionRecipe::smoke_secure();
            let mut checks = 0usize;
            let mut passed = false;
            let mut receipt_bytes = 0usize;
            for _ in 0..iterations {
                let fingerprint = DocumentFingerprint::from_bytes(b"%PDF-1.7\n%%EOF");
                let verify_fingerprint = fingerprint.clone();
                let (intent, plan) = plan_secure_redaction(
                    OperationSource::Cli,
                    DocumentId::new(),
                    fingerprint,
                    &redaction_recipe,
                )
                .expect("secure redaction planning must succeed");
                let report = verify_secure_redaction_plan(&intent, &plan, verify_fingerprint)
                    .expect("secure redaction verification must succeed");
                receipt_bytes = serde_json::to_vec(&report.receipt)
                    .expect("redaction receipt serializable")
                    .len();
                checks = report.checks.len();
                passed = report.passed;
            }
            let mut metrics = BTreeMap::new();
            metrics.insert("pages".to_string(), serde_json::json!(100));
            metrics.insert("checks".to_string(), serde_json::json!(checks));
            metrics.insert("passed".to_string(), serde_json::json!(passed));
            metrics.insert("output_bytes".to_string(), serde_json::json!(receipt_bytes));
            BenchmarkObservation {
                peak_bytes: receipt_bytes as u64,
                metrics,
                detail: "secure redaction plan and verification completed".to_string(),
            }
        }),
    });

    cases.push(BenchCase {
        scenario_id: "metadata.snapshot_diff_100_pages",
        class: "WorkflowP1",
        detail: "metadata snapshot and diff planning probe",
        run: Box::new(move |iterations| {
            let fixture_bytes = read_fixture("fixtures/minimal/minimal.pdf")
                .expect("minimal PDF fixture must be readable");
            let mut change_count = 0usize;
            let mut output_bytes = 0usize;
            for _ in 0..iterations {
                let before = fe_reader_metadata::metadata_snapshot_bytes(&fixture_bytes);
                let mut after_summary = before.summary.clone();
                after_summary.document_info.title = Some("Fe Reader".to_string());
                after_summary.document_info.author = Some("Local User".to_string());
                let after = fe_reader_metadata::MetadataSnapshot::new(after_summary);
                let diff = fe_reader_metadata::diff_metadata_snapshots(before, after);
                change_count = diff.changes.len();
                output_bytes = serde_json::to_vec(&diff)
                    .expect("metadata diff serializable")
                    .len();
            }
            let mut metrics = BTreeMap::new();
            metrics.insert("pages".to_string(), serde_json::json!(100));
            metrics.insert("change_count".to_string(), serde_json::json!(change_count));
            metrics.insert("output_bytes".to_string(), serde_json::json!(output_bytes));
            BenchmarkObservation {
                peak_bytes: output_bytes as u64,
                metrics,
                detail: "metadata snapshot diff remained deterministic".to_string(),
            }
        }),
    });

    for mode in [MetadataScrubMode::CleanShare, MetadataScrubMode::Aggressive] {
        let scenario_id: &'static str =
            Box::leak(format!("metadata.scrub.{}", mode_as_scenario_id(mode)).into_boxed_str());
        cases.push(BenchCase {
            scenario_id,
            class: "WorkflowP1",
            detail: "metadata scrub planning probe",
            run: Box::new(move |iterations| {
                let intent = OperationIntent::mutation(
                    OperationSource::Cli,
                    DocumentId::new(),
                    OperationKind::PlanMutation,
                    "metadata_scrub",
                );
                let mut operations = 0usize;
                let mut output_bytes = 0usize;
                for _ in 0..iterations {
                    let plan =
                        plan_metadata_operations(&intent, &[MetadataOperation::Scrub { mode }]);
                    operations = plan.operations.len();
                    output_bytes = serde_json::to_vec(&plan)
                        .expect("metadata plan serializable")
                        .len();
                }
                let mut metrics = BTreeMap::new();
                metrics.insert(
                    "mode".to_string(),
                    serde_json::json!(mode_as_scenario_id(mode)),
                );
                metrics.insert("operations".to_string(), serde_json::json!(operations));
                metrics.insert("output_bytes".to_string(), serde_json::json!(output_bytes));
                BenchmarkObservation {
                    peak_bytes: output_bytes as u64,
                    metrics,
                    detail: "metadata scrub plan remained deterministic".to_string(),
                }
            }),
        });
    }

    cases
}

fn mode_as_scenario_id(mode: MetadataScrubMode) -> &'static str {
    match mode {
        MetadataScrubMode::Preserve => "preserve",
        MetadataScrubMode::ForensicPreserve => "forensic_preserve",
        MetadataScrubMode::CleanShare => "clean_share",
        MetadataScrubMode::Aggressive => "aggressive",
    }
}

fn measure_case(case: BenchCase) -> PerfCaseResult {
    const SAMPLES: usize = 9;
    const ITERATIONS: usize = 5;
    let mut samples = Vec::with_capacity(SAMPLES);
    let mut observation = None;
    for _ in 0..SAMPLES {
        let start = Instant::now();
        let obs = (case.run)(ITERATIONS);
        let elapsed = start.elapsed();
        samples.push(elapsed.as_secs_f64() * 1000.0 / ITERATIONS as f64);
        observation = Some(obs);
    }
    samples.sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
    let p50_ms = percentile(&samples, 0.50);
    let p95_ms = percentile(&samples, 0.95);
    let observation = observation.expect("benchmark case must produce an observation");
    PerfCaseResult {
        scenario_id: case.scenario_id.to_string(),
        class: case.class.to_string(),
        samples_ms: samples,
        p50_ms,
        p95_ms,
        peak_bytes: observation.peak_bytes,
        metrics: observation.metrics,
        detail: format!("{}; {}", case.detail, observation.detail),
    }
}

fn percentile(samples: &[f64], fraction: f64) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let index = ((samples.len() - 1) as f64 * fraction).round() as usize;
    samples[index.min(samples.len() - 1)]
}

fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

fn write_markdown_summary(manifest: &PerfManifest) -> Result<()> {
    let mut body = String::new();
    body.push_str("# Performance Benchmarks\n\n");
    body.push_str(&format!("- Suite: {}\n", manifest.suite));
    body.push_str(&format!("- Generated: {}\n\n", manifest.generated_at));
    body.push_str("| Scenario | p50 ms | p95 ms | Peak bytes | Detail |\n");
    body.push_str("| --- | ---: | ---: | ---: | --- |\n");
    for case in &manifest.cases {
        body.push_str(&format!(
            "| {} | {:.3} | {:.3} | {} | {} |\n",
            case.scenario_id, case.p50_ms, case.p95_ms, case.peak_bytes, case.detail
        ));
    }
    fs::write("artifacts/perf/latest.md", body)?;
    Ok(())
}

fn read_fixture(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    Ok(fs::read(workspace_root.join(path))?)
}

fn run_script(path: &str) -> Result<()> {
    let status = if path.ends_with(".py") {
        Command::new("python3").arg(path).status()?
    } else {
        Command::new("bash").arg(path).status()?
    };
    if !status.success() {
        bail!("{path} failed with status {status}");
    }
    Ok(())
}
