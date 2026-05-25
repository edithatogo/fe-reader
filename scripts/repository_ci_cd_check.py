#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
failures: list[str] = []


def read(path: str) -> str:
    full = ROOT / path
    if not full.exists():
        failures.append(f"missing repository CI/CD file: {path}")
        return ""
    return full.read_text(encoding="utf-8", errors="replace")


def require_tokens(path: str, tokens: list[str]) -> None:
    text = read(path)
    for token in tokens:
        if token not in text:
            failures.append(f"{path} missing token: {token}")


codeowners = read(".github/CODEOWNERS")
if codeowners:
    required_patterns = [
        "*",
        "/.github/",
        "/contracts/",
        "/schemas/",
        "/scripts/",
        "/crates/fe_reader_core/",
        "/contracts/platform/",
        "/packaging/",
    ]
    for pattern in required_patterns:
        if not re.search(rf"^{re.escape(pattern)}\s+@\S+", codeowners, re.M):
            failures.append(f"CODEOWNERS missing owned pattern: {pattern}")

dependabot = read(".github/dependabot.yml")
if dependabot:
    for token in [
        "package-ecosystem: github-actions",
        "interval: weekly",
        "open-pull-requests-limit: 5",
        "labels:",
        "groups:",
    ]:
        if token not in dependabot:
            failures.append(f"dependabot missing policy token: {token}")
    if "package-ecosystem: cargo" in dependabot:
        failures.append("Dependabot must not overlap Renovate for Cargo scheduled updates")

renovate = read("renovate.json")
if renovate:
    try:
        data = json.loads(renovate)
    except json.JSONDecodeError as exc:
        failures.append(f"renovate.json is invalid JSON: {exc}")
    else:
        if "config:recommended" not in data.get("extends", []):
            failures.append("renovate.json must extend config:recommended")
        if data.get("enabledManagers") != ["cargo"]:
            failures.append("renovate.json must make Renovate authoritative for Cargo only")
        if data.get("dependencyDashboard") is not True:
            failures.append("renovate.json must enable dependency dashboard")
        rules = json.dumps(data.get("packageRules", []))
        for token in ["high-risk-pdf-contract", "frontier-lane", "Rust patch/minor dependencies"]:
            if token not in rules:
                failures.append(f"renovate package rules missing token: {token}")

ruleset = read(".github/rulesets/main-branch-ruleset.template.json")
if ruleset:
    try:
        data = json.loads(ruleset)
    except json.JSONDecodeError as exc:
        failures.append(f"main branch ruleset template is invalid JSON: {exc}")
    else:
        if data.get("target") != "branch":
            failures.append("main branch ruleset must target branches")
        if data.get("enforcement") != "active":
            failures.append("main branch ruleset must be active")
        rules = json.dumps(data.get("rules", []))
        for token in [
            "pull_request",
            "required_status_checks",
            "deletion",
            "non_fast_forward",
            "required_linear_history",
            "required_signatures",
            "strict-contracts",
            "repository-ci-cd",
            "rust",
            "policy",
            "smoke",
        ]:
            if token not in rules:
                failures.append(f"main branch ruleset missing token: {token}")
        advisory_contexts = [
            "frontier",
            "nightly",
            "performance",
            "visual",
            "corpus",
            "api-compatibility",
        ]
        required_checks = json.dumps(
            [
                check.get("context", "")
                for rule in data.get("rules", [])
                if rule.get("type") == "required_status_checks"
                for check in rule.get("parameters", {}).get("required_status_checks", [])
            ]
        )
        for context in advisory_contexts:
            if context in required_checks:
                failures.append(f"branch ruleset must not require advisory/frontier context without ADR: {context}")

for path in sorted((ROOT / ".github/workflows").glob("*.yml")):
    text = path.read_text(encoding="utf-8")
    rel = path.relative_to(ROOT)
    for token in ["permissions:", "concurrency:", "timeout-minutes:"]:
        if token not in text:
            failures.append(f"{rel} missing {token}")
    if "uses: " in text:
        for line in text.splitlines():
            stripped = line.strip()
            if stripped.startswith("- uses:") and "ALLOW_VERSION_TAGS_DURING_BOOTSTRAP" not in stripped:
                failures.append(f"{rel} action use missing bootstrap/pinning marker: {stripped}")

release_workflow = read(".github/workflows/07-release.yml")
if release_workflow:
    for token in [
        "bash scripts/release_evidence_check.sh",
        "bash scripts/sbom_audit.sh",
        "bash scripts/generate_provenance_attestation.sh",
        "bash scripts/signing_readiness_check.sh",
        "python3 scripts/release_provenance_check.py",
        "python3 scripts/release_matrix_check.py",
        "bash scripts/release_readiness_check.sh",
        "actions/upload-artifact",
        "target/release-evidence/**",
        "if-no-files-found: error",
        "retention-days:",
    ]:
        if token not in release_workflow:
            failures.append(f"release workflow missing evidence token: {token}")

evidence_schema = read("schemas/release-evidence.schema.json")
if evidence_schema:
    try:
        data = json.loads(evidence_schema)
    except json.JSONDecodeError as exc:
        failures.append(f"release evidence schema is invalid JSON: {exc}")
    else:
        required = set(data.get("required", []))
        if data.get("additionalProperties") is not False:
            failures.append("release evidence schema must reject undeclared top-level fields")
        for field in {"release_id", "channel", "source_commit", "toolchain", "artifacts"}:
            if field not in required:
                failures.append(f"release evidence schema missing required field: {field}")
        for field in ["sbom_path", "provenance_path", "signing_readiness_path", "workflow_run", "builder", "materials"]:
            if field not in data.get("properties", {}):
                failures.append(f"release evidence schema missing provenance field: {field}")
        source_commit = data.get("properties", {}).get("source_commit", {})
        if source_commit.get("pattern") != "^[0-9a-f]{40}$":
            failures.append("release evidence schema must require a 40-character lowercase git commit")
        artifacts = data.get("properties", {}).get("artifacts", {})
        artifact_required = set(artifacts.get("items", {}).get("required", []))
        for field in {"path", "sha256", "kind"}:
            if field not in artifact_required:
                failures.append(f"release evidence artifact items must require {field}")

require_tokens(
    "docs/repository-governance-branch-protection.md",
    [
        ".github/rulesets/main-branch-ruleset.template.json",
        "CODEOWNERS review",
        "required status checks",
        "Action pinning",
    ],
)
require_tokens(
    "docs/v9-coding-agent-start-here.md",
    [
        "python3 scripts/repository_ci_cd_check.py",
        "Configure branch protection / rulesets",
        "CI workflows pass static policy checks",
    ],
)

if failures:
    print("REPOSITORY CI/CD CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("repository CI/CD check passed")
