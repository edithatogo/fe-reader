#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "exhaustive-pdf-parity-taxonomy.json"
FAILURES: list[str] = []


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        FAILURES.append(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        FAILURES.append(f"command failed: {' '.join(command)}")


for rel, tokens in {
    "README.md": [
        "docs/exhaustive-pdf-parity-taxonomy-contract.md",
        "scripts/exhaustive_pdf_parity_taxonomy_check.py",
    ],
    "docs/stable-desktop-release.md": [
        "docs/exhaustive-pdf-parity-taxonomy-contract.md",
    ],
    "docs/launch-limitations-support.md": [
        "docs/exhaustive-pdf-parity-taxonomy-contract.md",
    ],
    "docs/exhaustive-pdf-parity-taxonomy-contract.md": [
        "Exhaustive PDF Parity Taxonomy and Contracts",
        "docs/pdf-parity-registry.md",
        "docs/pdf-baseline-parity-matrix.md",
        "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt",
    ],
}.items():
    text = read_text(rel)
    for token in tokens:
        if token not in text:
            FAILURES.append(f"{rel} missing token: {token}")

run(["python3", "scripts/pdf_parity_registry_check.py"])
run(["python3", "scripts/pdf_baseline_parity_check.py"])

registry_report = ROOT / "target" / "pdf-parity-registry-check.json"
baseline_report = ROOT / "target" / "pdf-baseline-parity-check.json"
for path, label in [(registry_report, "registry"), (baseline_report, "baseline")]:
    if not path.exists():
        FAILURES.append(f"missing {label} parity report")
        continue
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        FAILURES.append(f"{label} parity report invalid JSON: {exc}")
        continue
    if data.get("status") != "pass":
        FAILURES.append(f"{label} parity report must pass")

report = {
    "check": "exhaustive_pdf_parity_taxonomy",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "docs/pdf-parity-registry.md",
        "docs/pdf-parity-registry.json",
        "docs/pdf-baseline-parity-matrix.md",
        "docs/pdf-baseline-parity-matrix.json",
        "target/pdf-parity-registry-check.json",
        "target/pdf-baseline-parity-check.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("EXHAUSTIVE PDF PARITY TAXONOMY CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("exhaustive pdf parity taxonomy: pass")
