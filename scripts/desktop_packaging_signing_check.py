#!/usr/bin/env python3
"""Validate desktop stable packaging and signing definitions.

Writes target/release-evidence/desktop-packaging-signing.json.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
MATRIX_PATH = ROOT / "packaging" / "package-matrix.yaml"
REGISTRY_PATH = ROOT / "packaging" / "registry-status.yaml"
EVIDENCE_PATH = EVIDENCE_DIR / "desktop-packaging-signing.json"

REQUIRED_ARTIFACT_KINDS = {
    "macos": {"dmg"},
    "windows": {"msi", "msix"},
    "linux": {"appimage", "deb", "rpm", "tarball"},
}
REQUIRED_FIELDS = {"kind", "path_pattern", "checksum", "signing", "notarization", "registries"}
STRICT_CHANNELS = {"stable", "lts"}
DESKTOP_PLATFORMS = ("macos", "windows", "linux")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_yaml(path: Path) -> dict:
    value = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"{path.relative_to(ROOT)} must contain a mapping")
    return value


def validate_artifacts(matrix: dict) -> list[dict]:
    desktop_artifacts = matrix.get("desktop_artifacts")
    if not isinstance(desktop_artifacts, dict):
        raise SystemExit("package matrix missing desktop_artifacts mapping")

    entries: list[dict] = []
    for platform, required_kinds in REQUIRED_ARTIFACT_KINDS.items():
        platform_entries = desktop_artifacts.get(platform)
        if not isinstance(platform_entries, list) or not platform_entries:
            raise SystemExit(f"desktop_artifacts.{platform} must contain artifact definitions")
        seen_kinds = set()
        for entry in platform_entries:
            if not isinstance(entry, dict):
                raise SystemExit(f"desktop_artifacts.{platform} entries must be mappings")
            missing = REQUIRED_FIELDS - set(entry)
            if missing:
                raise SystemExit(
                    f"desktop_artifacts.{platform}.{entry.get('kind', '<unknown>')} missing {sorted(missing)}"
                )
            if not isinstance(entry["registries"], list) or not entry["registries"]:
                raise SystemExit(f"desktop_artifacts.{platform}.{entry['kind']} must list registries")
            if "{version}" not in entry["path_pattern"] or "{version}" not in entry["checksum"]:
                raise SystemExit(f"desktop_artifacts.{platform}.{entry['kind']} must include version placeholder")
            seen_kinds.add(entry["kind"])
            entries.append({"platform": platform, **entry})
        missing_kinds = sorted(required_kinds - seen_kinds)
        if missing_kinds:
            raise SystemExit(f"desktop_artifacts.{platform} missing required kinds: {missing_kinds}")
    return entries


def validate_registry(registry: dict) -> list[dict]:
    registries = registry.get("registries")
    if not isinstance(registries, dict):
        raise SystemExit("registry status missing registries mapping")
    required = ("github_releases", "homebrew", "winget", "chocolatey", "scoop", "flatpak", "snap", "aur")
    summaries = []
    for name in required:
        entry = registries.get(name)
        if not isinstance(entry, dict):
            raise SystemExit(f"registry status missing desktop registry: {name}")
        status = str(entry.get("status", ""))
        blocker = str(entry.get("blocker", ""))
        if not status:
            raise SystemExit(f"registry status missing status for {name}")
        if "deferred" in status and not blocker:
            raise SystemExit(f"registry status deferred without blocker for {name}")
        summaries.append({"name": name, "status": status, "blocker": blocker})
    return summaries


def main() -> int:
    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
    matrix = load_yaml(MATRIX_PATH)
    registry = load_yaml(REGISTRY_PATH)
    artifacts = validate_artifacts(matrix)
    registry_status = validate_registry(registry)

    status = "pass"
    detail = "Desktop packaging and signing definitions are complete"
    if channel in STRICT_CHANNELS:
        deferred = [entry for entry in registry_status if "deferred" in entry["status"]]
        if deferred:
            status = "blocked"
            detail = "Stable desktop release still has deferred desktop registry publication surfaces"

    report = {
        "check": "desktop_packaging_signing",
        "channel": channel,
        "status": status,
        "detail": detail,
        "platforms": list(DESKTOP_PLATFORMS),
        "required_artifact_kinds": {k: sorted(v) for k, v in REQUIRED_ARTIFACT_KINDS.items()},
        "artifacts": artifacts,
        "registry_status": registry_status,
        "required_files": [
            {
                "path": "packaging/package-matrix.yaml",
                "sha256": digest(MATRIX_PATH),
                "bytes": MATRIX_PATH.stat().st_size,
            },
            {
                "path": "packaging/registry-status.yaml",
                "sha256": digest(REGISTRY_PATH),
                "bytes": REGISTRY_PATH.stat().st_size,
            },
        ],
    }
    EVIDENCE_PATH.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if status == "blocked":
        raise SystemExit(detail)
    print(f"desktop packaging/signing: {status}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
