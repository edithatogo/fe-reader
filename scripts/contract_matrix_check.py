#!/usr/bin/env python3
from __future__ import annotations

import shlex
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "contracts" / "ci" / "contract-test-matrix.yaml"
WORKFLOW_DIR = ROOT / ".github" / "workflows"


def fail(message: str) -> None:
    raise SystemExit(f"contract matrix check failed: {message}")


def command_script(command: str) -> str | None:
    parts = shlex.split(command)
    if not parts:
        return None
    if parts[0] in {"python3", "bash"} and len(parts) > 1:
        return parts[1]
    if parts[0].startswith("scripts/"):
        return parts[0]
    return None


data = yaml.safe_load(MATRIX.read_text(encoding="utf-8"))
if not isinstance(data, dict):
    fail("matrix root must be a mapping")

checks = data.get("matrix")
if not isinstance(checks, dict) or not checks:
    fail("matrix must contain named checks")

workflow_text = "\n".join(
    path.read_text(encoding="utf-8", errors="replace")
    for path in sorted(WORKFLOW_DIR.glob("*.yml"))
)
phase_gate = (ROOT / "scripts" / "conductor_phase_gate.sh").read_text(encoding="utf-8", errors="replace")

required_entries = {
    "architecture",
    "strict_mutation_contract",
    "repository_ci_cd",
    "frontier_ci_policy",
    "schemas",
    "contract_matrix",
    "release_matrix",
    "release_readiness",
    "security_policy",
}
missing_entries = sorted(required_entries - set(checks))
if missing_entries:
    fail(f"missing required entries: {', '.join(missing_entries)}")

for name, entry in checks.items():
    if not isinstance(entry, dict):
        fail(f"{name} entry must be a mapping")
    command = entry.get("command")
    gate = entry.get("gate")
    if not isinstance(command, str) or not command.strip():
        fail(f"{name} missing command")
    if not isinstance(gate, str) or not gate.strip():
        fail(f"{name} missing gate")

    script = command_script(command)
    if script and not (ROOT / script).exists():
        fail(f"{name} references missing script {script}")

    blocks_pr = entry.get("blocks_pr")
    if blocks_pr is True and command.startswith(("python3 ", "bash ", "scripts/")):
        if command not in workflow_text and command not in phase_gate:
            fail(f"{name} blocks PRs but command is not wired into workflows or phase gate: {command}")

    advisory = gate.startswith("advisory") or blocks_pr is False
    if advisory:
        if entry.get("baseline_required") is not True:
            fail(f"{name} advisory check must declare baseline_required: true")
        baseline = entry.get("baseline_artifact")
        if not isinstance(baseline, str) or not baseline:
            fail(f"{name} advisory check must declare baseline_artifact")
        if gate in {"P0", "release", "P0_for_release", "P0_for_public_release"}:
            fail(f"{name} advisory check must not use hard gate {gate}")
    else:
        if gate.startswith("advisory"):
            fail(f"{name} hard check must not use advisory gate {gate}")

print("contract matrix check passed")
