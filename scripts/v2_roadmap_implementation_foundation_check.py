#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "v2-roadmap-implementation-foundation.json"
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


def load_json(rel: str) -> dict:
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
        "docs/v2-roadmap-foundation.md",
        "docs/v2-roadmap-implementation-foundation-contract.md",
        "scripts/v2_roadmap_implementation_foundation_check.py",
    ],
)
require_tokens(
    "docs/v2-roadmap-foundation.md",
    [
        "v2 Roadmap Foundation",
        "post-launch advanced roadmap",
        "Stable reader baseline evidence passes",
    ],
)
require_tokens(
    "docs/post-launch-advanced-roadmap.md",
    [
        "track-BO-v2-roadmap-implementation-foundation",
        "v2 roadmap work must stay behind stable launch gates",
    ],
)
require_tokens(
    "docs/v2-roadmap-implementation-foundation-contract.md",
    [
        "v2 Roadmap Implementation Foundation Contract",
        "docs/v2-roadmap-foundation.md",
        "docs/post-launch-advanced-roadmap.md",
        "target/release-evidence/advanced-roadmap.json",
    ],
)

for rel in [
    "docs/stable-reader-readiness.md",
    "docs/stable-desktop-release.md",
    "docs/marketing-readiness.md",
    "docs/launch-limitations-support.md",
]:
    require_tokens(rel, ["docs/v2-roadmap-implementation-foundation-contract.md"])

run(["python3", "scripts/advanced_roadmap_check.py"])
advanced_roadmap = load_json("target/release-evidence/advanced-roadmap.json")
stable_reader_readiness = load_json("target/release-evidence/stable-reader-readiness.json")
release_readiness = load_json("target/release-evidence/release-readiness.json")
stable_release_evidence = load_json("target/release-evidence/stable-release-evidence.json")

if advanced_roadmap.get("status") != "pass":
    FAILURES.append("advanced roadmap must pass")
if stable_reader_readiness.get("status") != "pass":
    FAILURES.append("stable reader readiness must pass")
if release_readiness.get("status") != "pass":
    FAILURES.append("release readiness must pass")
if stable_release_evidence.get("status") != "pass":
    FAILURES.append("stable release evidence must pass")

report = {
    "check": "v2_roadmap_implementation_foundation",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "docs/v2-roadmap-foundation.md",
        "docs/post-launch-advanced-roadmap.md",
        "docs/stable-reader-readiness.md",
        "docs/stable-desktop-release.md",
        "docs/marketing-readiness.md",
        "target/release-evidence/stable-reader-readiness.json",
        "target/release-evidence/release-readiness.json",
        "target/release-evidence/stable-release-evidence.json",
        "target/release-evidence/launch-qa.json",
        "target/release-evidence/advanced-roadmap.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("V2 ROADMAP IMPLEMENTATION FOUNDATION CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("v2 roadmap implementation foundation: pass")
