#!/usr/bin/env python3
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "target" / "frontier-reports"
EVIDENCE.mkdir(parents=True, exist_ok=True)


def read(rel):
    return (ROOT / rel).read_text(encoding="utf-8")


def load_json(rel):
    return json.loads(read(rel))


def require(condition, message, failures):
    if not condition:
        failures.append(message)


def main():
    failures = []
    snapshot = load_json("contracts/snapshots/frontier/wave6.frontier-policy.preview.json")
    require(snapshot["contract"] == "wave6_frontier_policy", "frontier snapshot contract name drifted", failures)
    require(snapshot["version"] == "0.1.0", "frontier snapshot version drifted", failures)
    require(snapshot["stability"] == "preview", "frontier snapshot stability drifted", failures)
    require(snapshot["default_state"] == "disabled", "frontier snapshot must be disabled by default", failures)
    require(snapshot["mutation_policy"] == "no_high_risk_auto_mutation", "frontier snapshot must forbid high-risk auto mutation", failures)
    require(
        snapshot["evidence_policy"] == "suggestions_must_cite_local_or_corpus_evidence",
        "frontier snapshot evidence policy drifted",
        failures,
    )
    require(
        snapshot["promotion_requires"] == [
            "accepted_benchmark_evidence",
            "visual_or_contract_regression_evidence",
            "adr_approval",
            "rollback_plan",
        ],
        "frontier snapshot promotion requirements drifted",
        failures,
    )
    for feature in snapshot["features"]:
        require("flag_id" in feature, "frontier feature missing flag id", failures)
        require(feature["category"] in {"local_intelligence", "gpu_acceleration", "toolchain_optimization"}, "frontier feature category drifted", failures)
        require(feature["default_enabled"] is False, f"frontier feature enabled by default: {feature['flag_id']}", failures)
        require(feature["requires_policy_check"] is True, f"frontier feature missing policy check: {feature['flag_id']}", failures)
        require(feature["requires_evidence_citations"] is True, f"frontier feature missing evidence citations: {feature['flag_id']}", failures)

    gpu_toml = read("crates/fe_reader_render_gpu/Cargo.toml")
    require("default = []" in gpu_toml, "GPU frontier crate must keep default features empty", failures)
    require("optional = true" in gpu_toml, "GPU frontier dependencies must be optional", failures)

    core_toml = read("crates/fe_reader_core/Cargo.toml").lower()
    for token in ["candle", "burn", "tokenizers", "ort", "wgpu", "vello", "skia-safe"]:
        require(token not in core_toml, f"fe_reader_core must not depend on frontier token {token}", failures)

    for script in [
        "scripts/gpu_frontier_smoke.sh",
        "scripts/toolchain_experiment_smoke.sh",
        "scripts/frontier_ci_check.py",
    ]:
        text = read(script)
        require("target/frontier-reports" in text, f"{script} must emit advisory frontier reports", failures)

    docs = (
        read("docs/bleeding-edge-policy.md")
        + "\n"
        + read("docs/performance-engineering.md")
        + "\n"
        + read("docs/toolchain-optimization-experimental-lanes.md")
    )
    for token in [
        "feature-gated",
        "benchmark",
        "rollback",
        "frontier-intelligence",
        "owner",
        "exit criteria",
        "visual regression",
    ]:
        require(token in docs, f"frontier docs missing {token}", failures)
    require("main: stable-core and stable-app only." in docs, "frontier docs missing main lane rule", failures)
    require("Promotion requires an ADR" in docs or "Promotion requires an ADR" in read("docs/bleeding-edge-policy.md"), "frontier docs missing promotion ADR rule", failures)

    report = {
        "check": "wave6_frontier_optional",
        "status": "pass",
        "policy_snapshot": "contracts/snapshots/frontier/wave6.frontier-policy.preview.json",
        "advisory_scripts": [
            "scripts/gpu_frontier_smoke.sh",
            "scripts/toolchain_experiment_smoke.sh",
        ],
        "governance": {
            "feature_gated": True,
            "benchmark_required": True,
            "rollback_required": True,
            "default_disabled": True,
            "promotion_requirements": snapshot["promotion_requires"],
        },
    }
    evidence = EVIDENCE / "wave6-frontier-optional.json"
    evidence.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")

    if failures:
        for failure in failures:
            print(f"wave6 frontier failure: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print("wave6 frontier optional smoke: ok")


if __name__ == "__main__":
    main()
