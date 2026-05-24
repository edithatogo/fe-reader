# Track N: Performance Engineering

## Purpose

Make performance measurable, enforceable, and cross-platform from the first implementation wave.

## Scope

- Benchmark scenario registry.
- Performance budgets and regression gates.
- CLI smoke benchmarks.
- Criterion/divan/iai-callgrind benchmark harnesses.
- PGO/LTO build profiles.
- Platform profiler playbooks.
- Tile-render, text extraction, workflow, redaction, metadata, conversion, and startup performance tracking.
- Binary size and memory budgets.

## Non-goals

- Do not optimise before contracts exist.
- Do not add unsafe code for speed without a safety review.
- Do not make GPU, non-system allocators, or PGO default until measured.
- Do not hide failed performance gates with auto-fix.

## Deliverables

- `docs/performance-engineering.md`
- `docs/bleeding-edge-policy.md`
- `benchmarks/budgets/performance-budgets.yaml`
- `benchmarks/scenarios/*.yaml`
- `contracts/rust/performance.rs`
- `scripts/perf_smoke.sh`
- `scripts/perf_profile_linux.sh`
- `ci/performance.yml.sample`
- `xtask perf` command plan

## Exit criteria

- Smoke performance script runs without blocking early development.
- Standard benchmark targets compile.
- Every P0/P1 feature has a budget or a TODO budget waiver.
- Regression report format exists.
- Release builds have `release-thinlto`, `release-fat`, and `bench` profiles.
