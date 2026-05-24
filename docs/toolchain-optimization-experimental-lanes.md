# Toolchain Optimisation and Experimental Lanes

Fe Reader should use bleeding-edge techniques where they are measurable and reversible, not by destabilising the core.

## Build-speed lane

- `sccache` for repeated local/CI builds.
- `cargo-chef` or equivalent dependency-layer caching in containers.
- `mold`/`lld` evaluation where supported.
- `nextest` for fast test execution.
- feature-targeted CI matrices instead of all-features everything for every PR.

## Runtime-performance lane

- ThinLTO default candidate for release.
- FatLTO measured but not assumed.
- PGO training corpus for reader/open/render/search/workflow scenarios.
- BOLT or platform-equivalent binary optimisation as an experiment lane.
- allocator experiments: system allocator vs mimalloc/jemalloc where platform policy permits.
- SIMD/text scanning experiments isolated behind feature flags.

## Renderer lane

- PDFium adapter is the production path.
- GPU compositor and overlay acceleration live in `fe_reader_render_gpu`.
- Vello/wgpu/skia experiments are allowed only behind `frontier-render-*` flags.
- Renderer experiments require visual regression and performance evidence.

## Experimental dependency rules

A bleeding-edge dependency must define:

```text
owner
reason
risk
feature flag
maturity level
benchmark or compatibility target
exit criteria
fallback path
review date
```

## Prohibited without explicit ADR

- Nightly-only Rust in release builds.
- Git dependencies in stable releases.
- Experimental allocator defaulting on all platforms.
- Renderer changes without visual regression baselines.
- GPU path as sole renderer.
- PGO profile that only optimises synthetic microbenchmarks.
