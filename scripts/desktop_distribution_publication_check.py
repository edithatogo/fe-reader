#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import pathlib
import re
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[1]
# Release evidence is emitted under target/release-evidence for CI upload.
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)

PUBLIC_CHANNELS = {"preview", "beta", "stable", "lts", "store_submission"}
STRICT_CHANNELS = {"stable", "lts", "store_submission"}
DESKTOP_SURFACES = {
    "github_releases",
    "homebrew",
    "winget",
    "chocolatey",
    "scoop",
    "flatpak",
    "snap",
    "aur",
}
ALLOWED_STATUS_PREFIXES = ("ready_", "published_", "blocked_")
FORBIDDEN_SECRET_KEYS = {"token", "secret", "password", "credential", "api_key", "apikey"}


def fail(message: str) -> None:
    failures.append(message)


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


def contains_forbidden_secret_keys(value: object, trail: str) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            key_text = str(key).lower()
            if any(secret_key in key_text for secret_key in FORBIDDEN_SECRET_KEYS):
                fail(f"forbidden credential-like key in publication metadata: {trail}.{key}")
            contains_forbidden_secret_keys(child, f"{trail}.{key}")
    elif isinstance(value, list):
        for index, child in enumerate(value):
            contains_forbidden_secret_keys(child, f"{trail}[{index}]")


def artifact_index(package_matrix: dict, version: str) -> dict[str, dict]:
    artifacts: dict[str, dict] = {}
    desktop = package_matrix.get("desktop_artifacts")
    if not isinstance(desktop, dict):
        fail("package matrix missing desktop_artifacts mapping")
        return artifacts
    for platform, entries in desktop.items():
        if not isinstance(entries, list):
            fail(f"desktop_artifacts.{platform} must be a list")
            continue
        for entry in entries:
            if not isinstance(entry, dict):
                fail(f"desktop_artifacts.{platform} entry must be a mapping")
                continue
            kind = entry.get("kind")
            if not isinstance(kind, str):
                fail(f"desktop_artifacts.{platform} entry missing kind")
                continue
            path_pattern = entry.get("path_pattern")
            checksum = entry.get("checksum")
            if not isinstance(path_pattern, str) or not isinstance(checksum, str):
                fail(f"desktop artifact {platform}/{kind} missing path/checksum patterns")
                continue
            artifacts[kind] = {
                "platform": platform,
                "path": path_pattern.format(version=version),
                "checksum": checksum.format(version=version),
                "registries": entry.get("registries", []),
            }
    return artifacts


def surface_status_is_blocked(status: str) -> bool:
    return status.startswith("blocked_")


failures: list[str] = []
release_channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
distribution = load_yaml("packaging/desktop-distribution.yaml")
registry_status = load_yaml("packaging/registry-status.yaml")
package_matrix = load_yaml("packaging/package-matrix.yaml")

version = distribution.get("version")
release_tag = distribution.get("release_tag")
if not isinstance(version, str) or not version:
    fail("desktop distribution metadata missing version")
if release_tag != f"v{version}":
    fail("desktop distribution release_tag must equal v{version}")

release_notes = distribution.get("release_notes")
if not isinstance(release_notes, str) or not release_notes:
    fail("desktop distribution metadata missing release_notes")
else:
    notes = read_text(release_notes)
    for token in ["Publication Policy", "signed artifacts", "maintainer approval", "GitHub Releases"]:
        if notes and token not in notes:
            fail(f"release notes missing token: {token}")

approval = distribution.get("approval")
if not isinstance(approval, dict):
    fail("desktop distribution approval block missing")
else:
    if approval.get("required") is not True:
        fail("desktop distribution approval.required must be true")
    for key in ["approver", "evidence", "policy"]:
        if not approval.get(key):
            fail(f"desktop distribution approval missing {key}")

contains_forbidden_secret_keys(distribution, "packaging.desktop-distribution")
contains_forbidden_secret_keys(registry_status, "packaging.registry-status")

surfaces = distribution.get("surfaces")
registries = registry_status.get("registries")
if not isinstance(surfaces, dict):
    fail("desktop distribution surfaces must be a mapping")
    surfaces = {}
if not isinstance(registries, dict):
    fail("registry status registries must be a mapping")
    registries = {}

missing_surfaces = DESKTOP_SURFACES - set(surfaces)
extra_surfaces = set(surfaces) - DESKTOP_SURFACES
if missing_surfaces:
    fail(f"desktop distribution missing surfaces: {sorted(missing_surfaces)}")
if extra_surfaces:
    fail(f"desktop distribution has unexpected surfaces: {sorted(extra_surfaces)}")

artifacts = artifact_index(package_matrix, version if isinstance(version, str) else "")
strict_publication = release_channel in STRICT_CHANNELS
surface_reports: list[dict] = []

for surface_name in sorted(DESKTOP_SURFACES):
    surface = surfaces.get(surface_name)
    registry = registries.get(surface_name)
    if not isinstance(surface, dict):
        fail(f"{surface_name} surface must be a mapping")
        continue
    if not isinstance(registry, dict):
        fail(f"registry-status missing desktop surface: {surface_name}")
        registry = {}
    status = surface.get("status")
    if not isinstance(status, str) or not status.startswith(ALLOWED_STATUS_PREFIXES):
        fail(f"{surface_name} status must start with ready_, published_ or blocked_")
        status = ""
    if registry.get("status") != status:
        fail(f"{surface_name} registry-status mismatch: {registry.get('status')!r} != {status!r}")
    blocker = surface.get("blocker")
    if surface_status_is_blocked(status) and not isinstance(blocker, str):
        fail(f"{surface_name} blocked status requires exact blocker text")
    if not surface_status_is_blocked(status) and not approval.get("required"):
        fail(f"{surface_name} non-blocked status requires maintainer approval gate")
    manifest = surface.get("manifest")
    manifest_text = ""
    if isinstance(manifest, str):
        manifest_text = read_text(manifest)
        if registry.get("manifest") != manifest:
            fail(f"{surface_name} registry manifest mismatch")
    elif surface_name != "github_releases":
        fail(f"{surface_name} missing manifest path")
    artifact_kind = surface.get("artifact")
    artifact_kinds = surface.get("artifacts")
    expected_kinds: list[str] = []
    if isinstance(artifact_kind, str):
        expected_kinds.append(artifact_kind)
    if isinstance(artifact_kinds, list):
        expected_kinds.extend(str(kind) for kind in artifact_kinds)
    for kind in expected_kinds:
        artifact = artifacts.get(kind)
        if not artifact:
            fail(f"{surface_name} references unknown artifact kind: {kind}")
            continue
        artifact_path = ROOT / artifact["path"]
        checksum_path = ROOT / artifact["checksum"]
        if status.startswith(("ready_", "published_")) or strict_publication:
            if not artifact_path.exists():
                fail(f"{surface_name} missing release artifact: {artifact['path']}")
            if not checksum_path.exists():
                fail(f"{surface_name} missing release checksum: {artifact['checksum']}")
        if manifest_text and status.startswith(("ready_", "published_")) and "PLACEHOLDER" in manifest_text:
            fail(f"{surface_name} manifest still contains PLACEHOLDER while marked {status}")
    surface_reports.append(
        {
            "surface": surface_name,
            "status": status,
            "manifest": manifest if isinstance(manifest, str) else None,
            "blocker": blocker if isinstance(blocker, str) else None,
        }
    )

readme = read_text("README.md")
if readme:
    for token in [
        "https://github.com/edithatogo/fe-reader/releases",
        "docs/desktop-distribution-publication.md",
        "packaging/registry-status.yaml",
    ]:
        if token not in readme:
            fail(f"README missing public distribution link/token: {token}")

docs_site = read_text("docs-site/src/content/docs/release-pipeline.md")
if docs_site:
    for token in [
        "Desktop distribution",
        "packaging/desktop-distribution.yaml",
        "scripts/desktop_distribution_publication_check.py",
    ]:
        if token not in docs_site:
            fail(f"docs-site release pipeline missing token: {token}")

repo_metadata = load_yaml(".github/repository-metadata.yaml")
if repo_metadata:
    for key in ["homepage", "repository", "releases", "documentation", "packages"]:
        if key not in repo_metadata:
            fail(f"repository metadata missing {key}")

for rel in [
    "docs/desktop-distribution-publication.md",
    "packaging/desktop-distribution.yaml",
    "packaging/registry-status.yaml",
]:
    if not (ROOT / rel).exists():
        fail(f"missing desktop publication support file: {rel}")

if release_channel in PUBLIC_CHANNELS:
    stable_evidence = EVIDENCE_DIR / "stable-release-evidence.json"
    if not stable_evidence.exists():
        fail("public release channel missing stable release evidence")

report = {
    "check": "desktop_distribution_publication",
    "status": "fail" if failures else "pass",
    "channel": release_channel,
    "version": version,
    "release_tag": release_tag,
    "release_notes": release_notes,
    "approval_required": bool(isinstance(approval, dict) and approval.get("required") is True),
    "surfaces": surface_reports,
    "strict_channels": sorted(STRICT_CHANNELS),
}
(EVIDENCE_DIR / "desktop-distribution-publication.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)

if failures:
    print("DESKTOP DISTRIBUTION PUBLICATION CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("desktop distribution publication: ok")
