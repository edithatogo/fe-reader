#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read(path: Path) -> str:
    if not path.exists():
        failures.append(f"missing file: {rel(path)}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def require_tokens(path: Path, tokens: list[str]) -> None:
    text = read(path)
    for token in tokens:
        if token not in text:
            failures.append(f"{rel(path)} missing token: {token}")


def has_unchecked_task(text: str) -> bool:
    return "- [ ]" in text or "- [~]" in text


def check_completed_archive_plans() -> None:
    archive = ROOT / "conductor" / "archive"
    if not archive.exists():
        return
    for metadata_path in sorted(archive.glob("track-*/metadata.json")):
        try:
            metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            failures.append(f"{rel(metadata_path)} invalid JSON: {exc}")
            continue
        if metadata.get("status") != "completed":
            continue
        track_dir = metadata_path.parent
        plan = track_dir / "plan.md"
        plan_text = read(plan)
        if not has_unchecked_task(plan_text):
            continue
        evidence = track_dir / "completion-evidence.md"
        evidence_text = read(evidence)
        for token in [
            "Completion scope",
            "Deferred external gates",
            "Review evidence",
            "unchecked plan items are not active implementation tasks",
        ]:
            if token not in evidence_text:
                failures.append(f"{rel(evidence)} missing archive normalization token: {token}")


def check_workflow_and_automation() -> None:
    require_tokens(
        ROOT / "AGENTS.md",
        [
            "v10 Conductor lifecycle rules",
            "refs/notes/conductor",
            "conductor-review",
            "GitHub Actions green",
        ],
    )
    require_tokens(
        ROOT / "conductor" / "workflow.md",
        [
            "Task Workflow",
            "Phase Workflow",
            "Track Closeout Workflow",
            "git notes --ref=conductor",
            "git push origin refs/notes/conductor",
            "GitHub Actions passing",
        ],
    )
    automation_path = ROOT / "conductor" / "automation.yaml"
    automation_text = read(automation_path)
    if automation_text:
        try:
            automation = yaml.safe_load(automation_text)
        except yaml.YAMLError as exc:
            failures.append(f"{rel(automation_path)} invalid YAML: {exc}")
            automation = {}
        lifecycle = automation.get("lifecycle", {}) if isinstance(automation, dict) else {}
        if lifecycle.get("note_ref") != "refs/notes/conductor":
            failures.append("conductor automation lifecycle.note_ref must be refs/notes/conductor")
        for key in [
            "task_commit_required",
            "task_git_note_required",
            "task_push_required",
            "phase_review_required",
            "track_review_required",
            "track_archive_after_review",
            "track_actions_green_required",
        ]:
            if key not in lifecycle:
                failures.append(f"conductor automation lifecycle missing {key}")
        workflows = set(lifecycle.get("required_workflows", []))
        for workflow in [
            "PR Contracts",
            "Rust Stable",
            "Security Supply Chain",
            "Cross Platform Smoke",
            "API Compatibility",
            "Docs Site",
            "Platform Tests",
        ]:
            if workflow not in workflows:
                failures.append(f"conductor automation lifecycle missing workflow {workflow}")


def check_ci_ordering() -> None:
    workflow = read(ROOT / ".github" / "workflows" / "00-pr-contracts.yml")
    if not workflow:
        return
    accessibility_index = workflow.find("python3 scripts/accessibility_audit_smoke.py")
    parity_index = workflow.find("python3 scripts/pdf_parity_registry_check.py")
    lifecycle_index = workflow.find("python3 scripts/conductor_lifecycle_check.py")
    if accessibility_index == -1 or parity_index == -1 or accessibility_index > parity_index:
        failures.append("PR Contracts must generate accessibility smoke evidence before PDF parity checks")
    if lifecycle_index == -1 or lifecycle_index > parity_index:
        failures.append("PR Contracts must run conductor lifecycle check before PDF parity checks")


def check_git_note_required() -> None:
    result = subprocess.run(
        ["git", "notes", "--ref=conductor", "show", "HEAD"],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if result.returncode != 0:
        failures.append("HEAD is missing a refs/notes/conductor git note")


def check_unpushed() -> None:
    result = subprocess.run(
        ["git", "rev-list", "--count", "origin/main..HEAD"],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if result.returncode == 0 and result.stdout.strip() not in {"", "0"}:
        failures.append(f"HEAD has {result.stdout.strip()} unpushed commits relative to origin/main")


parser = argparse.ArgumentParser()
parser.add_argument("--require-git-note", action="store_true")
parser.add_argument("--require-pushed", action="store_true")
args = parser.parse_args()

failures: list[str] = []
check_workflow_and_automation()
check_completed_archive_plans()
check_ci_ordering()
if args.require_git_note:
    check_git_note_required()
if args.require_pushed:
    check_unpushed()

if failures:
    print("CONDUCTOR LIFECYCLE CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("conductor lifecycle: ok")
