#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "professional-workflow-parity.json"
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
        "docs/professional-workflow-parity-contract.md",
        "scripts/professional_workflow_parity_check.py",
    ],
)
require_tokens(
    "docs/professional-workflow-parity-contract.md",
    [
        "Professional Workflow Parity Contract",
        "scripts/forms_contract_smoke.sh",
        "scripts/redaction_verification_smoke.sh",
        "scripts/conversion_contract_smoke.sh",
    ],
)
for rel in [
    "docs/stable-reader-readiness.md",
    "docs/pdf-baseline-parity-matrix.md",
    "docs/pdf-parity-registry.md",
]:
    require_tokens(rel, ["docs/professional-workflow-parity-contract.md"])

run(["bash", "scripts/forms_contract_smoke.sh"])
run(["bash", "scripts/redaction_verification_smoke.sh"])
run(["bash", "scripts/conversion_contract_smoke.sh"])
run(["python3", "scripts/audit_receipt_contract_check.py"])

forms = require_json("target/conversion-reports/conversion-contract-smoke.json")
redaction = require_json("target/release-evidence/redaction-verification-smoke.json")

if forms.get("status") != "pass":
    FAILURES.append("conversion contract smoke must pass")
if redaction.get("status") not in {"pass", "ok"}:
    FAILURES.append("redaction verification evidence must pass")
if not (ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.audit_receipt.preview.json").exists():
    FAILURES.append("audit receipt snapshot must exist")

report = {
    "check": "professional_workflow_parity",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "scripts/forms_contract_smoke.sh",
        "scripts/redaction_verification_smoke.sh",
        "scripts/conversion_contract_smoke.sh",
        "scripts/audit_receipt_contract_check.py",
        "target/conversion-reports/conversion-contract-smoke.json",
        "target/release-evidence/audit-receipt-contract.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("PROFESSIONAL WORKFLOW PARITY CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("professional workflow parity: pass")
