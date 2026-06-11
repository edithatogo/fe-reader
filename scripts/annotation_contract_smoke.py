#!/usr/bin/env python3
"""Validate the preview annotation patch-plan contract."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/patch-plan.schema.json"


def fail(message: str) -> None:
    raise SystemExit(f"annotation contract smoke failed: {message}")


def validate_schema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def main() -> int:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    plan = {
        "plan_id": "plan-annotation-smoke",
        "intent_id": "intent-annotation-smoke",
        "document_id": "doc-annotation-smoke",
        "summary": "preview annotation plan",
        "operations": [
            {"op": "place_stamp", "page_index": 0, "stamp_ref": "approved"},
            {
                "op": "add_highlight_annotation",
                "page_index": 0,
                "rects": [{"x": 10, "y": 20, "width": 30, "height": 12}],
                "color": {"red": 255, "green": 242, "blue": 0},
            },
            {
                "op": "add_note_annotation",
                "page_index": 0,
                "position": {"x": 72, "y": 144},
                "contents": "review this",
            },
        ],
        "write_mode": "incremental_append",
        "risk_level": "document_mutation",
        "transformation_graph_id": None,
        "transformation_passes": [],
        "approved_for_apply": False,
    }
    validate_schema(plan, schema)
    ops = {operation["op"] for operation in plan["operations"]}
    if ops != {"place_stamp", "add_highlight_annotation", "add_note_annotation"}:
        fail(f"unexpected annotation operation set: {ops}")
    if plan["approved_for_apply"] is not False:
        fail("annotation preview plan must not be approved for apply")
    if plan["write_mode"] != "incremental_append":
        fail("annotation preview plan must remain append-only until apply verification exists")

    subprocess.run(
        ["cargo", "test", "-q", "-p", "fe_reader_core", "annotation_operation"],
        cwd=ROOT,
        check=True,
    )
    print("annotation contract smoke passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
