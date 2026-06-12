#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
failures: list[str] = []


def read(path: str) -> str:
    full = ROOT / path
    if not full.exists():
        failures.append(f"missing frontier CI file: {path}")
        return ""
    return full.read_text(encoding="utf-8", errors="replace")


frontier = read(".github/workflows/05-frontier-nightly.yml")
performance = read(".github/workflows/06-performance-nightly.yml")
matrix = read("contracts/ci/contract-test-matrix.yaml")

for path, text in [
    (".github/workflows/05-frontier-nightly.yml", frontier),
    (".github/workflows/06-performance-nightly.yml", performance),
]:
    if not text:
        continue
    for token in ["workflow_dispatch:", "schedule:", "continue-on-error: true", "permissions:\n  contents: read"]:
        if token not in text:
            failures.append(f"{path} missing frontier isolation token: {token}")
    if "pull_request:" in text or "push:" in text:
        failures.append(f"{path} must not run on PR/push")
    for forbidden in ["contents: write", "id-token: write", "packages: write"]:
        if forbidden in text:
            failures.append(f"{path} contains forbidden frontier token: {forbidden}")

if frontier:
    try:
        frontier_doc = yaml.safe_load(frontier)
    except yaml.YAMLError as exc:
        failures.append(f"frontier nightly invalid yaml: {exc}")
        frontier_doc = None
    if not isinstance(frontier_doc, dict) or "jobs" not in frontier_doc:
        failures.append("frontier nightly invalid structure")
    else:
        job = frontier_doc["jobs"].get("frontier", {})
        matrix_doc = job.get("strategy", {}).get("matrix", {})
        if matrix_doc.get("toolchain") != ["beta", "nightly"]:
            failures.append("frontier nightly toolchain matrix mismatch")
        if job.get("continue-on-error") is not True:
            failures.append("frontier nightly must continue-on-error")
        step_runs = [step.get("run") for step in job.get("steps", []) if isinstance(step, dict)]
        for command in [
            "rustup toolchain install ${{ matrix.toolchain }} --profile minimal",
            "cargo +${{ matrix.toolchain }} check --workspace --all-targets",
            "bash scripts/miri_smoke.sh",
            "bash scripts/sanitizer_smoke.sh",
            "bash scripts/fuzz_smoke.sh",
            "bash scripts/gpu_frontier_smoke.sh",
            "bash scripts/differential_oracle_smoke.sh",
        ]:
            if command not in step_runs:
                failures.append(f"frontier nightly missing command: {command}")

if performance:
    try:
        performance_doc = yaml.safe_load(performance)
    except yaml.YAMLError as exc:
        failures.append(f"performance nightly invalid yaml: {exc}")
        performance_doc = None
    if not isinstance(performance_doc, dict) or "jobs" not in performance_doc:
        failures.append("performance nightly invalid structure")
    else:
        job = performance_doc["jobs"].get("performance", {})
        if job.get("continue-on-error") is not True:
            failures.append("performance nightly must continue-on-error")
        step_runs = [step.get("run") for step in job.get("steps", []) if isinstance(step, dict)]
        for command in [
            "bash scripts/perf_smoke.sh",
            "bash scripts/toolchain_experiment_smoke.sh",
        ]:
            if command not in step_runs:
                failures.append(f"performance nightly missing command: {command}")
        upload_steps = [
            step for step in job.get("steps", []) if isinstance(step, dict) and step.get("uses", "").startswith("actions/upload-artifact")
        ]
        if not upload_steps:
            failures.append("performance nightly missing artifact upload step")
        else:
            with_section = upload_steps[0].get("with", {})
            if with_section.get("path") != "artifacts/perf/**":
                failures.append("performance nightly artifact path mismatch")
            if with_section.get("if-no-files-found") != "warn":
                failures.append("performance nightly artifact missing warn mode")
            if with_section.get("retention-days") != 14:
                failures.append("performance nightly artifact retention mismatch")

if matrix:
    for token in [
        "frontier_miri",
        "frontier_sanitizers",
        "frontier_fuzz",
        "frontier_gpu",
        "frontier_toolchain_experiments",
        "performance_smoke_advisory",
    ]:
        if token not in matrix:
            failures.append(f"contract test matrix missing frontier entry: {token}")
    for bad in re.finditer(r"frontier_[A-Za-z0-9_]+:\n(?:    .+\n)+?    gate: (P0|release|P0_for_release)", matrix):
        entry = bad.group(0).splitlines()[0]
        if entry != "frontier_ci_policy:":
            failures.append(f"frontier matrix entry is hard-gated: {entry}")

for script in [
    "scripts/miri_smoke.sh",
    "scripts/sanitizer_smoke.sh",
    "scripts/fuzz_smoke.sh",
    "scripts/gpu_frontier_smoke.sh",
    "scripts/toolchain_experiment_smoke.sh",
]:
    text = read(script)
    if text and "target/frontier-reports" not in text:
        failures.append(f"{script} must write an advisory report under target/frontier-reports")

perf_smoke = read("scripts/perf_smoke.sh")
if perf_smoke:
    for token in ["artifacts/perf", "summary.md", "manifest.json"]:
        if token not in perf_smoke:
            failures.append(f"perf_smoke.sh missing performance evidence token: {token}")

if failures:
    print("FRONTIER CI CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("frontier CI check passed")
