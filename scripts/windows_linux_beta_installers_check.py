#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "windows-linux-beta-installers.json"
FAILURES: list[str] = []


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        FAILURES.append(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def require_tokens(rel: str, tokens: list[str]) -> None:
    text = read_text(rel)
    for token in tokens:
        if token not in text:
            FAILURES.append(f"{rel} missing token: {token}")


def require_json(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        FAILURES.append(f"{rel} invalid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        FAILURES.append(f"{rel} must contain a mapping")
        return {}
    return data


require_tokens(
    "README.md",
    [
        "docs/windows-linux-beta-installers-contract.md",
        "scripts/windows_linux_beta_installers_check.py",
    ],
)
require_tokens(
    "docs/windows-linux-beta-installers-contract.md",
    [
        "Windows/Linux beta installers",
        "packaging/package-matrix.yaml",
        "target/release-artifacts/prerelease-placeholders/fe-reader-0.1.0-preview.1-release-artifact-inventory.json",
        "Windows beta installers",
        "Linux beta installers",
    ],
)
require_tokens(
    "docs/stable-desktop-release.md",
    [
        "docs/windows-linux-beta-installers-contract.md",
    ],
)
require_tokens(
    "docs/desktop-distribution-publication.md",
    [
        "docs/windows-linux-beta-installers-contract.md",
    ],
)
require_tokens(
    "docs-site/src/content/docs/stable-desktop-release.md",
    [
        "docs/windows-linux-beta-installers-contract.md",
    ],
)

release_matrix = require_json("target/release-evidence/release-matrix.json")
desktop_packaging = require_json("target/release-evidence/desktop-packaging-signing.json")
desktop_distribution = require_json("target/release-evidence/desktop-distribution-publication.json")
release_readiness = require_json("target/release-evidence/release-readiness.json")
placeholder_inventory = require_json(
    "target/release-artifacts/prerelease-placeholders/fe-reader-0.1.0-preview.1-release-artifact-inventory.json"
)

if release_matrix.get("status") != "pass":
    FAILURES.append("release matrix must pass for beta installer contract")
if desktop_packaging.get("status") != "pass":
    FAILURES.append("desktop packaging/signing must pass for beta installer contract")
if desktop_distribution.get("status") != "pass":
    FAILURES.append("desktop distribution publication must pass for beta installer contract")
if release_readiness.get("status") != "pass":
    FAILURES.append("release readiness must pass for beta installer contract")
if placeholder_inventory.get("artifact_count", 0) < 9:
    FAILURES.append("prerelease placeholder inventory is incomplete")

report = {
    "check": "windows_linux_beta_installers",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "packaging/package-matrix.yaml",
        "packaging/release-channels.yaml",
        "packaging/desktop-distribution.yaml",
        "target/release-artifacts/prerelease-placeholders/fe-reader-0.1.0-preview.1-release-artifact-inventory.json",
        "target/release-evidence/release-matrix.json",
        "target/release-evidence/desktop-packaging-signing.json",
        "target/release-evidence/desktop-distribution-publication.json",
        "target/release-evidence/release-readiness.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("WINDOWS/LINUX BETA INSTALLERS CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("windows/linux beta installers: pass")
