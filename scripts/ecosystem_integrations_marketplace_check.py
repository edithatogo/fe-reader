#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT = ROOT / "contracts/snapshots/ecosystem/ecosystem-integrations-marketplace.preview.json"
MARKETPLACE = ROOT / "fixtures/ecosystem/marketplace/metadata.preview.json"
EVIDENCE_DIR = ROOT / "target/release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

REQUIRED_PUBLICATION = {
    "signed_artifacts",
    "sbom",
    "provenance",
    "compatibility_report",
    "security_policy_link",
    "support_policy_link",
    "maintainer_approval",
    "rollback_plan",
}
REQUIRED_CHECKS = {
    "plugin_contract_smoke",
    "workflow_pack_smoke",
    "public_api_snapshot_check",
    "c_abi_snapshot_check",
    "nuget_wrapper_check",
    "strict_mutation_contract_check",
}
DOC_TOKENS = {
    "docs/ecosystem-integrations-marketplace.md": [
        "ecosystem_integrations_marketplace",
        "disabled by default",
        "does not block desktop stable launch",
        "compatibility snapshots",
        "maintainer approval",
        "read-only or plan-only",
        "Rollback disables plugin/runtime publication",
    ],
    "docs/developer-ecosystem-sdk.md": [
        "ecosystem_integrations_marketplace",
        "compatibility snapshots",
    ],
    "docs/external-application-integrations.md": [
        "ecosystem_integrations_marketplace",
        "read-only or plan-only",
    ],
    "README.md": [
        "docs/ecosystem-integrations-marketplace.md",
        "ecosystem_integrations_marketplace",
    ],
    "docs-site/src/content/docs/ecosystem-integrations-marketplace.md": [
        "Ecosystem Integrations and Marketplace",
        "ecosystem_integrations_marketplace",
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
    expected = {
        "contract": "ecosystem_integrations_marketplace",
        "stability": "preview",
        "feature_gate": "ecosystem_integrations_marketplace",
        "owner": "ecosystem-maintainers",
        "default_state": "disabled",
        "publication_default": "deferred",
        "mutation_policy": "read_only_or_plan_only_by_default",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            failures.append(f"ecosystem {key} must be {value}")
    if snapshot.get("launch_blocking") is not False:
        failures.append("ecosystem marketplace must not block desktop stable launch")
    if snapshot.get("unsafe_plugin_runtime_allowed") is not False:
        failures.append("unsafe plugin runtime must remain disallowed")
    missing_publication = sorted(REQUIRED_PUBLICATION - set(snapshot.get("publication_requires", [])))
    if missing_publication:
        failures.append(f"missing publication requirements: {', '.join(missing_publication)}")
    missing_checks = sorted(REQUIRED_CHECKS - set(snapshot.get("required_checks", [])))
    if missing_checks:
        failures.append(f"missing required checks: {', '.join(missing_checks)}")
    snapshots = snapshot.get("required_compatibility_snapshots", [])
    if not isinstance(snapshots, list) or len(snapshots) < 6:
        failures.append("required_compatibility_snapshots must cover public API surfaces")
    for rel in snapshots:
        if not isinstance(rel, str) or not (ROOT / rel).exists():
            failures.append(f"compatibility snapshot missing: {rel}")
    for surface in snapshot.get("marketplace_surfaces", []):
        if surface.get("status") not in {"deferred", "metadata_preview"}:
            failures.append(f"{surface.get('surface')} status must remain deferred or metadata_preview")
        if "mutation_policy" not in surface:
            failures.append(f"{surface.get('surface')} missing mutation_policy")
    rollback = snapshot.get("rollback", {})
    if rollback.get("strategy") != "disable_publication_and_remove_marketplace_claims":
        failures.append("rollback strategy must disable publication and remove marketplace claims")


def require_marketplace_metadata(metadata: dict) -> None:
    if metadata.get("feature_gate") != "ecosystem_integrations_marketplace":
        failures.append("marketplace metadata feature gate mismatch")
    if metadata.get("publication_status") != "deferred":
        failures.append("marketplace publication must remain deferred")
    if metadata.get("support") != "SUPPORT.md":
        failures.append("marketplace metadata must link SUPPORT.md")
    if metadata.get("security") != "SECURITY.md":
        failures.append("marketplace metadata must link SECURITY.md")
    if metadata.get("automation_safety") != "read_only_or_plan_only_by_default":
        failures.append("marketplace metadata must preserve automation safety")
    evidence = set(metadata.get("required_evidence", []))
    for token in ("signed_artifacts", "sbom", "provenance", "compatibility_report", "maintainer_approval"):
        if token not in evidence:
            failures.append(f"marketplace metadata missing evidence: {token}")


def require_docs() -> None:
    for rel, tokens in DOC_TOKENS.items():
        text = read_text(rel)
        for token in tokens:
            if token not in text:
                failures.append(f"{rel} missing token: {token}")


def require_supporting_contracts() -> None:
    plugin = load_json(ROOT / "contracts/snapshots/plugin-abi/fe_reader_plugin_host.preview.json")
    if plugin.get("abi") != "proposal_only":
        failures.append("plugin ABI must remain proposal_only")
    if plugin.get("mutation_policy") != "plugins_propose_patch_plans_only":
        failures.append("plugin mutation policy changed")
    mcp = load_json(ROOT / "contracts/snapshots/mcp/fe_reader_mcp.tools.preview.json")
    if mcp.get("mutation_policy") != "read_only_or_plan_only":
        failures.append("MCP tools must remain read_only_or_plan_only")
    for rel in ("SUPPORT.md", "SECURITY.md", "schemas/plugin-manifest.schema.json", "schemas/workflow-pack.schema.json"):
        if not (ROOT / rel).exists():
            failures.append(f"missing ecosystem support file: {rel}")


def require_ci_wiring() -> None:
    command = "python3 scripts/ecosystem_integrations_marketplace_check.py"
    pr_contracts = read_text(".github/workflows/00-pr-contracts.yml")
    phase_gate = read_text("scripts/conductor_phase_gate.sh")
    matrix_text = read_text("contracts/ci/contract-test-matrix.yaml")
    if command not in pr_contracts:
        failures.append("PR contracts must run ecosystem_integrations_marketplace_check.py")
    if command not in phase_gate:
        failures.append("Conductor phase gate must run ecosystem_integrations_marketplace_check.py")
    if matrix_text:
        matrix = yaml.safe_load(matrix_text)
        entry = (matrix or {}).get("matrix", {}).get("ecosystem_integrations_marketplace")
        if not isinstance(entry, dict):
            failures.append("contract test matrix missing ecosystem_integrations_marketplace")
        else:
            if entry.get("gate") != "advisory_post_launch":
                failures.append("ecosystem_integrations_marketplace must remain advisory_post_launch")
            if entry.get("blocks_pr") is not False:
                failures.append("ecosystem_integrations_marketplace must not block PR promotion as a feature gate")
            if entry.get("promotion_requires_maintainer_approval") is not True:
                failures.append("ecosystem promotion must require maintainer approval")


def require_docs_nav() -> None:
    config = read_text("docs-site/astro.config.mjs")
    if "ecosystem-integrations-marketplace" not in config:
        failures.append("docs site sidebar must include ecosystem-integrations-marketplace")


def main() -> int:
    snapshot = load_json(SNAPSHOT)
    metadata = load_json(MARKETPLACE)
    require_snapshot(snapshot)
    require_marketplace_metadata(metadata)
    require_docs()
    require_supporting_contracts()
    require_ci_wiring()
    require_docs_nav()
    report = {
        "check": "ecosystem_integrations_marketplace",
        "status": "fail" if failures else "pass",
        "feature_gate": snapshot.get("feature_gate"),
        "publication_default": snapshot.get("publication_default"),
        "launch_blocking": snapshot.get("launch_blocking"),
        "failures": failures,
    }
    (EVIDENCE_DIR / "ecosystem-integrations-marketplace.json").write_text(
        json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
    )
    if failures:
        print("ECOSYSTEM INTEGRATIONS MARKETPLACE CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1
    print("ecosystem integrations marketplace: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
