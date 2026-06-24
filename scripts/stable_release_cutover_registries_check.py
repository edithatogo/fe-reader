#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

import yaml

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "stable-release-cutover-registries.json"
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


def require_yaml(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    try:
        data = yaml.safe_load(text)
    except yaml.YAMLError as exc:
        FAILURES.append(f"{rel} invalid YAML: {exc}")
        return {}
    if not isinstance(data, dict):
        FAILURES.append(f"{rel} must contain a mapping")
        return {}
    return data


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        FAILURES.append(f"command failed: {' '.join(command)}")


require_tokens(
    "README.md",
    [
        "docs/stable-release-cutover-registries-contract.md",
        "scripts/stable_release_cutover_registries_check.py",
    ],
)
require_tokens(
    "docs/stable-release-cutover-registries-contract.md",
    [
        "Stable Release Cutover and Registries Contract",
        "docs/stable-desktop-release.md",
        "docs/desktop-distribution-publication.md",
        "packaging/desktop-distribution.yaml",
        "packaging/registry-status.yaml",
    ],
)
for rel in [
    "docs/stable-desktop-release.md",
    "docs/desktop-distribution-publication.md",
    "docs/launch-limitations-support.md",
    "docs/release-notes/v0.1.0-preview.1.md",
]:
    require_tokens(rel, ["docs/stable-release-cutover-registries-contract.md"])

run(["python3", "scripts/stable_release_evidence_check.py"])
run(["python3", "scripts/desktop_distribution_publication_check.py"])

stable_release_evidence = require_json("target/release-evidence/stable-release-evidence.json")
desktop_distribution_publication = require_json("target/release-evidence/desktop-distribution-publication.json")
release_readiness = require_json("target/release-evidence/release-readiness.json")
registry_status = require_yaml("packaging/registry-status.yaml")

if stable_release_evidence.get("status") != "pass":
    FAILURES.append("stable release evidence must pass")
if desktop_distribution_publication.get("status") != "pass":
    FAILURES.append("desktop distribution publication must pass")
if release_readiness.get("status") != "pass":
    FAILURES.append("release readiness must pass")
if "registries" not in registry_status:
    FAILURES.append("registry status must include registries mapping")

report = {
    "check": "stable_release_cutover_registries",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "docs/stable-desktop-release.md",
        "docs/desktop-distribution-publication.md",
        "packaging/desktop-distribution.yaml",
        "packaging/registry-status.yaml",
        "target/release-evidence/stable-release-evidence.json",
        "target/release-evidence/desktop-distribution-publication.json",
        "target/release-evidence/release-readiness.json",
        "target/release-evidence/launch-qa.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("STABLE RELEASE CUTOVER AND REGISTRIES CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("stable release cutover and registries: pass")
