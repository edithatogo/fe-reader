#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "advanced-pdf-family-parity.json"
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


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        FAILURES.append(f"command failed: {' '.join(command)}")


require_tokens(
    "README.md",
    [
        "docs/advanced-pdf-family-parity-contract.md",
        "scripts/advanced_pdf_family_parity_check.py",
    ],
)
require_tokens(
    "docs/advanced-pdf-family-parity-contract.md",
    [
        "Advanced PDF Family Parity Contract",
        "docs/pdf-baseline-parity-matrix.md",
        "docs/pdf-parity-registry.md",
        "advanced_pdf_baseline",
    ],
)
for rel in [
    "docs/stable-desktop-release.md",
    "docs/stable-reader-readiness.md",
    "docs/pdf-baseline-parity-matrix.md",
    "docs/pdf-parity-registry.md",
]:
    require_tokens(rel, ["docs/advanced-pdf-family-parity-contract.md"])

run(["python3", "scripts/pdf_baseline_parity_check.py"])
run(["python3", "scripts/pdf_parity_registry_check.py"])

baseline = require_json("target/pdf-baseline-parity-check.json")
registry = require_json("target/pdf-parity-registry-check.json")

if baseline.get("status") != "pass":
    FAILURES.append("baseline parity check must pass")
if registry.get("status") != "pass":
    FAILURES.append("registry parity check must pass")

report = {
    "check": "advanced_pdf_family_parity",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "docs/pdf-baseline-parity-matrix.md",
        "docs/pdf-baseline-parity-matrix.json",
        "docs/pdf-parity-registry.md",
        "docs/pdf-parity-registry.json",
        "target/pdf-baseline-parity-check.json",
        "target/pdf-parity-registry-check.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("ADVANCED PDF FAMILY PARITY CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("advanced pdf family parity: pass")
