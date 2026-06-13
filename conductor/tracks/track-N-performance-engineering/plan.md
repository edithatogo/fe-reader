# Track N Plan

## N0: Baseline harness

Dependencies: A0 core contracts, K0 quality scaffolding.

Tasks:

1. [x] Materialise benchmark directories and budget YAML.
2. [x] Add `contracts/rust/performance.rs` to the implementation workspace.
3. [x] Add `cargo xtask perf smoke` command.
4. [x] Add CLI-level `fe-reader --version` and `fe-reader inspect` smoke scenarios.
5. [x] Add advisory Conductor phase gate call to `scripts/perf_smoke.sh`.

Review:

```bash
scripts/perf_smoke.sh
cargo bench --workspace --no-run
```

## N1: Reader performance

Dependencies: B1 tile rendering, A1 text spans, D1 shell.

Tasks:

1. [x] Add tile render benchmark.
2. [x] Add first-page render benchmark.
3. [x] Add text span extraction benchmark.
4. [x] Track bitmap cache memory by byte budget.
5. [x] Add visible-page prefetch/cancellation metrics.

Review:

```bash
cargo xtask perf bench --suite reader
cargo xtask perf compare --budget benchmarks/budgets/performance-budgets.yaml
```

## N2: Workflow performance

Dependencies: A2 patch planning, F1 workflow templates, G1 metadata.

Tasks:

1. [x] Add affidavit apply benchmark for 10, 100, and 500 pages.
2. [x] Add metadata snapshot/diff/scrub benchmark.
3. [x] Add secure-redaction plan and rewrite benchmark.
4. [x] Track output size, XObject reuse count, and verification time.

Review:

```bash
cargo xtask perf bench --suite workflows
cargo xtask perf report --format markdown
```

## N3: Deterministic Linux CI

Dependencies: N0/N1 benchmark targets compile.

Tasks:

1. [x] Add `iai-callgrind` targets for parser, page tree, text extraction, template planning, metadata diff.
2. [x] Add Cachegrind and DHAT runs for allocation-heavy scenarios.
3. [x] Store baseline artifacts.
4. [x] Fail nightly CI on material regression after baseline is accepted.

Review:

```bash
cargo bench --bench callgrind_core
```

## N4: Platform profiler passes

Dependencies: D/E platform shells.

Tasks:

1. [x] Windows: WPR/WPA playbook for startup, file open, tile rendering, WebView/native bridge.
2. [x] macOS: Instruments Time Profiler/Allocations/Energy playbook.
3. [x] Linux: perf/flamegraph/heaptrack playbook.
4. [x] Android: Android Studio Profiler/Perfetto/simpleperf playbook.
5. [x] iOS: Xcode Instruments/Metal System Trace/Energy Log playbook.
6. [x] Web: browser performance panel and WASM load profile.

Review: attach profiler traces or summaries to release candidate report.

## N5: Release optimisation lane

Dependencies: stable perf corpus and budgets.

Tasks:

1. [x] Implement `release-thinlto` and `release-fat` build comparison.
2. [x] Add PGO training workloads.
3. [x] Compare system allocator vs optional mimalloc on desktop.
4. [x] Track binary size with `cargo bloat` and `cargo llvm-lines`.
5. [x] Generate signed `artifacts/perf/summary.md` before publishing.

Review:

```bash
cargo xtask perf pgo-train --suite default
cargo xtask perf pgo-build
cargo xtask perf compare --baseline previous-release --candidate current
cargo xtask perf release --suite default
```
