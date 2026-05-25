#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

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
    for command in [
        "toolchain: [beta, nightly]",
        "cargo +${{ matrix.toolchain }} check --workspace --all-targets",
        "bash scripts/miri_smoke.sh",
        "bash scripts/sanitizer_smoke.sh",
        "bash scripts/fuzz_smoke.sh",
        "bash scripts/gpu_frontier_smoke.sh",
        "bash scripts/differential_oracle_smoke.sh",
    ]:
        if command not in frontier:
            failures.append(f"frontier nightly missing command: {command}")

if performance:
    for command in [
        "bash scripts/perf_smoke.sh",
        "bash scripts/toolchain_experiment_smoke.sh",
        "actions/upload-artifact",
        "artifacts/perf/**",
        "if-no-files-found: warn",
    ]:
        if command not in performance:
            failures.append(f"performance nightly missing evidence token: {command}")

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
