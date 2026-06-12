#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import yaml

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
    try:
        doc = yaml.safe_load(text)
    except yaml.YAMLError as exc:
        failures.append(f"{path.relative_to(ROOT)} invalid workflow yaml: {exc}")
        doc = None
    rel = path.relative_to(ROOT)
    for token in ["permissions:", "concurrency:", "timeout-minutes:"]:
        if token not in text:
            failures.append(f"{rel} missing {token}")
    if "uses: " in text:
        for line in text.splitlines():
            stripped = line.strip()
            if (
                (stripped.startswith("- uses:") or stripped.startswith("uses:"))
                and "ALLOW_VERSION_TAGS_DURING_BOOTSTRAP" not in stripped
            ):
                failures.append(f"{rel} action use missing bootstrap/pinning marker: {stripped}")

    if path.name == "08-docs-site.yml":
        if not isinstance(doc, dict) or "jobs" not in doc:
            failures.append(f"{rel} invalid workflow structure")
        else:
            build = doc["jobs"].get("build", {})
            deploy = doc["jobs"].get("deploy", {})
            if build.get("name") != "Build Starlight docs":
                failures.append(f"{rel} build job name mismatch")
            if deploy.get("name") != "Deploy Starlight docs":
                failures.append(f"{rel} deploy job name mismatch")
            if build.get("timeout-minutes") != 15 or deploy.get("timeout-minutes") != 15:
                failures.append(f"{rel} timeout-minutes mismatch")
            if build.get("permissions", {}).get("contents") != "read":
                failures.append(f"{rel} build permissions mismatch")
            if deploy.get("permissions", {}).get("pages") != "write" or deploy.get("permissions", {}).get("id-token") != "write":
                failures.append(f"{rel} deploy permissions mismatch")
            build_step_uses = {step.get("uses") for step in build.get("steps", []) if isinstance(step, dict)}
            deploy_step_uses = {step.get("uses") for step in deploy.get("steps", []) if isinstance(step, dict)}
            for action in [
                "actions/checkout@v6 # ALLOW_VERSION_TAGS_DURING_BOOTSTRAP",
                "actions/setup-node@v6 # ALLOW_VERSION_TAGS_DURING_BOOTSTRAP",
                "actions/configure-pages@v6 # ALLOW_VERSION_TAGS_DURING_BOOTSTRAP",
                "actions/upload-pages-artifact@v5 # ALLOW_VERSION_TAGS_DURING_BOOTSTRAP",
                "actions/deploy-pages@v5 # ALLOW_VERSION_TAGS_DURING_BOOTSTRAP",
            ]:
                if action not in text:
                    failures.append(f"{rel} missing docs site deployment token: {action}")

release_workflow = read(".github/workflows/07-release.yml")
if release_workflow:
    try:
        release_doc = yaml.safe_load(release_workflow)
    except yaml.YAMLError as exc:
        failures.append(f"release workflow invalid yaml: {exc}")
        release_doc = None
    if not isinstance(release_doc, dict) or "jobs" not in release_doc:
        failures.append("release workflow invalid structure")
    else:
        job = release_doc["jobs"].get("release-readiness", {})
        steps = job.get("steps", [])
        step_runs = {step.get("run") for step in steps if isinstance(step, dict) and "run" in step}
        required_runs = {
            "bash scripts/release_evidence_check.sh",
            "bash scripts/sbom_audit.sh",
            "bash scripts/generate_provenance_attestation.sh",
            "bash scripts/signing_readiness_check.sh",
            "python3 scripts/release_provenance_check.py",
            "python3 scripts/release_matrix_check.py",
            "bash scripts/release_readiness_check.sh",
        }
        if not required_runs.issubset(step_runs):
            failures.append("release workflow missing required release readiness steps")
        upload_steps = [
            step for step in steps if isinstance(step, dict) and step.get("uses", "").startswith("actions/upload-artifact")
        ]
        if not upload_steps:
            failures.append("release workflow missing artifact upload step")
        else:
            with_section = upload_steps[0].get("with", {})
            if with_section.get("path") != "target/release-evidence/**":
                failures.append("release workflow artifact path mismatch")
            if with_section.get("if-no-files-found") != "error":
                failures.append("release workflow artifact missing error mode")
            if with_section.get("retention-days") != 30:
                failures.append("release workflow artifact retention mismatch")

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
