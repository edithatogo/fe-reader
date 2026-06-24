#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target"
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
        "docs/corpus-oracle-evidence-factory-contract.md",
        "scripts/corpus_oracle_evidence_factory_check.py",
    ],
)
require_tokens(
    "docs/pdf-parity-registry.md",
    [
        "docs/corpus-oracle-evidence-factory-contract.md",
    ],
)
require_tokens(
    "docs/pdf-baseline-parity-matrix.md",
    [
        "docs/corpus-oracle-evidence-factory-contract.md",
    ],
)
require_tokens(
    "docs/corpus-oracle-evidence-factory-contract.md",
    [
        "Corpus/Oracle Evidence Factory Contract",
        "fixtures/corpus/manifest.json",
        "scripts/compatibility_corpus_report.py",
        "scripts/differential_oracle_smoke.sh",
    ],
)

run(["python3", "scripts/compatibility_corpus_report.py"])
run(["bash", "scripts/differential_oracle_smoke.sh"])

corpus_report = require_json("target/compatibility-corpus-report.json")
oracle_report = require_json("target/oracle-reports/wave1-render-smoke.json")

if corpus_report.get("report_kind") != "compatibility-corpus":
    FAILURES.append("compatibility corpus report kind drifted")
if corpus_report.get("fixture_count", 0) < 1:
    FAILURES.append("compatibility corpus report must include fixtures")
if not corpus_report.get("parity_families"):
    FAILURES.append("compatibility corpus report must include parity families")

if oracle_report.get("operation") != "render_visual_similarity":
    FAILURES.append("oracle smoke must report render_visual_similarity")
if oracle_report.get("fixture_id") != "basic-text-search-fixture":
    FAILURES.append("oracle smoke must cover the accepted fixture")
if oracle_report.get("comparison", {}).get("status") not in {"oracle_unavailable", "pass"}:
    FAILURES.append("oracle smoke comparison status drifted")

report = {
    "check": "corpus_oracle_evidence_factory",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "fixtures/corpus/manifest.json",
        "target/compatibility-corpus-report.json",
        "target/compatibility-corpus-report.md",
        "target/oracle-reports/wave1-render-smoke.json",
        "docs/pdf-parity-registry.md",
        "docs/pdf-baseline-parity-matrix.md",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
(EVIDENCE_DIR / "corpus-oracle-evidence-factory.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("CORPUS/ORACLE EVIDENCE FACTORY CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("corpus/oracle evidence factory: pass")
