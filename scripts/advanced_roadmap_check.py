#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TRACKS_DIR = ROOT / "conductor" / "tracks"
TRACKS = [
    "track-AY-post-launch-pdf-baseline-parity",
    "track-AZ-mobile-public-launch",
    "track-BA-frontier-intelligence-governance",
    "track-BB-opt-in-collaboration-sync",
    "track-BC-rendering-performance-promotion",
    "track-BD-ecosystem-integrations-marketplace",
]
REQUIRED_METADATA = {
    "track_id",
    "type",
    "status",
    "created_at",
    "updated_at",
    "description",
    "launch_blocking",
    "feature_gate",
    "owner",
    "rollback_plan",
    "exit_criteria",
}
REQUIRED_TOKENS = {
    "spec.md": [
        "does not block desktop stable launch",
        "Feature gate",
        "Rollback",
        "Exit criteria",
    ],
    "plan.md": [
        "scripts/conductor_phase_gate.sh",
        "feature gate",
        "rollback",
        "exit criteria",
    ],
}


def fail(message: str) -> None:
    failures.append(message)


def read_text(path: pathlib.Path) -> str:
    if not path.exists():
        fail(f"missing file: {path.relative_to(ROOT)}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


failures: list[str] = []
registry = read_text(ROOT / "conductor" / "tracks.md")
roadmap = read_text(ROOT / "docs" / "post-launch-advanced-roadmap.md")

for track_id in TRACKS:
    directory = TRACKS_DIR / track_id
    if not directory.is_dir():
        fail(f"missing track directory: conductor/tracks/{track_id}")
        continue
    metadata_path = directory / "metadata.json"
    try:
        metadata = json.loads(read_text(metadata_path))
    except json.JSONDecodeError as exc:
        fail(f"{metadata_path.relative_to(ROOT)} invalid JSON: {exc}")
        metadata = {}
    missing = REQUIRED_METADATA - set(metadata)
    if missing:
        fail(f"{track_id} metadata missing {sorted(missing)}")
    if metadata.get("track_id") != track_id:
        fail(f"{track_id} metadata track_id mismatch")
    if metadata.get("launch_blocking") is not False:
        fail(f"{track_id} must be non-blocking for desktop stable launch")
    for field in ["feature_gate", "owner", "rollback_plan", "exit_criteria"]:
        if not metadata.get(field):
            fail(f"{track_id} metadata missing {field}")
    for filename, tokens in REQUIRED_TOKENS.items():
        text = read_text(directory / filename)
        for token in tokens:
            if token not in text:
                fail(f"{track_id}/{filename} missing token: {token}")
    if f"./tracks/{track_id}/" not in registry:
        fail(f"tracks registry missing {track_id}")
    if track_id not in roadmap:
        fail(f"post-launch roadmap doc missing {track_id}")

if "Desktop stable launch remains governed by tracks AS through AW" not in roadmap:
    fail("post-launch roadmap must state desktop launch gate relationship")
if "ML/RAG remains disabled by default" not in roadmap:
    fail("post-launch roadmap must keep ML/RAG disabled by default")
if "cloud collaboration is opt-in" not in roadmap:
    fail("post-launch roadmap must keep cloud collaboration opt-in")

report = {
    "check": "advanced_roadmap",
    "status": "fail" if failures else "pass",
    "tracks": TRACKS,
    "failures": failures,
}
(ROOT / "target" / "advanced-roadmap-check.json").parent.mkdir(parents=True, exist_ok=True)
(ROOT / "target" / "advanced-roadmap-check.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")

if failures:
    print("ADVANCED ROADMAP CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("advanced roadmap: ok")
