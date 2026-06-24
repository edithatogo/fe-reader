#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "marketing-claim-governance.json"
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
        "docs/marketing-claim-governance-contract.md",
        "scripts/marketing_claim_governance_check.py",
    ],
)
require_tokens(
    "docs/marketing-claim-governance-contract.md",
    [
        "Marketing Claim Governance Contract",
        "docs/marketing-readiness.md",
        "scripts/marketing_claim_evidence_governance_check.py",
        "technical preview",
    ],
)
for rel in [
    "docs/marketing-readiness.md",
    "docs/stable-reader-readiness.md",
    "docs/pdf-parity-registry.md",
    "docs/pdf-baseline-parity-matrix.md",
    "docs/release-notes/v0.1.0-preview.1.md",
]:
    require_tokens(rel, ["docs/marketing-claim-governance-contract.md"])

run(["python3", "scripts/marketing_claim_evidence_governance_check.py"])
marketing = require_json("target/release-evidence/marketing-readiness.json")

if marketing.get("status") != "pass":
    FAILURES.append("marketing readiness evidence must pass")

report = {
    "check": "marketing_claim_governance",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "docs/marketing-readiness.md",
        "target/release-evidence/marketing-readiness.json",
        "scripts/marketing_claim_evidence_governance_check.py",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("MARKETING CLAIM GOVERNANCE CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("marketing claim governance: pass")
