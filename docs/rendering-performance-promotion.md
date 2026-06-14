# Rendering Performance Promotion

The `rendering_performance_promotion` feature gate is disabled by default and does not block desktop stable launch. It exists to prevent unsupported rendering, GPU and performance claims from moving into public release notes without reproducible evidence.

## Promotion Boundary

Rendering promotion is evidence-first:

- CPU-safe rendering fallback is always required.
- GPU and frontier renderer paths remain optional and disabled by default.
- ML rendering is out of scope.
- Performance claims require reproducible budgets, visual regression evidence, differential oracle evidence and platform startup, memory, power and thermal summaries.

## Evidence Map

| Evidence | Current artifact |
|---|---|
| CPU render smoke | `scripts/wave1_render_smoke.sh` |
| Visual regression | `scripts/visual_regression_compare.py --smoke` |
| Differential oracle | `scripts/differential_oracle_smoke.sh` |
| Performance budget manifest | `benchmarks/budgets/performance-budgets.yaml` |
| Platform startup, memory, power and thermal summaries | `docs/platform-performance-playbooks.md` and `fixtures/rendering/performance/platform-summary.preview.json` |
| Optional GPU frontier smoke | `scripts/gpu_frontier_smoke.sh` |

## Budget Summary

The initial promotion gate checks `render.first_page.150dpi`, `render.tile.512_2x_warm`, `app.cold_start.desktop` and a planned `power.thermal.desktop_render_loop` budget. Budget values live in `benchmarks/budgets/performance-budgets.yaml` where possible, with the preview promotion snapshot documenting the fallback for each scenario.

## GPU Promotion

GPU promotion requires feature flags to remain disabled by default, CPU fallback, platform-specific Linux/macOS/Windows smoke, visual regression reports, differential oracle reports, memory budget reports, power/thermal reports and rollback evidence. If any evidence is incomplete, GPU paths stay disabled and CPU/PDFium-safe rendering remains the supported fallback.

## Rollback

Rollback disables optional GPU and frontier renderers, keeps CPU/PDFium-safe rendering, retains evidence artifacts and removes any public GPU/performance claim that lacks current reports.

## Evidence

The machine-readable promotion contract is `contracts/snapshots/rendering/rendering-performance-promotion.preview.json`.

Run:

```bash
python3 scripts/rendering_performance_promotion_check.py
```
