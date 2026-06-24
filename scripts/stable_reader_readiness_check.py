#!/usr/bin/env python3
"""Validate the stable-reader baseline and the marketing boundary."""

from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "stable-reader-readiness.json"
STRICT_CHANNELS = {"stable", "lts", "store_submission"}


def fail(message: str) -> None:
    failures.append(message)


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        fail(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def load_json(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        fail(f"{rel} invalid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        fail(f"{rel} must contain a mapping")
        return {}
    return data


def evidence_status(path: str) -> dict:
    full = ROOT / path
    if not full.exists():
        return {"path": path, "status": "missing"}
    if full.suffix != ".json":
        return {"path": path, "status": "present"}
    try:
        data = json.loads(full.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {"path": path, "status": "invalid_json"}
    return {"path": path, "status": str(data.get("status", "present")), "details": str(data.get("check", ""))}


def run(command: list[str]) -> dict:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    return {
        "command": " ".join(command),
        "status": "pass" if result.returncode == 0 else "fail",
        "returncode": result.returncode,
        "stdout_tail": result.stdout[-800:],
        "stderr_tail": result.stderr[-800:],
    }


failures: list[str] = []
channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
strict = channel in STRICT_CHANNELS

commands = [
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "doctor"],
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "inspect", "fixtures/minimal/minimal.pdf", "--json"],
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "metadata", "fixtures/minimal/minimal.pdf", "--json"],
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "search", "fixtures/corpus/basic/text-search-fixture.pdf", "Reader", "--case-sensitive", "--json"],
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "accessibility", "fixtures/minimal/minimal.pdf", "--json"],
    ["bash", "scripts/wave1_render_smoke.sh"],
    ["bash", "scripts/perf_smoke.sh"],
]
command_reports = [run(command) for command in commands]
for report in command_reports:
    if report["status"] != "pass":
        fail(f"stable reader smoke command failed: {report['command']}")

docs = {
    rel: read_text(rel)
    for rel in [
        "README.md",
        "docs/stable-desktop-release.md",
        "docs/stable-reader-readiness.md",
        "docs/usable-stable-bleeding-edge-pdf-reader-contract.md",
        "docs/launch-limitations-support.md",
        "docs/pdf-baseline-parity-matrix.md",
        "docs/ux-accessibility-human-factors.md",
    ]
}

for rel, tokens in {
    "README.md": ["docs/stable-reader-readiness.md", "scripts/stable_reader_readiness_check.py"],
    "docs/stable-desktop-release.md": ["stable-reader readiness", "target/release-evidence/stable-reader-readiness.json"],
    "docs/stable-reader-readiness.md": ["Stable Reader Readiness", "stable_reader_readiness_check.py"],
    "docs/usable-stable-bleeding-edge-pdf-reader-contract.md": [
        "usable stable bleeding-edge PDF reader",
        "stable-reader-readiness",
        "launch-qa",
    ],
    "docs/launch-limitations-support.md": ["stable-reader readiness", "marketing"],
    "docs/pdf-baseline-parity-matrix.md": ["advanced_pdf_baseline", "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt"],
}.items():
    text = docs.get(rel, "")
    if not text:
        continue
    for token in tokens:
        if token not in text:
            fail(f"{rel} missing token: {token}")

launch_qa = load_json("target/release-evidence/launch-qa.json")
release_readiness = load_json("target/release-evidence/release-readiness.json")
stable_release_evidence = load_json("target/release-evidence/stable-release-evidence.json")
pdf_parity = load_json("target/pdf-baseline-parity-check.json")

claims = [
    {
        "id": "open-local-pdfs",
        "support_level": "supported-preview",
        "summary": "Open and inspect local PDFs from CLI entry points.",
        "evidence": [
            "scripts/smoke_cli_contract.sh",
        ],
        "limitation": "",
    },
    {
        "id": "reader-navigation",
        "support_level": "supported-preview",
        "summary": "Navigate pages, zoom, fit, rotate, thumbnails and session state through the baseline reader workflow.",
        "evidence": [
            "scripts/v8_cli_smoke.sh",
            "scripts/wave1_render_smoke.sh",
            "scripts/wave5_integration_smoke.sh",
            "docs/pdf-baseline-parity-matrix.md",
        ],
        "limitation": "Navigation surfaces remain limited to the baseline reader workflow until broader UI automation lands.",
    },
    {
        "id": "metadata-search-safe-open",
        "support_level": "supported-preview",
        "summary": "Inspect metadata, search deterministically and surface safe-open diagnostics.",
        "evidence": [
            "target/search-compatibility-report.json",
            "scripts/pdf_lab_inspect_smoke.sh",
            "scripts/pdf_repair_smoke.sh",
            "scripts/metadata_wave2_smoke.sh",
        ],
        "limitation": "",
    },
    {
        "id": "text-diagnostics",
        "support_level": "supported-preview",
        "summary": "Surface text extraction diagnostics, missing ToUnicode warnings and geometry fallbacks.",
        "evidence": [
            "scripts/metadata_wave2_smoke.sh",
            "scripts/pdf_lab_text_map_smoke.sh",
            "scripts/search_index_smoke.sh",
        ],
        "limitation": "CJK, RTL and ligature shaping remain corpus-dependent until expanded text fixtures and oracle coverage land.",
    },
    {
        "id": "accessibility-and-reader-navigation",
        "support_level": "supported-preview",
        "summary": "Expose keyboard and accessibility evidence for the reader baseline.",
        "evidence": [
            "target/accessibility-reports/smoke.json",
            "docs/ux-accessibility-human-factors.md",
        ],
        "limitation": "",
    },
    {
        "id": "accessibility-reader-parity",
        "support_level": "supported-preview",
        "summary": "Provide keyboard and screen-reader accessibility reports for the reader baseline and tagged-PDF inspection workflows.",
        "evidence": [
            "scripts/accessibility_audit_smoke.py",
            "target/accessibility-reports/smoke.json",
            "docs/pdf-baseline-parity-matrix.md",
        ],
        "limitation": "Tagged-PDF and PDF/UA oracle coverage remains limited to available local adapters and documented limitations.",
    },
    {
        "id": "professional-workflow-boundary",
        "support_level": "plan-only",
        "summary": "Professional workflows remain policy-gated and audit-backed.",
        "evidence": [
            "contracts/cli/cli-contract.md",
            "scripts/forms_contract_smoke.sh",
            "scripts/redaction_verification_smoke.sh",
            "scripts/conversion_contract_smoke.sh",
        ],
        "limitation": "Mutating workflows remain review/policy gated and are not broad marketing claims.",
    },
    {
        "id": "marketing-boundary",
        "support_level": "documented-limitation",
        "summary": "Stable reader marketing remains constrained by release and registry evidence.",
        "evidence": [
            "target/release-evidence/release-readiness.json",
            "target/release-evidence/stable-release-evidence.json",
            "target/pdf-baseline-parity-check.json",
            "docs/launch-limitations-support.md",
        ],
        "limitation": "Broad marketing still requires signed artifacts, checksums, release evidence and registry approval.",
    },
    {
        "id": "searchable-pdf-gating",
        "support_level": "documented-limitation",
        "summary": "Keep OCR-backed searchable-PDF planning provider-gated and opt-in.",
        "evidence": [
            "scripts/ocr_searchable_pdf_contract_smoke.py",
            "docs/scanning-ocr-ingestion.md",
        ],
        "limitation": "Cloud OCR is not enabled by default and local OCR remains a later provider-gated workflow.",
    },
]

for claim in claims:
    for evidence_path in claim["evidence"]:
        status = evidence_status(evidence_path)["status"]
        if status in {"missing", "invalid_json", "fail", "blocked", "error"}:
            fail(f"{claim['id']} evidence not ready: {evidence_path} ({status})")
    if claim["support_level"] in {"plan-only", "documented-limitation"} and not claim["limitation"]:
        fail(f"{claim['id']} missing limitation text")

if strict:
    for path in [
        "target/release-evidence/release-readiness.json",
        "target/release-evidence/stable-release-evidence.json",
        "target/pdf-baseline-parity-check.json",
        "target/accessibility-reports/smoke.json",
        "target/search-compatibility-report.json",
        "target/visual-regression/text-search-fixture/comparison.json",
    ]:
        status = evidence_status(path)["status"]
        if status in {"missing", "invalid_json", "fail", "blocked", "error"}:
            fail(f"strict stable-reader evidence not ready: {path} ({status})")

if release_readiness.get("status") != "pass":
    fail("release readiness must pass for stable reader readiness")
if stable_release_evidence.get("status") != "pass":
    fail("stable release evidence must pass for stable reader readiness")
if pdf_parity.get("status") != "pass":
    fail("pdf baseline parity must pass for stable reader readiness")

report = {
    "check": "stable_reader_readiness",
    "channel": channel,
    "marketing_ready": False,
    "status": "fail" if failures else "pass",
    "commands": command_reports,
    "claims": claims,
    "evidence": [
        evidence_status("target/release-evidence/launch-qa.json"),
        evidence_status("target/release-evidence/release-readiness.json"),
        evidence_status("target/release-evidence/stable-release-evidence.json"),
        evidence_status("target/pdf-baseline-parity-check.json"),
        evidence_status("target/accessibility-reports/smoke.json"),
        evidence_status("target/search-compatibility-report.json"),
        evidence_status("target/visual-regression/text-search-fixture/comparison.json"),
    ],
    "docs": sorted(docs),
    "failures": failures,
}

EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")

if failures:
    print("stable reader readiness check failed")
    for failure in failures:
        print(f" - {failure}")
    raise SystemExit(1)

print("stable reader readiness: pass")
