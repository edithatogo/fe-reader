#!/usr/bin/env python3
"""Smoke-check Wave 3 workflow-pack JSON and plan-only Rust behavior."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/workflow-pack.schema.json"
PACKS = [
    ROOT / "templates/workflows/legal/affidavit.initials.every-page.json",
    ROOT / "templates/workflows/healthcare/deidentify.basic.json",
    ROOT / "templates/workflows/government/foi.redaction.exemption-tags.json",
    ROOT / "templates/workflows/research/highlights.to-markdown-zotero.json",
    ROOT / "templates/workflows/publishing/pdfa-preflight.json",
]


def fail(message: str) -> None:
    raise SystemExit(f"workflow pack smoke failed: {message}")


def validate_json() -> None:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    try:
        import jsonschema  # type: ignore
    except Exception:
        jsonschema = None

    seen_ids: set[str] = set()
    families: set[str] = set()
    for path in PACKS:
        payload = json.loads(path.read_text(encoding="utf-8"))
        if jsonschema is not None:
            jsonschema.validate(instance=payload, schema=schema)
        workflow_id = payload["workflow_id"]
        if workflow_id in seen_ids:
            fail(f"duplicate workflow_id {workflow_id}")
        seen_ids.add(workflow_id)
        families.add(payload["workflow_family"])
        if not payload["steps"]:
            fail(f"{workflow_id} has no steps")

    expected_families = {"legal", "healthcare", "government", "research", "publishing"}
    if families != expected_families:
        fail(f"families expected {sorted(expected_families)}, got {sorted(families)}")


def run_rust_tests() -> None:
    subprocess.run(
        ["cargo", "test", "-q", "-p", "fe_reader_workflows"],
        cwd=ROOT,
        check=True,
    )


def main() -> int:
    validate_json()
    run_rust_tests()
    print("workflow pack smoke passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
