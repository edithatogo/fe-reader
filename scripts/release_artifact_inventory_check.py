#!/usr/bin/env python3
"""Inventory release artifacts and checksum files.

The check is advisory for dev/nightly/preview lanes where desktop publication is
still blocked, and fail-closed for stable/store lanes.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "release-artifact-inventory.json"
PACKAGE_MATRIX = ROOT / "packaging" / "package-matrix.yaml"
DESKTOP_DISTRIBUTION = ROOT / "packaging" / "desktop-distribution.yaml"
STRICT_CHANNELS = {"stable", "lts", "store_submission"}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_yaml(path: Path) -> dict:
    value = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"{path.relative_to(ROOT)} must contain a mapping")
    return value


def expected_artifacts(matrix: dict, version: str) -> list[dict]:
    desktop_artifacts = matrix.get("desktop_artifacts")
    if not isinstance(desktop_artifacts, dict):
        raise SystemExit("packaging/package-matrix.yaml missing desktop_artifacts")

    expected: list[dict] = []
    for platform in ("macos", "windows", "linux"):
        entries = desktop_artifacts.get(platform)
        if not isinstance(entries, list):
            raise SystemExit(f"desktop_artifacts.{platform} must be a list")
        for entry in entries:
            if not isinstance(entry, dict):
                raise SystemExit(f"desktop_artifacts.{platform} entries must be mappings")
            kind = entry.get("kind")
            path_pattern = entry.get("path_pattern")
            checksum_pattern = entry.get("checksum")
            if not isinstance(kind, str) or not isinstance(path_pattern, str) or not isinstance(checksum_pattern, str):
                raise SystemExit(f"desktop_artifacts.{platform} entry missing kind/path/checksum")
            expected.append(
                {
                    "platform": platform,
                    "kind": kind,
                    "path": path_pattern.format(version=version),
                    "checksum": checksum_pattern.format(version=version),
                    "signing": entry.get("signing"),
                    "notarization": entry.get("notarization"),
                    "registries": entry.get("registries", []),
                }
            )
    return expected


def checksum_file_value(path: Path) -> str:
    text = path.read_text(encoding="utf-8", errors="replace").strip()
    if not text:
        return ""
    return text.split()[0]


def main() -> int:
    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
    strict = channel in STRICT_CHANNELS
    distribution = load_yaml(DESKTOP_DISTRIBUTION)
    version = distribution.get("version")
    if not isinstance(version, str) or not version:
        raise SystemExit("packaging/desktop-distribution.yaml missing version")
    matrix = load_yaml(PACKAGE_MATRIX)

    artifacts: list[dict] = []
    failures: list[str] = []
    missing: list[str] = []
    invalid_checksums: list[str] = []

    for expected in expected_artifacts(matrix, version):
        artifact_path = ROOT / expected["path"]
        checksum_path = ROOT / expected["checksum"]
        artifact_exists = artifact_path.exists()
        checksum_exists = checksum_path.exists()
        actual_sha = sha256(artifact_path) if artifact_exists else None
        expected_sha = checksum_file_value(checksum_path) if checksum_exists else None
        checksum_matches = bool(actual_sha and expected_sha and actual_sha == expected_sha)

        if not artifact_exists:
            missing.append(expected["path"])
        if not checksum_exists:
            missing.append(expected["checksum"])
        if artifact_exists and checksum_exists and not checksum_matches:
            invalid_checksums.append(expected["checksum"])

        artifacts.append(
            {
                **expected,
                "exists": artifact_exists,
                "bytes": artifact_path.stat().st_size if artifact_exists else 0,
                "sha256": actual_sha,
                "checksum_exists": checksum_exists,
                "checksum_value": expected_sha,
                "checksum_matches": checksum_matches,
            }
        )

    if strict and missing:
        failures.append(f"missing release artifacts/checksums: {sorted(missing)}")
    if strict and invalid_checksums:
        failures.append(f"invalid release artifact checksums: {sorted(invalid_checksums)}")

    report = {
        "check": "release_artifact_inventory",
        "channel": channel,
        "version": version,
        "status": "fail" if failures else ("blocked" if missing or invalid_checksums else "pass"),
        "strict_channels": sorted(STRICT_CHANNELS),
        "artifact_count": len(artifacts),
        "present_artifact_count": sum(1 for artifact in artifacts if artifact["exists"]),
        "missing": sorted(missing),
        "invalid_checksums": sorted(invalid_checksums),
        "artifacts": artifacts,
        "failures": failures,
    }
    OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"release artifact inventory failure: {failure}")
        raise SystemExit(1)
    print(f"release artifact inventory: {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
