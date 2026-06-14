#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT = ROOT / "contracts/snapshots/rendering/rendering-performance-promotion.preview.json"
PLATFORM_FIXTURE = ROOT / "fixtures/rendering/performance/platform-summary.preview.json"
EVIDENCE_DIR = ROOT / "target/release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

REQUIRED_EVIDENCE = {
    "cpu_render_smoke",
    "visual_regression_smoke",
    "differential_oracle_smoke",
    "performance_budget_manifest",
    "platform_startup_memory_power_summary",
    "gpu_frontier_smoke_if_gpu_claimed",
}
REQUIRED_GPU_PROMOTION = {
    "feature_flag_disabled_by_default",
    "cpu_fallback",
    "platform_specific_smoke_linux",
    "platform_specific_smoke_macos",
    "platform_specific_smoke_windows",
    "visual_regression_report",
    "differential_oracle_report",
    "memory_budget_report",
    "power_thermal_budget_report",
    "rollback_plan",
}
REQUIRED_SCENARIOS = {
    "render.first_page.150dpi",
    "render.tile.512_2x_warm",
    "app.cold_start.desktop",
}
DOC_TOKENS = {
    "docs/rendering-performance-promotion.md": [
        "rendering_performance_promotion",
        "disabled by default",
        "does not block desktop stable launch",
        "CPU-safe rendering fallback",
        "Visual regression",
        "Differential oracle",
        "GPU paths stay disabled",
        "Rollback disables optional GPU",
    ],
    "docs/performance-engineering.md": [
        "rendering_performance_promotion",
        "CPU/PDFium-safe fallback",
    ],
    "docs/hardware-acceleration.md": [
        "rendering_performance_promotion",
        "disabled by default",
    ],
    "docs/launch-limitations-support.md": [
        "rendering_performance_promotion",
        "GPU",
    ],
    "README.md": [
        "docs/rendering-performance-promotion.md",
        "rendering_performance_promotion",
    ],
    "docs-site/src/content/docs/rendering-performance-promotion.md": [
        "Rendering Performance Promotion",
        "rendering_performance_promotion",
        "disabled by default",
    ],
}

failures: list[str] = []


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        failures.append(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def load_json(path: Path) -> dict:
    if not path.exists():
        failures.append(f"missing file: {path.relative_to(ROOT)}")
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        failures.append(f"{path.relative_to(ROOT)} invalid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        failures.append(f"{path.relative_to(ROOT)} must contain an object")
        return {}
    return data


def load_yaml(path: Path) -> dict:
    if not path.exists():
        failures.append(f"missing file: {path.relative_to(ROOT)}")
        return {}
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        failures.append(f"{path.relative_to(ROOT)} must contain a mapping")
        return {}
    return data


def require_snapshot(snapshot: dict) -> None:
    expected_scalars = {
        "contract": "rendering_performance_promotion",
        "stability": "preview",
        "feature_gate": "rendering_performance_promotion",
        "owner": "rendering-maintainers",
        "default_state": "disabled",
        "gpu_default": "disabled",
        "frontier_renderer_default": "disabled",
        "promotion_policy": "evidence_required_before_claims",
    }
    for key, expected in expected_scalars.items():
        if snapshot.get(key) != expected:
            failures.append(f"rendering promotion {key} must be {expected}")
    if snapshot.get("launch_blocking") is not False:
        failures.append("rendering performance promotion must not block desktop stable launch")
    if snapshot.get("cpu_fallback_required") is not True:
        failures.append("CPU fallback must be required")
    if snapshot.get("ml_rendering_allowed") is not False:
        failures.append("ML rendering must remain out of scope")

    evidence = set(snapshot.get("required_evidence", []))
    missing_evidence = sorted(REQUIRED_EVIDENCE - evidence)
    if missing_evidence:
        failures.append(f"missing required evidence: {', '.join(missing_evidence)}")

    gpu = set(snapshot.get("gpu_promotion_requires", []))
    missing_gpu = sorted(REQUIRED_GPU_PROMOTION - gpu)
    if missing_gpu:
        failures.append(f"missing GPU promotion requirements: {', '.join(missing_gpu)}")

    rollback = snapshot.get("rollback")
    if not isinstance(rollback, dict):
        failures.append("rollback must be a mapping")
    else:
        if rollback.get("strategy") != "disable_optional_gpu_and_frontier_renderers":
            failures.append("rollback must disable optional GPU and frontier renderers")
        if rollback.get("fallback") != "cpu_pdfium_safe_rendering":
            failures.append("rollback fallback must be cpu_pdfium_safe_rendering")

    budgets = snapshot.get("budget_classes")
    if not isinstance(budgets, list) or not budgets:
        failures.append("budget_classes must be a non-empty list")
        return
    seen = {budget.get("scenario_id") for budget in budgets if isinstance(budget, dict)}
    missing = sorted(REQUIRED_SCENARIOS - seen)
    if missing:
        failures.append(f"promotion snapshot missing budget scenarios: {', '.join(missing)}")
    for budget in budgets:
        if not isinstance(budget, dict):
            failures.append("budget entry must be an object")
            continue
        if "fallback" not in budget:
            failures.append(f"budget {budget.get('scenario_id')} must declare fallback")
        if budget.get("scenario_id", "").startswith("render.") and budget.get("max_p95_ms") is None:
            failures.append(f"render budget {budget.get('scenario_id')} must declare max_p95_ms")

    targets = snapshot.get("platform_targets")
    if not isinstance(targets, list) or len(targets) < 3:
        failures.append("platform_targets must cover linux, macos and windows")
        return
    platforms = {target.get("platform") for target in targets if isinstance(target, dict)}
    for platform in ("linux", "macos", "windows"):
        if platform not in platforms:
            failures.append(f"platform target missing: {platform}")
    for target in targets:
        if not isinstance(target, dict):
            continue
        script = target.get("required_script")
        if not isinstance(script, str) or not (ROOT / script).exists():
            failures.append(f"platform target script missing: {script}")


def require_budget_alignment(snapshot: dict) -> None:
    budgets = load_yaml(ROOT / "benchmarks/budgets/performance-budgets.yaml")
    rows = budgets.get("budgets", [])
    if not isinstance(rows, list):
        failures.append("performance budgets file must contain budgets list")
        return
    by_id = {row.get("scenario_id"): row for row in rows if isinstance(row, dict)}
    for scenario in REQUIRED_SCENARIOS:
        if scenario not in by_id:
            failures.append(f"performance budgets missing scenario: {scenario}")
    for budget in snapshot.get("budget_classes", []):
        if not isinstance(budget, dict):
            continue
        scenario = budget.get("scenario_id")
        if scenario not in by_id or budget.get("max_p95_ms") is None:
            continue
        if by_id[scenario].get("max_p95_ms") != budget.get("max_p95_ms"):
            failures.append(f"budget mismatch for {scenario}")


def require_platform_fixture(fixture: dict) -> None:
    if fixture.get("privacy_policy") != "synthetic_no_private_documents":
        failures.append("platform fixture must be synthetic_no_private_documents")
    if fixture.get("feature_gate") != "rendering_performance_promotion":
        failures.append("platform fixture feature gate mismatch")
    sources = set(fixture.get("evidence_sources", []))
    for source in (
        "benchmarks/budgets/performance-budgets.yaml",
        "scripts/wave1_render_smoke.sh",
        "scripts/visual_regression_compare.py",
        "scripts/differential_oracle_smoke.sh",
        "scripts/perf_smoke.sh",
        "scripts/gpu_frontier_smoke.sh",
    ):
        if source not in sources:
            failures.append(f"platform fixture missing evidence source: {source}")
    platforms = fixture.get("platforms")
    if not isinstance(platforms, list) or len(platforms) < 3:
        failures.append("platform fixture must cover desktop platforms")
        return
    for platform in platforms:
        if platform.get("gpu_promotion_status") != "disabled_until_evidence":
            failures.append(f"{platform.get('platform')} GPU promotion must stay disabled until evidence")
        if not isinstance(platform.get("startup_budget_ms"), int):
            failures.append(f"{platform.get('platform')} missing startup_budget_ms")
        if not isinstance(platform.get("peak_rss_budget_mb"), int):
            failures.append(f"{platform.get('platform')} missing peak_rss_budget_mb")


def require_docs() -> None:
    for rel, tokens in DOC_TOKENS.items():
        text = read_text(rel)
        if not text:
            continue
        for token in tokens:
            if token not in text:
                failures.append(f"{rel} missing token: {token}")


def require_ci_wiring() -> None:
    command = "python3 scripts/rendering_performance_promotion_check.py"
    pr_contracts = read_text(".github/workflows/00-pr-contracts.yml")
    phase_gate = read_text("scripts/conductor_phase_gate.sh")
    matrix_text = read_text("contracts/ci/contract-test-matrix.yaml")
    if command not in pr_contracts:
        failures.append("PR contracts must run rendering_performance_promotion_check.py")
    if command not in phase_gate:
        failures.append("Conductor phase gate must run rendering_performance_promotion_check.py")
    if matrix_text:
        matrix = yaml.safe_load(matrix_text)
        entry = (matrix or {}).get("matrix", {}).get("rendering_performance_promotion")
        if not isinstance(entry, dict):
            failures.append("contract test matrix missing rendering_performance_promotion")
        else:
            if entry.get("gate") != "advisory_post_launch":
                failures.append("rendering_performance_promotion must remain advisory_post_launch")
            if entry.get("blocks_pr") is not False:
                failures.append("rendering_performance_promotion must not block PR promotion as a feature gate")
            if entry.get("promotion_requires_adr") is not True:
                failures.append("GPU/frontier rendering promotion must require ADR")


def require_docs_nav() -> None:
    config = read_text("docs-site/astro.config.mjs")
    if "rendering-performance-promotion" not in config:
        failures.append("docs site sidebar must include rendering-performance-promotion")


def main() -> int:
    snapshot = load_json(SNAPSHOT)
    platform_fixture = load_json(PLATFORM_FIXTURE)
    require_snapshot(snapshot)
    require_budget_alignment(snapshot)
    require_platform_fixture(platform_fixture)
    require_docs()
    require_ci_wiring()
    require_docs_nav()

    report = {
        "check": "rendering_performance_promotion",
        "status": "fail" if failures else "pass",
        "feature_gate": snapshot.get("feature_gate"),
        "launch_blocking": snapshot.get("launch_blocking"),
        "cpu_fallback_required": snapshot.get("cpu_fallback_required"),
        "gpu_default": snapshot.get("gpu_default"),
        "failures": failures,
    }
    (EVIDENCE_DIR / "rendering-performance-promotion.json").write_text(
        json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
    )

    if failures:
        print("RENDERING PERFORMANCE PROMOTION CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1
    print("rendering performance promotion: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
