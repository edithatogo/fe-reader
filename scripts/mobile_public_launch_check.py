#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "packaging/mobile-public-launch.json"
REGISTRY = ROOT / "packaging/registry-status.yaml"
DOC = ROOT / "docs/mobile-public-launch-readiness.md"
DOCS_SITE = ROOT / "docs-site/src/content/docs/mobile-public-launch.md"
WORKFLOW = ROOT / ".github/workflows/09-platform-tests.yml"
EVIDENCE_DIR = ROOT / "target/release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

REQUIRED_GUARDS = {
    "patch_plan_id",
    "document_hash_match",
    "policy_allow_rule",
    "approval_token",
    "audit_receipt",
}
REQUIRED_PLATFORMS = {"android", "ios"}
DOC_TOKENS = {
    "docs/mobile-public-launch-readiness.md": [
        "mobile_public_launch",
        "does not block desktop stable launch",
        "read-only or plan-only",
        "privacy declaration",
        "accessibility",
        "power",
        "rollback",
    ],
    "docs-site/src/content/docs/mobile-public-launch.md": [
        "Mobile Public Launch",
        "mobile_public_launch",
        "does not block desktop stable launch",
    ],
    "README.md": [
        "docs/mobile-public-launch-readiness.md",
        "mobile_public_launch",
    ],
    "packaging/android/play/play-publishing-checklist.md": [
        "signed AAB",
        "privacy declaration",
        "accessibility",
        "power",
        "Play Console",
    ],
    "packaging/ios/appstore/app-store-checklist.md": [
        "TestFlight",
        "privacy manifest",
        "accessibility",
        "power",
        "App Store Connect",
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
        failures.append(f"{path.relative_to(ROOT)} is not valid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        failures.append(f"{path.relative_to(ROOT)} must contain a JSON object")
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


def require_registry_state() -> None:
    registry = load_yaml(REGISTRY)
    registries = registry.get("registries")
    if not isinstance(registries, dict):
        failures.append("packaging/registry-status.yaml missing registries mapping")
        return
    expected = {
        "google_play": "checklist_ready_publish_deferred",
        "app_store": "checklist_ready_publish_deferred",
    }
    for registry_name, expected_status in expected.items():
        item = registries.get(registry_name)
        if not isinstance(item, dict):
            failures.append(f"registry status missing {registry_name}")
            continue
        if item.get("status") != expected_status:
            failures.append(
                f"{registry_name} must remain {expected_status} until mobile public evidence is complete"
            )
        if not item.get("blocker"):
            failures.append(f"{registry_name} must declare a publication blocker")


def require_matrix(matrix: dict) -> None:
    if matrix.get("feature_gate") != "mobile_public_launch":
        failures.append("mobile public launch matrix must use feature_gate mobile_public_launch")
    if matrix.get("desktop_launch_blocking") is not False:
        failures.append("mobile public launch must not block desktop stable launch")
    if matrix.get("status") != "deferred":
        failures.append("mobile public launch status must remain deferred until all store evidence exists")

    policy = matrix.get("policy")
    if not isinstance(policy, dict):
        failures.append("mobile public launch matrix missing policy mapping")
        policy = {}
    if policy.get("automation_mutation_policy") != "read_only_or_plan_only":
        failures.append("mobile automation must remain read_only_or_plan_only")
    if policy.get("supports_apply") is not False:
        failures.append("mobile public launch must not expose apply support")
    guards = set(policy.get("required_mutation_guards", []))
    missing_guards = sorted(REQUIRED_GUARDS - guards)
    if missing_guards:
        failures.append(f"mobile public launch missing mutation guards: {', '.join(missing_guards)}")

    rollback = matrix.get("rollback")
    if not isinstance(rollback, dict) or rollback.get("strategy") != "keep_mobile_packages_deferred":
        failures.append("mobile public launch must document keep_mobile_packages_deferred rollback")

    exit_criteria = matrix.get("exit_criteria")
    if not isinstance(exit_criteria, list) or len(exit_criteria) < 5:
        failures.append("mobile public launch must list actionable exit criteria")

    platforms = matrix.get("platforms")
    if not isinstance(platforms, dict):
        failures.append("mobile public launch matrix missing platforms mapping")
        return
    missing_platforms = sorted(REQUIRED_PLATFORMS - set(platforms))
    if missing_platforms:
        failures.append(f"mobile public launch missing platforms: {', '.join(missing_platforms)}")

    for platform_name in sorted(REQUIRED_PLATFORMS):
        platform = platforms.get(platform_name)
        if not isinstance(platform, dict):
            failures.append(f"{platform_name} platform entry must be a mapping")
            continue
        if platform.get("status") != "deferred":
            failures.append(f"{platform_name} must remain deferred until artifacts and store evidence exist")
        for key in ("channels", "required_artifacts", "required_evidence", "blockers"):
            value = platform.get(key)
            if not isinstance(value, list) or not value:
                failures.append(f"{platform_name} must list {key}")
        for evidence in platform.get("required_evidence", []):
            if not (ROOT / evidence).exists():
                failures.append(f"{platform_name} evidence path missing: {evidence}")
        blockers = " ".join(platform.get("blockers", [])).lower()
        for token in ("privacy", "sign", "evidence"):
            if token not in blockers:
                failures.append(f"{platform_name} blockers must mention {token}")


def require_docs() -> None:
    for rel, tokens in DOC_TOKENS.items():
        text = read_text(rel)
        if not text:
            continue
        for token in tokens:
            if token not in text:
                failures.append(f"{rel} missing token: {token}")


def require_workflow() -> None:
    workflow = read_text(".github/workflows/09-platform-tests.yml")
    if not workflow:
        return
    for token in (
        "permissions:",
        "contents: read",
        "concurrency:",
        "timeout-minutes:",
        "android-emulator:",
        "ios-simulator:",
        "scripts/android_emulator_smoke.sh",
        "scripts/mobile_public_launch_check.py",
    ):
        if token not in workflow:
            failures.append(f"platform workflow missing token: {token}")


def main() -> int:
    matrix = load_json(MATRIX)
    require_matrix(matrix)
    require_registry_state()
    require_docs()
    require_workflow()

    report = {
        "check": "mobile_public_launch",
        "status": "fail" if failures else "pass",
        "feature_gate": matrix.get("feature_gate"),
        "desktop_launch_blocking": matrix.get("desktop_launch_blocking"),
        "publication_status": matrix.get("status"),
        "platforms": sorted((matrix.get("platforms") or {}).keys()),
        "failures": failures,
    }
    (EVIDENCE_DIR / "mobile-public-launch.json").write_text(
        json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
    )

    if failures:
        print("MOBILE PUBLIC LAUNCH CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1

    print("mobile public launch: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
