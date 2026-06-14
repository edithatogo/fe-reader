#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT = ROOT / "contracts/snapshots/frontier/frontier-intelligence-governance.preview.json"
WAVE6 = ROOT / "contracts/snapshots/frontier/wave6.frontier-policy.preview.json"
EVAL_MANIFEST = ROOT / "fixtures/frontier/evaluation/manifest.json"
EVIDENCE_DIR = ROOT / "target/frontier-reports"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

REQUIRED_PROMOTION = {
    "privacy_review",
    "security_review",
    "synthetic_or_public_eval_report",
    "resource_budget_report",
    "explicit_opt_in_ui_cli",
    "adr_approval",
    "rollback_plan",
}
REQUIRED_CONTROLS = {
    "explicit_opt_in_ui",
    "explicit_opt_in_cli",
    "disable_switch",
    "purge_local_models",
    "purge_local_indexes",
}
FORBIDDEN_CORE_TOKENS = {
    "candle",
    "burn",
    "tokenizers",
    "ort",
    "onnxruntime",
    "llama",
    "rag",
    "embedding",
}
DOC_TOKENS = {
    "docs/frontier-intelligence-governance.md": [
        "frontier_intelligence_preview",
        "disabled by default",
        "does not block desktop stable launch",
        "synthetic or public",
        "no private document text",
        "explicit opt-in",
        "disable switch",
        "rollback",
        "resource-budget",
    ],
    "docs/local-ai-future-roadmap.md": [
        "frontier_intelligence_preview",
        "disabled by default",
        "synthetic or public",
    ],
    "docs/launch-limitations-support.md": [
        "frontier_intelligence_preview",
        "ML/RAG",
    ],
    "README.md": [
        "docs/frontier-intelligence-governance.md",
        "frontier_intelligence_preview",
    ],
    "docs-site/src/content/docs/frontier-intelligence-governance.md": [
        "Frontier Intelligence Governance",
        "frontier_intelligence_preview",
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


def require_snapshot(snapshot: dict) -> None:
    expected_scalars = {
        "contract": "frontier_intelligence_governance",
        "stability": "preview",
        "feature_gate": "frontier_intelligence_preview",
        "owner": "frontier-maintainers",
        "default_state": "disabled",
        "network_default": "disabled",
        "telemetry_default": "disabled",
        "model_download_default": "disabled",
        "mutation_policy": "no_high_risk_auto_mutation",
        "privacy_policy": "local_only_no_private_eval_corpus",
    }
    for key, expected in expected_scalars.items():
        if snapshot.get(key) != expected:
            failures.append(f"frontier governance {key} must be {expected}")
    if snapshot.get("launch_blocking") is not False:
        failures.append("frontier intelligence governance must not block desktop stable launch")

    controls = set(snapshot.get("user_controls", []))
    missing_controls = sorted(REQUIRED_CONTROLS - controls)
    if missing_controls:
        failures.append(f"frontier governance missing user controls: {', '.join(missing_controls)}")

    promotion = set(snapshot.get("promotion_requires", []))
    missing_promotion = sorted(REQUIRED_PROMOTION - promotion)
    if missing_promotion:
        failures.append(f"frontier governance missing promotion requirements: {', '.join(missing_promotion)}")

    budgets = snapshot.get("resource_budgets")
    if not isinstance(budgets, dict):
        failures.append("frontier governance missing resource_budgets mapping")
    else:
        for key in (
            "network_requests_without_user_action",
            "private_document_text_in_logs",
            "private_document_text_in_eval_fixtures",
        ):
            if budgets.get(key) != 0:
                failures.append(f"frontier governance budget {key} must be zero")

    fallback = set(snapshot.get("deterministic_fallback", []))
    for token in ("deterministic_extraction", "deterministic_search", "workflow_packs"):
        if token not in fallback:
            failures.append(f"frontier governance missing deterministic fallback: {token}")

    rollback = snapshot.get("rollback")
    if not isinstance(rollback, dict) or rollback.get("strategy") != "disable_frontier_flags_and_use_deterministic_fallbacks":
        failures.append("frontier governance rollback strategy is missing or unsafe")

    for feature in snapshot.get("preview_features", []):
        if not isinstance(feature, dict):
            failures.append("frontier preview feature must be an object")
            continue
        flag_id = str(feature.get("flag_id", ""))
        if not flag_id.startswith("frontier_intelligence_preview."):
            failures.append(f"frontier preview flag must use feature gate prefix: {flag_id}")
        for key in (
            "default_enabled",
            "requires_policy_check",
            "requires_local_model_provenance",
            "requires_evidence_citations",
            "forbids_high_risk_auto_mutation",
        ):
            expected = False if key == "default_enabled" else True
            if feature.get(key) is not expected:
                failures.append(f"{flag_id} {key} must be {expected}")


def require_eval_manifest(manifest: dict) -> None:
    if manifest.get("privacy_policy") != "synthetic_or_public_only":
        failures.append("frontier eval manifest must be synthetic_or_public_only")
    for key in ("no_private_documents", "no_private_prompts", "no_credentials"):
        if manifest.get(key) is not True:
            failures.append(f"frontier eval manifest must set {key}=true")
    if manifest.get("network_required") is not False:
        failures.append("frontier eval manifest must not require network")

    fixtures = manifest.get("fixtures")
    if not isinstance(fixtures, list) or not fixtures:
        failures.append("frontier eval manifest must list fixtures")
        return
    for fixture in fixtures:
        if not isinstance(fixture, dict):
            failures.append("frontier eval fixture must be an object")
            continue
        if fixture.get("source") not in {"synthetic", "public_domain", "public_corpus"}:
            failures.append(f"frontier eval fixture source is not privacy-safe: {fixture.get('fixture_id')}")
        if fixture.get("contains_private_data") is not False:
            failures.append(f"frontier eval fixture contains private data: {fixture.get('fixture_id')}")
        expected = set(fixture.get("expected_evidence", []))
        for token in ("page", "span", "bounding_box", "source_fixture"):
            if token not in expected:
                failures.append(f"frontier eval fixture missing expected evidence {token}: {fixture.get('fixture_id')}")


def require_wave6_alignment() -> None:
    wave6 = load_json(WAVE6)
    if not wave6:
        return
    if wave6.get("default_state") != "disabled":
        failures.append("wave6 frontier policy must remain disabled by default")
    for feature in wave6.get("features", []):
        if feature.get("category") == "local_intelligence" and feature.get("default_enabled") is not False:
            failures.append("wave6 local intelligence must remain disabled by default")


def require_docs() -> None:
    for rel, tokens in DOC_TOKENS.items():
        text = read_text(rel)
        if not text:
            continue
        for token in tokens:
            if token not in text:
                failures.append(f"{rel} missing token: {token}")


def require_ci_wiring() -> None:
    frontier = read_text(".github/workflows/05-frontier-nightly.yml")
    pr_contracts = read_text(".github/workflows/00-pr-contracts.yml")
    matrix_text = read_text("contracts/ci/contract-test-matrix.yaml")
    if "python3 scripts/frontier_intelligence_governance_check.py" not in frontier:
        failures.append("frontier nightly must run frontier_intelligence_governance_check.py")
    if "python3 scripts/frontier_intelligence_governance_check.py" not in pr_contracts:
        failures.append("PR contracts must run frontier_intelligence_governance_check.py")
    if matrix_text:
        matrix = yaml.safe_load(matrix_text)
        entry = (matrix or {}).get("matrix", {}).get("frontier_intelligence_governance")
        if not isinstance(entry, dict):
            failures.append("contract test matrix missing frontier_intelligence_governance")
        else:
            if entry.get("gate") != "advisory_frontier":
                failures.append("frontier_intelligence_governance must remain advisory_frontier")
            if entry.get("blocks_pr") is not False:
                failures.append("frontier_intelligence_governance must not block PR promotion as a feature gate")
            if entry.get("promotion_requires_adr") is not True:
                failures.append("frontier_intelligence_governance promotion must require ADR")


def require_core_clean() -> None:
    core_toml = read_text("crates/fe_reader_core/Cargo.toml").lower()
    for token in FORBIDDEN_CORE_TOKENS:
        if token in core_toml:
            failures.append(f"fe_reader_core must not depend on frontier intelligence token: {token}")


def main() -> int:
    snapshot = load_json(SNAPSHOT)
    manifest = load_json(EVAL_MANIFEST)
    require_snapshot(snapshot)
    require_eval_manifest(manifest)
    require_wave6_alignment()
    require_docs()
    require_ci_wiring()
    require_core_clean()

    report = {
        "check": "frontier_intelligence_governance",
        "status": "fail" if failures else "pass",
        "feature_gate": snapshot.get("feature_gate"),
        "default_state": snapshot.get("default_state"),
        "launch_blocking": snapshot.get("launch_blocking"),
        "eval_manifest": str(EVAL_MANIFEST.relative_to(ROOT)),
        "failures": failures,
    }
    (EVIDENCE_DIR / "frontier-intelligence-governance.json").write_text(
        json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
    )

    if failures:
        print("FRONTIER INTELLIGENCE GOVERNANCE CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1

    print("frontier intelligence governance: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
