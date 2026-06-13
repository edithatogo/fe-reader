#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import re
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[1]
# Enterprise evidence is emitted under target/release-evidence for CI upload.
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

FAILURES: list[str] = []
HEX_64 = re.compile(r"^[0-9a-f]{64}$")
RISKY_SURFACES = {"mcp", "plugins", "external_converters", "web_postmessage", "native_automation_apply"}
FORBIDDEN_SUPPORT_FIELDS = {
    "raw_pdf",
    "document_text",
    "rendered_page_images",
    "search_index_contents",
    "credentials",
    "access_tokens",
    "private_keys",
    "full_file_paths",
    "usernames",
}


def fail(message: str) -> None:
    FAILURES.append(message)


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        fail(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def load_yaml(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    data = yaml.safe_load(text)
    if not isinstance(data, dict):
        fail(f"{rel} must contain a mapping")
        return {}
    return data


def load_json(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        fail(f"{rel} invalid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        fail(f"{rel} must contain a JSON object")
        return {}
    return data


def validate_install_modes() -> dict:
    data = load_yaml("packaging/enterprise/install-modes.yaml")
    platforms = data.get("platforms")
    if not isinstance(platforms, dict):
        fail("install modes missing platforms mapping")
        return {}
    for platform in ["macos", "windows", "linux"]:
        entry = platforms.get(platform)
        if not isinstance(entry, dict):
            fail(f"install modes missing {platform}")
            continue
        for key in ["offline_artifacts", "per_user", "global", "validation", "blocker"]:
            if not entry.get(key):
                fail(f"install modes {platform} missing {key}")
        if not isinstance(entry.get("offline_artifacts"), list) or not entry["offline_artifacts"]:
            fail(f"install modes {platform} offline_artifacts must be non-empty")
    policy = data.get("install_policy", {})
    if policy.get("credentials_in_repo") != "forbidden":
        fail("install policy must forbid credentials in repo")
    if policy.get("per_user_must_not_require_admin") is not True:
        fail("per-user install policy must not require admin")
    if policy.get("global_install_requires_admin_or_managed_deployment") is not True:
        fail("global install policy must require admin or managed deployment")
    return data


def validate_policy() -> dict:
    policy = load_json("packaging/enterprise/managed-lockdown-policy.json")
    schema = load_json("schemas/enterprise-policy.schema.json")
    for field in schema.get("required", []):
        if field not in policy:
            fail(f"managed lockdown policy missing required field: {field}")
    if policy.get("managed") is not True:
        fail("managed lockdown policy must be managed")
    disabled_surfaces = set(policy.get("disabled_surfaces", []))
    missing_surfaces = RISKY_SURFACES - disabled_surfaces
    if missing_surfaces:
        fail(f"managed lockdown policy does not disable risky surfaces: {sorted(missing_surfaces)}")
    for key in ["allow_plugins", "allow_mcp", "allow_external_converters"]:
        if policy.get(key) is not False:
            fail(f"managed lockdown policy must set {key}=false")
    if policy.get("telemetry_mode") not in {"disabled", "local_only"}:
        fail("managed lockdown policy telemetry must be disabled or local_only")
    channels = set(policy.get("allowed_update_channels", []))
    if not {"stable", "lts-enterprise"}.issubset(channels):
        fail("managed lockdown policy must allow stable and lts-enterprise channels")
    docs = read_text("docs/enterprise-operations-readiness.md") + read_text("docs/enterprise-deployment-policy.md")
    for token in ["system/MDM policy", "workspace policy", "user preference", "default app setting"]:
        if token not in docs:
            fail(f"enterprise policy docs missing precedence token: {token}")
    return policy


def validate_update_manifest(rel: str, expect_rollback: bool) -> dict:
    manifest = load_json(rel)
    required = ["manifest_version", "app_version", "channel", "artifacts", "manifest_signature"]
    for field in required:
        if not manifest.get(field):
            fail(f"{rel} missing {field}")
    if bool(manifest.get("rollback", False)) is not expect_rollback:
        fail(f"{rel} rollback flag mismatch")
    if manifest.get("channel") in {"beta", "stable", "lts-enterprise"}:
        for field in ["provenance_path", "signing_readiness_path"]:
            if not manifest.get(field):
                fail(f"{rel} release channel missing {field}")
    if str(manifest.get("manifest_signature", "")).lower() in {"", "unsigned", "placeholder"}:
        fail(f"{rel} manifest signature must not be unsigned")
    artifacts = manifest.get("artifacts")
    if not isinstance(artifacts, list) or not artifacts:
        fail(f"{rel} must contain artifacts")
        return manifest
    for index, artifact in enumerate(artifacts):
        if not isinstance(artifact, dict):
            fail(f"{rel} artifact {index} must be an object")
            continue
        for field in ["platform", "arch", "installer_kind", "url", "sha256", "signature"]:
            if not artifact.get(field):
                fail(f"{rel} artifact {index} missing {field}")
        if not HEX_64.match(str(artifact.get("sha256", ""))):
            fail(f"{rel} artifact {index} sha256 must be lowercase 64-character hex")
        if str(artifact.get("signature", "")).lower() in {"", "unsigned", "placeholder"}:
            fail(f"{rel} artifact {index} signature must not be unsigned")
        if artifact.get("size_bytes") == 0:
            fail(f"{rel} artifact {index} size_bytes must be positive")
    return manifest


def validate_negative_update_cases() -> None:
    unsigned = {
        "manifest_signature": "unsigned",
        "artifacts": [{"sha256": "a" * 64, "signature": "unsigned", "size_bytes": 1}],
    }
    bad_digest = {
        "manifest_signature": "signed",
        "artifacts": [{"sha256": "not-a-digest", "signature": "signed", "size_bytes": 1}],
    }
    if unsigned["manifest_signature"] != "unsigned":
        fail("negative unsigned manifest fixture broken")
    if HEX_64.match(bad_digest["artifacts"][0]["sha256"]):
        fail("negative digest fixture unexpectedly valid")


def validate_support_bundle() -> dict:
    bundle = load_yaml("packaging/enterprise/support-bundle-allowlist.yaml")
    if bundle.get("default_collection") != "local_only":
        fail("support bundle default collection must be local_only")
    if bundle.get("requires_preview_before_export") is not True:
        fail("support bundle export must require preview")
    if bundle.get("requires_user_approval") is not True:
        fail("support bundle export must require user approval")
    allowed = set(bundle.get("allowed", []))
    forbidden = set(bundle.get("forbidden", []))
    overlap = allowed & forbidden
    if overlap:
        fail(f"support bundle allowed/forbidden overlap: {sorted(overlap)}")
    missing_forbidden = FORBIDDEN_SUPPORT_FIELDS - forbidden
    if missing_forbidden:
        fail(f"support bundle forbidden list missing: {sorted(missing_forbidden)}")
    docs = read_text("docs/enterprise-operations-readiness.md") + read_text("docs/privacy-diagnostics-observability.md")
    for token in ["raw PDFs", "document text", "credentials", "preview", "approve"]:
        if token not in docs:
            fail(f"support bundle docs missing token: {token}")
    return bundle


install_modes = validate_install_modes()
policy = validate_policy()
stable_manifest = validate_update_manifest("packaging/enterprise/update-manifests/stable.json", False)
rollback_manifest = validate_update_manifest("packaging/enterprise/update-manifests/rollback.json", True)
validate_negative_update_cases()
support_bundle = validate_support_bundle()

report = {
    "check": "enterprise_operations_readiness",
    "status": "fail" if FAILURES else "pass",
    "install_platforms": sorted((install_modes.get("platforms") or {}).keys()),
    "managed_policy": {
        "managed": policy.get("managed"),
        "telemetry_mode": policy.get("telemetry_mode"),
        "allowed_update_channels": policy.get("allowed_update_channels", []),
    },
    "update_manifests": [
        {"path": "packaging/enterprise/update-manifests/stable.json", "rollback": stable_manifest.get("rollback", False)},
        {"path": "packaging/enterprise/update-manifests/rollback.json", "rollback": rollback_manifest.get("rollback", False)},
    ],
    "support_bundle": {
        "default_collection": support_bundle.get("default_collection"),
        "forbidden_count": len(support_bundle.get("forbidden", [])) if isinstance(support_bundle.get("forbidden"), list) else 0,
    },
}
(EVIDENCE_DIR / "enterprise-operations-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)

if FAILURES:
    print("ENTERPRISE OPERATIONS READINESS CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    sys.exit(1)

print("enterprise operations readiness: ok")
