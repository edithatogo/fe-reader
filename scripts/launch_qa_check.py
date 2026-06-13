#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import pathlib
import subprocess
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[1]
# Launch QA evidence is emitted under target/release-evidence for release upload.
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
STRICT_CHANNELS = {"stable", "lts", "store_submission"}

COMMANDS: list[list[str]] = [
    ["bash", "scripts/smoke_cli_contract.sh"],
    ["python3", "scripts/desktop_packaging_signing_check.py"],
    ["python3", "scripts/stable_release_evidence_check.py"],
    ["python3", "scripts/desktop_distribution_publication_check.py"],
    ["python3", "scripts/enterprise_operations_readiness_check.py"],
    ["python3", "scripts/release_matrix_check.py"],
    ["python3", "scripts/release_provenance_check.py"],
    ["bash", "scripts/release_readiness_check.sh"],
    ["bash", "scripts/security_policy_check.sh"],
    ["python3", "scripts/accessibility_audit_smoke.py"],
]

EVIDENCE_CHECKS = {
    "release_readiness": "target/release-evidence/release-readiness.json",
    "release_matrix": "target/release-evidence/release-matrix.json",
    "desktop_packaging_signing": "target/release-evidence/desktop-packaging-signing.json",
    "desktop_distribution_publication": "target/release-evidence/desktop-distribution-publication.json",
    "enterprise_operations_readiness": "target/release-evidence/enterprise-operations-readiness.json",
    "visual_regression": "target/visual-regression/text-search-fixture/comparison.json",
    "compatibility_corpus": "target/compatibility-corpus-report.json",
    "search_compatibility": "target/search-compatibility-report.json",
    "accessibility": "target/accessibility-reports/smoke.json",
}

DOC_TOKENS = {
    "README.md": [
        "scripts/launch_qa_check.py",
        "docs/stable-desktop-release.md",
        "docs/launch-limitations-support.md",
        "SHA256SUMS",
        "SECURITY.md",
    ],
    "docs/stable-desktop-release.md": [
        "macOS",
        "Windows",
        "Linux",
        "GitHub Releases",
        "SHA256SUMS",
        "scripts/launch_qa_check.py",
    ],
    "docs/launch-limitations-support.md": [
        "mobile",
        "ML/RAG",
        "cloud collaboration",
        "SECURITY.md",
        "SUPPORT.md",
    ],
    "docs-site/src/content/docs/stable-desktop-release.md": [
        "Stable Desktop Release",
        "GitHub Releases",
        "SHA256SUMS",
        "Known launch limitations",
    ],
}


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        failures.append(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def load_yaml(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    data = yaml.safe_load(text)
    if not isinstance(data, dict):
        failures.append(f"{rel} must contain a mapping")
        return {}
    return data


def command_name(command: list[str]) -> str:
    return " ".join(command)


def run_command(command: list[str]) -> dict:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    return {
        "command": command_name(command),
        "status": "pass" if result.returncode == 0 else "fail",
        "returncode": result.returncode,
        "stdout_tail": result.stdout[-1000:],
        "stderr_tail": result.stderr[-1000:],
    }


def status_for_json(path: pathlib.Path) -> str | None:
    if not path.exists() or path.suffix != ".json":
        return None
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return "invalid_json"
    status = data.get("status")
    return str(status) if status is not None else "present"


failures: list[str] = []
channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
strict = channel in STRICT_CHANNELS
command_reports = [run_command(command) for command in COMMANDS]
for report in command_reports:
    if report["status"] != "pass":
        failures.append(f"launch QA command failed: {report['command']}")

evidence_reports: list[dict] = []
for name, rel in EVIDENCE_CHECKS.items():
    path = ROOT / rel
    status = status_for_json(path) if path.exists() else None
    evidence_reports.append({"name": name, "path": rel, "exists": path.exists(), "status": status or "missing"})
    if strict and not path.exists():
        failures.append(f"stable launch QA missing evidence: {rel}")
    if strict and status in {"fail", "failed", "error", "blocked", "invalid_json"}:
        failures.append(f"stable launch QA evidence {rel} has blocking status {status}")

for rel, tokens in DOC_TOKENS.items():
    text = read_text(rel)
    if not text:
        continue
    for token in tokens:
        if token not in text:
            failures.append(f"{rel} missing token: {token}")

metadata = load_yaml(".github/repository-metadata.yaml")
packages = metadata.get("packages") if isinstance(metadata.get("packages"), dict) else {}
for key in ["homepage", "repository", "releases", "documentation"]:
    if not metadata.get(key):
        failures.append(f"repository metadata missing {key}")
for key in ["release_index", "stable_desktop_release", "launch_qa", "support", "security"]:
    if not packages.get(key):
        failures.append(f"repository metadata packages missing {key}")

support = read_text("SUPPORT.md")
security = read_text("SECURITY.md")
if support and "private PDFs" not in support:
    failures.append("SUPPORT.md must warn against sharing private PDFs")
if security and "privately" not in security.lower():
    failures.append("SECURITY.md must document private vulnerability reporting")

report = {
    "check": "launch_qa",
    "channel": channel,
    "status": "fail" if failures else "pass",
    "commands": command_reports,
    "evidence": evidence_reports,
    "documentation": sorted(DOC_TOKENS),
    "metadata": {
        "homepage": metadata.get("homepage"),
        "repository": metadata.get("repository"),
        "releases": metadata.get("releases"),
        "documentation": metadata.get("documentation"),
    },
    "failures": failures,
}
(EVIDENCE_DIR / "launch-qa.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")

if failures:
    print("LAUNCH QA CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("launch QA: ok")
