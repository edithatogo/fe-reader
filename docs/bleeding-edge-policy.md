# Bleeding-Edge Version Policy

Fe Reader should be modern and aggressive without letting unstable dependencies contaminate the core engine. The rule is **latest stable by default, experimental lanes for frontier work**.

## Dependency lanes

| Lane | Purpose | Allowed dependencies | Failure policy |
|---|---|---|---|
| `stable-core` | Libraries required by the headless engine | mature crates, pinned semver, no git dependencies | hard CI failure |
| `stable-app` | Tauri, UI shell, platform bridges | latest stable versions that build on all target OSes | hard CI failure after Wave 1 |
| `frontier-render` | GPU compositor, vector overlay engines | `wgpu`, `vello`, `skia-safe`, PDFium alternatives | advisory until benchmark win is proven |
| `frontier-plugin` | WASM plugin runtimes | `extism`, later `wasmtime` | disabled by default until sandbox tests pass |
| `frontier-intelligence` | optional local NLP/NER/RAG | Candle/Burn/tokenizers/ONNX providers | late-wave only; never required by core |
| `experimental-git` | short-lived trial branches | git refs/forks | must have expiry date and owner |

## Current recommended versions for first implementation

The package intentionally pins concrete versions so a coding agent can start without ambiguity. Refresh these before each release candidate.

```toml
[workspace.package]
edition = "2024"
rust-version = "1.95.0"

[workspace.dependencies]
lopdf = "0.40.0"
pdf-writer = "0.14.0"
pdfium-render = "0.9.1"
uniffi = "0.31.1"
rmcp = "1.6.0"
extism = "deferred until a non-vulnerable Wasmtime-backed release is available"
wasmtime = { version = "44.0.1", optional = true }
wgpu = "29.0.3"
vello = "0.8.0"
skia-safe = "0.93.1"
criterion = "0.8.2"
divan = "0.1.21"
iai-callgrind = "0.16.1"
pprof = { version = "0.15.0", features = ["flamegraph", "criterion"] }
rayon = "1.12.0"
dashmap = "6.1.0"
mimalloc = "0.1.50"
```

## What “bleeding edge” does not mean

Do not put nightly-only compiler features, GPU experiments, AI inference stacks, or plugin runtimes directly inside `fe_reader_core`. They belong behind traits and feature flags:

```text
fe_reader_core             stable, deterministic, fuzzed
fe_reader_render_pdfium    production render adapter
fe_reader_render_gpu       optional compositor/overlay experiments
fe_reader_plugin_host      optional WASM plugin runtime, no runtime dependency in Wave 0
fe_reader_intelligence_*   late-wave optional local NLP/RAG
```

## Fork and git dependency rules

A fork is allowed only when one of these is true:

1. A blocking bug prevents a Fe Reader milestone.
2. A security fix is required and upstream has not merged it.
3. A required API is accepted upstream but not yet released.
4. A short spike is needed to evaluate performance.

Every fork must have:

```yaml
upstream: https://github.com/example/project
fork: https://github.com/fereader/project
owner: track-owner
reason: blocking bug/performance/security/api spike
opened: YYYY-MM-DD
rebase_cadence: weekly
exit_criteria: upstream release includes fix OR spike rejected
max_lifetime_days: 60
```

## Release train policy

- `main`: stable-core and stable-app only.
- `next`: latest stable versions plus optional frontier crates.
- `experiments/*`: git dependencies, nightly flags, alternate renderers, local inference.
- `release/*`: dependency refresh frozen except for security fixes.

## Review rule

Conductor review must fail if:

- a git dependency enters `main` without a fork record;
- `fe_reader_core` imports Tauri/PDFium/MCP/WASM/AI/UI/platform crates;
- a frontier feature becomes a default feature;
- a dependency update removes support for one of the five native platforms without explicit approval.

Every Wave 6 frontier capability remains feature-gated until accepted benchmark,
visual, compatibility or policy evidence supports promotion. Promotion requires
an ADR, explicit owner, rollback plan and exit criteria; local-intelligence,
GPU and PGO/BOLT experiments must stay disabled by default until that evidence
exists.

## Evidence

- `scripts/wave6_frontier_optional_smoke.py` validates the preview policy snapshot and the frontier lane documentation.
- Frontier lanes stay advisory and disabled by default until an ADR promotes them.
