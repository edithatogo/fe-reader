#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT = ROOT / "contracts/snapshots/collaboration/opt-in-collaboration-sync.preview.json"
PROVIDER_FIXTURE = ROOT / "fixtures/collaboration/sync/provider-capabilities.preview.json"
EVIDENCE_DIR = ROOT / "target/release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

REQUIRED_CONTROLS = {
    "explicit_opt_in_ui",
    "explicit_opt_in_cli",
    "provider_disable_switch",
    "forget_workspace",
    "purge_sync_metadata",
    "export_local_review_packet",
    "revoke_provider_token",
}
REQUIRED_GUARDS = {
    "document_hash_match",
    "patch_plan_id",
    "policy_evaluation",
    "approval_token_or_interactive_confirmation",
    "audit_receipt",
}
REQUIRED_SUPPORT_EXCLUSIONS = {
    "document_bytes",
    "document_text",
    "review_packet_bodies",
    "cache_entry_payloads",
    "provider_tokens",
    "provider_account_ids",
    "sync_remote_paths",
}
REQUIRED_PROMOTION = {
    "privacy_review",
    "security_review",
    "provider_capability_report",
    "support_bundle_redaction_test",
    "explicit_opt_in_ui_cli",
    "offline_auth_failure_tests",
    "rollback_plan",
    "adr_approval_for_network_providers",
}
DOC_TOKENS = {
    "docs/opt-in-collaboration-sync.md": [
        "opt_in_collaboration_sync",
        "disabled by default",
        "does not block desktop stable launch",
        "silent upload",
        "capability-discovered",
        "support bundles exclude",
        "Rollback disables sync providers",
    ],
    "docs/collaboration-local-first.md": [
        "opt_in_collaboration_sync",
        "disabled by default",
        "must not silently upload",
    ],
    "docs/offline-collaboration-review-packets.md": [
        "opt_in_collaboration_sync",
        "disabled by default",
        "provider tokens",
    ],
    "docs/launch-limitations-support.md": [
        "opt_in_collaboration_sync",
        "cloud collaboration is deferred",
    ],
    "README.md": [
        "docs/opt-in-collaboration-sync.md",
        "opt_in_collaboration_sync",
    ],
    "docs-site/src/content/docs/opt-in-collaboration-sync.md": [
        "Opt-in Collaboration and Sync",
        "opt_in_collaboration_sync",
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
        "contract": "opt_in_collaboration_sync",
        "stability": "preview",
        "feature_gate": "opt_in_collaboration_sync",
        "owner": "collaboration-maintainers",
        "default_state": "disabled",
        "network_default": "disabled",
        "telemetry_default": "disabled",
        "analytics_default": "disabled",
        "mutation_policy": "plan_only_until_user_approval",
        "privacy_policy": "privacy_sensitive_packets_caches_quality_signals",
    }
    for key, expected in expected_scalars.items():
        if snapshot.get(key) != expected:
            failures.append(f"collaboration sync {key} must be {expected}")
    if snapshot.get("launch_blocking") is not False:
        failures.append("opt-in collaboration sync must not block desktop stable launch")
    if snapshot.get("local_only_default") is not True:
        failures.append("local-only behavior must remain the default")
    if snapshot.get("silent_upload_allowed") is not False:
        failures.append("silent upload must be forbidden")
    if snapshot.get("sync_without_user_action_allowed") is not False:
        failures.append("sync without user action must be forbidden")

    controls = set(snapshot.get("required_user_controls", []))
    missing_controls = sorted(REQUIRED_CONTROLS - controls)
    if missing_controls:
        failures.append(f"missing user controls: {', '.join(missing_controls)}")

    guards = set(snapshot.get("required_mutation_guards", []))
    missing_guards = sorted(REQUIRED_GUARDS - guards)
    if missing_guards:
        failures.append(f"missing mutation guards: {', '.join(missing_guards)}")

    promotion = set(snapshot.get("promotion_requires", []))
    missing_promotion = sorted(REQUIRED_PROMOTION - promotion)
    if missing_promotion:
        failures.append(f"missing promotion requirements: {', '.join(missing_promotion)}")

    provider_discovery = snapshot.get("provider_discovery")
    if not isinstance(provider_discovery, dict):
        failures.append("provider_discovery must be a mapping")
    else:
        required = {
            "capability_probe_required": True,
            "auth_required_state": "unavailable_until_user_connects",
            "offline_failure_mode": "local_only_queue_disabled_by_default",
            "auth_failure_mode": "read_only_provider_unavailable",
            "conflict_failure_mode": "manual_review_packet_merge",
            "default_upload_mode": "never",
        }
        for key, expected in required.items():
            if provider_discovery.get(key) != expected:
                failures.append(f"provider_discovery {key} must be {expected}")

    privacy = snapshot.get("privacy_boundaries")
    if not isinstance(privacy, dict):
        failures.append("privacy_boundaries must be a mapping")
    else:
        exclusions = set(privacy.get("support_bundles_exclude", []))
        missing_exclusions = sorted(REQUIRED_SUPPORT_EXCLUSIONS - exclusions)
        if missing_exclusions:
            failures.append(f"support bundle exclusions missing: {', '.join(missing_exclusions)}")
        for key in ("review_packets", "cache_entries", "quality_signals"):
            if "privacy_sensitive" not in str(privacy.get(key, "")):
                failures.append(f"{key} must be marked privacy_sensitive")

    providers = snapshot.get("supported_preview_providers")
    if not isinstance(providers, list) or not providers:
        failures.append("supported_preview_providers must be a non-empty list")
        return
    for provider in providers:
        if not isinstance(provider, dict):
            failures.append("provider entry must be an object")
            continue
        provider_id = provider.get("provider_id")
        if provider.get("default_enabled") is not False:
            failures.append(f"{provider_id} must be disabled by default")
        if provider.get("requires_explicit_opt_in") is not True:
            failures.append(f"{provider_id} must require explicit opt-in")
        if provider.get("requires_capability_discovery") is not True:
            failures.append(f"{provider_id} must require capability discovery")
        if provider.get("supports_upload") is not False:
            failures.append(f"{provider_id} must not support upload in this preview gate")
        if provider.get("supports_analytics") is not False:
            failures.append(f"{provider_id} must not support analytics")
        if provider.get("network_required") is True and provider.get("promotion_requires_adr") is not True:
            failures.append(f"{provider_id} network provider must require ADR before promotion")

    rollback = snapshot.get("rollback")
    if not isinstance(rollback, dict):
        failures.append("rollback must be a mapping")
    else:
        if rollback.get("strategy") != "disable_sync_providers_and_keep_local_workspaces":
            failures.append("rollback strategy must disable providers while keeping local workspaces")
        if "provider_tokens" not in set(rollback.get("purges", [])):
            failures.append("rollback must purge provider_tokens")


def require_provider_fixture(fixture: dict) -> None:
    if fixture.get("privacy_policy") != "synthetic_no_private_documents":
        failures.append("provider capability fixture must use synthetic_no_private_documents")
    exclusions = set(fixture.get("support_bundle_exclusions", []))
    missing_exclusions = sorted(REQUIRED_SUPPORT_EXCLUSIONS - exclusions)
    if missing_exclusions:
        failures.append(f"provider fixture missing support exclusions: {', '.join(missing_exclusions)}")
    providers = fixture.get("providers")
    if not isinstance(providers, list) or not providers:
        failures.append("provider fixture must list providers")
        return
    for provider in providers:
        if provider.get("default_enabled") is not False:
            failures.append(f"provider fixture {provider.get('provider_id')} must be disabled by default")
        failure_modes = set(provider.get("failure_modes", []))
        if not failure_modes:
            failures.append(f"provider fixture {provider.get('provider_id')} must list failure modes")
        if provider.get("network_required") is True and "auth_required" not in failure_modes:
            failures.append(f"network provider {provider.get('provider_id')} must include auth_required failure")
        if provider.get("network_required") is True and "offline" not in failure_modes:
            failures.append(f"network provider {provider.get('provider_id')} must include offline failure")


def require_docs() -> None:
    for rel, tokens in DOC_TOKENS.items():
        text = read_text(rel)
        if not text:
            continue
        for token in tokens:
            if token not in text:
                failures.append(f"{rel} missing token: {token}")


def require_schemas_and_contracts() -> None:
    required_paths = [
        "schemas/review-packet.schema.json",
        "schemas/cache-manifest.schema.json",
        "schemas/workspace-record.schema.json",
        "contracts/rust/offline_collaboration.rs",
        "contracts/rust/cache_workspace.rs",
    ]
    for rel in required_paths:
        if not (ROOT / rel).exists():
            failures.append(f"missing collaboration contract dependency: {rel}")
    offline = read_text("contracts/rust/offline_collaboration.rs")
    for token in ("ReviewPacket", "base_document_sha256", "ReviewPacketImportResult"):
        if token not in offline:
            failures.append(f"offline collaboration contract missing token: {token}")


def require_ci_wiring() -> None:
    pr_contracts = read_text(".github/workflows/00-pr-contracts.yml")
    phase_gate = read_text("scripts/conductor_phase_gate.sh")
    matrix_text = read_text("contracts/ci/contract-test-matrix.yaml")
    command = "python3 scripts/opt_in_collaboration_sync_check.py"
    if command not in pr_contracts:
        failures.append("PR contracts must run opt_in_collaboration_sync_check.py")
    if command not in phase_gate:
        failures.append("Conductor phase gate must run opt_in_collaboration_sync_check.py")
    if matrix_text:
        matrix = yaml.safe_load(matrix_text)
        entry = (matrix or {}).get("matrix", {}).get("opt_in_collaboration_sync")
        if not isinstance(entry, dict):
            failures.append("contract test matrix missing opt_in_collaboration_sync")
        else:
            if entry.get("gate") != "advisory_post_launch":
                failures.append("opt_in_collaboration_sync must remain advisory_post_launch")
            if entry.get("blocks_pr") is not False:
                failures.append("opt_in_collaboration_sync must not block PRs as a feature promotion gate")
            if entry.get("promotion_requires_adr") is not True:
                failures.append("network sync promotion must require ADR")


def require_docs_nav() -> None:
    config = read_text("docs-site/astro.config.mjs")
    if "opt-in-collaboration-sync" not in config:
        failures.append("docs site sidebar must include opt-in-collaboration-sync")


def main() -> int:
    snapshot = load_json(SNAPSHOT)
    fixture = load_json(PROVIDER_FIXTURE)
    require_snapshot(snapshot)
    require_provider_fixture(fixture)
    require_docs()
    require_schemas_and_contracts()
    require_ci_wiring()
    require_docs_nav()

    report = {
        "check": "opt_in_collaboration_sync",
        "status": "fail" if failures else "pass",
        "feature_gate": snapshot.get("feature_gate"),
        "launch_blocking": snapshot.get("launch_blocking"),
        "local_only_default": snapshot.get("local_only_default"),
        "providers": [
            provider.get("provider_id")
            for provider in snapshot.get("supported_preview_providers", [])
            if isinstance(provider, dict)
        ],
        "failures": failures,
    }
    (EVIDENCE_DIR / "opt-in-collaboration-sync.json").write_text(
        json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
    )

    if failures:
        print("OPT-IN COLLABORATION SYNC CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1
    print("opt-in collaboration sync: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
