# Performance Engineering Plan

Fe Reader should treat performance as a product feature, not a cleanup task. Every wave gets explicit budgets, benchmarks, traces, and regression gates.

## Performance principles

1. **Measure before optimizing.** No speculative micro-optimizations in core PDF code.
2. **Budget visible user latency.** Page open, scroll, search, annotation, save, and redaction must have defined targets.
3. **Benchmark the pipeline, not only functions.** Rendering, text extraction, indexing, redaction, metadata, conversion, and platform integration all need end-to-end scenarios.
4. **Keep core deterministic.** Benchmarks must run from CLI fixtures without UI dependencies.
5. **Use per-platform profilers.** Generic Rust benchmarks are not enough for Windows/macOS/iOS/Android/Linux.
6. **Optimise hot paths behind contracts.** Swapping PDFium, GPU compositors, allocators, or parsers must not change public operation contracts.

## Performance budget classes

| Budget class | Examples | Gate |
|---|---|---|
| P0 interactive | tile render visible page, text selection, annotation stroke latency | hard gate by Wave 2 |
| P1 workflow | apply initials to 100 pages, metadata scrub, redact candidate plan | hard gate by Wave 3 |
| P2 batch | OCR, conversion, preflight, PDF/A validation | advisory until Wave 5 |
| P3 frontier | GPU overlays, local NLP, semantic search | advisory until Wave 6 |

## Initial numeric targets

These are starter budgets for implementation; they must be recalibrated with real fixture corpora.

| Operation | Target hardware | Budget |
|---|---|---:|
| CLI inspect 10-page PDF | laptop-class CPU | < 200 ms |
| CLI inspect 500-page text PDF | laptop-class CPU | < 2.5 s |
| first page rendered at 150 DPI | desktop/laptop | < 250 ms warm, < 600 ms cold |
| tile render 512x512 at 2x zoom | desktop/laptop | p95 < 40 ms warm |
| page text spans extraction | 1 page | p95 < 30 ms warm |
| search 500-page extracted index | local text index | p95 < 150 ms |
| add visual initials to 100 pages | no raster duplication | < 1.0 s |
| metadata snapshot/diff | 100-page PDF | < 150 ms |
| secure redaction sanitized rewrite | 100-page PDF | < 5 s initial target |
| UI annotation stroke-to-display | tablet/desktop | p95 < 16 ms, p99 < 33 ms |
| app cold start desktop | without opening file | < 1.5 s |

## Benchmark hierarchy

```text
benchmarks/
  budgets/performance-budgets.yaml
  fixtures/manifest.yaml
  reports/
  scenarios/
    open_inspect.yaml
    render_tiles.yaml
    extract_text.yaml
    search_index.yaml
    affidavit_apply.yaml
    metadata_scrub.yaml
    redaction_plan.yaml
    secure_rewrite.yaml
    conversion_markdown_docx.yaml
```

## Rust benchmark tools

| Tool | Use |
|---|---|
| `criterion = 0.8.2` | statistics-driven microbenchmarks and regression analysis |
| `divan = 0.1.21` | low-friction benches for parser/layout/render primitives |
| `iai-callgrind = 0.16.1` | deterministic CI performance checks on Linux through Callgrind/Cachegrind/DHAT |
| `pbench` or custom harness | tail latency reporting for p95/p99/p99.9 UI-like operations |
| `hyperfine` | CLI command-level benchmarking |
| `cargo flamegraph` | Linux/macOS CPU flamegraphs through perf/DTrace |
| `samply` | cross-platform sampling profiles viewable in Firefox Profiler |
| `pprof = 0.15.0` | opt-in embedded profiles for server/MCP/plugin scenarios |
| `cargo bloat` | binary size regressions |
| `cargo llvm-lines` | monomorphisation/code-size investigations |

## Per-platform profilers

| Platform | Primary tools | Notes |
|---|---|---|
| Linux | `perf`, `cargo flamegraph`, `heaptrack`, Valgrind/Callgrind/DHAT | best platform for CI perf gates |
| Windows | Windows Performance Recorder/Analyzer, ETW, Visual Studio Profiler, AMD uProf | use for WebView/Tauri startup and render scheduling |
| macOS | Instruments Time Profiler, Allocations, Metal System Trace, DTrace/flamegraph | use for native shell, sandbox/bookmark, rendering and energy |
| Android | Android Studio Profiler, Perfetto, simpleperf, systrace | measure scroll/annotation/input latency and battery |
| iOS/iPadOS | Xcode Instruments, Metal System Trace, Time Profiler, Allocations, Energy Log | measure Pencil latency, file coordination, and memory pressure |
| Web/PWA | Chrome Performance panel, Lighthouse, WebGPU capture tools | measure WASM load, file access, tile compositing |

The concrete platform playbooks live in [docs/platform-performance-playbooks.md](/Volumes/PortableSSD/GitHub/fe-reader/docs/platform-performance-playbooks.md).

## Build profiles

Use separate build profiles rather than one overloaded `release` profile.

```toml
[profile.dev]
opt-level = 0
debug = true
incremental = true

[profile.dev.package."*"]
opt-level = 1

[profile.release]
opt-level = 3
lto = false
codegen-units = 16
debug = "line-tables-only"
panic = "abort"
strip = "symbols"

[profile.release-thinlto]
inherits = "release"
lto = "thin"
codegen-units = 1

[profile.release-fat]
inherits = "release"
lto = "fat"
codegen-units = 1

[profile.bench]
opt-level = 3
debug = true
lto = "thin"
codegen-units = 1
```

`release-thinlto` should be the default distribution candidate. `release-fat` is measured but not assumed faster.

## Release optimisation lane

The release lane compares distribution builds and allocator variants, then writes a signed report under `artifacts/perf/release/`.

```bash
cargo xtask perf release --suite default
```

The default comparison builds:

- `release-thinlto` with the system allocator;
- `release-fat` with the system allocator;
- `release-thinlto` with the optional `mimalloc-allocator` feature.

The lane records binary size and size-tool status in:

- `artifacts/perf/release/manifest.json`
- `artifacts/perf/release/summary.md`
- `artifacts/perf/release/summary.md.sha256`

## PGO and BOLT lane

Add an `xtask` command:

```bash
cargo xtask perf pgo-train --suite default
cargo xtask perf pgo-build --profile release-thinlto
cargo xtask perf compare --baseline main --candidate HEAD
```

Training workloads must include:

- open + render first page;
- continuous scroll through a 200-page PDF;
- text extraction and search;
- metadata diff/scrub;
- apply affidavit template;
- secure redaction rewrite;
- export Markdown/DOCX through providers.

PGO/BOLT is only accepted into release builds if it improves at least two P0/P1 budgets and does not regress startup, binary size, or memory beyond thresholds.

## Allocation and memory plan

- Start with the system allocator.
- Add optional `mimalloc = 0.1.50` feature for desktop performance experiments.
- Do not make non-system allocators default until measured on all desktop platforms.
- Track peak RSS, allocation count, tile cache size, text span cache size, and bitmap memory.
- Add explicit memory budgets for mobile.

## Data structures and hot-path rules

- Use stable object IDs and compact indexes for page/text/layout caches.
- Avoid copying PDF bytes; prefer `Arc<[u8]>`, memory maps where safe, and slice views where the parser supports them.
- Avoid storing rendered bitmaps in core models.
- Tile cache must be bounded by bytes, not page count.
- Use `rayon = 1.12.0` only for embarrassingly parallel batch jobs; avoid uncontrolled parallelism in interactive UI paths.
- The write engine should use streaming writes where possible and avoid whole-document in-memory duplication for large files.

## Regression gate levels

| Gate | When | Commands |
|---|---|---|
| Smoke | every Conductor phase | `cargo test`, CLI smoke, selected divan benches where available |
| Standard | daily CI | Criterion, divan, hyperfine scenarios, binary size check |
| Deterministic | nightly Linux CI | iai-callgrind, Cachegrind, DHAT, fuzz smoke |
| Platform | release candidate | WPR/WPA, Instruments, Android Profiler, Xcode Instruments |
| Release | before publishing | compare against previous release and store signed report |

## Required reports

Every milestone should emit:

```text
artifacts/perf/
  manifest.json
  budgets.json
  criterion/
  divan/
  iai-callgrind/
  flamegraphs/
  platform/
  binary-size/
  memory/
  summary.md
```

The summary must include:

- budget pass/fail table;
- top 10 hot functions;
- peak memory by scenario;
- binary size by target;
- regressions since previous baseline;
- accepted performance-risk exceptions.

## Rendering promotion gate

Rendering and GPU performance claims are governed by the `rendering_performance_promotion` feature gate. The gate is disabled by default and requires visual regression, differential oracle, budget and platform evidence before claims are promoted. CPU/PDFium-safe fallback remains mandatory for every promoted renderer path.
