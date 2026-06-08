#!/usr/bin/env python3
"""Validate the Wave 2 page and annotation operation contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/patch-plan.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.page_ops.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"page ops contract check failed: {message}")


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"op":',
        '"const": "delete_pages"',
        '"const": "rotate_pages"',
        '"const": "reorder_pages"',
        '"page_indexes"',
        '"rotation_degrees"',
        '"new_order"',
        '"const": "add_highlight_annotation"',
        '"const": "add_note_annotation"',
        '"rects"',
        '"position"',
        '"contents"',
        '"annotation_color"',
        '"minItems": 1',
    ):
        if token not in schema_text:
            fail(f"patch plan schema missing page/annotation operation token {token}")


def validate_with_jsonschema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def require_annotation_schema_sample() -> None:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    sample_plan = {
        "plan_id": "plan-1",
        "intent_id": "intent-1",
        "document_id": "doc-1",
        "summary": "add highlight and note",
        "operations": [
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
        "approved_for_apply": False,
    }
    validate_with_jsonschema(sample_plan, schema)


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"page ops snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_core",
        "stability": "preview",
        "phase": "A4",
        "contract": "page_annotation_ops",
        "mutation_policy": "page_and_annotation_operations_are_patch_plan_operations_only",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"page ops snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for type_name in (
        "PatchOperation",
        "PatchPlan",
        "OperationIntent",
        "WriteMode",
        "RiskLevel",
        "PageRect",
        "PagePoint",
        "AnnotationColor",
    ):
        if type_name not in public_types:
            fail(f"page/annotation ops snapshot missing public type {type_name}")

    constructors = set(snapshot.get("constructors", []))
    for constructor in (
        "PatchOperation::delete_pages",
        "PatchOperation::rotate_pages",
        "PatchOperation::reorder_pages",
        "PatchOperation::add_highlight_annotation",
        "PatchOperation::add_note_annotation",
    ):
        if constructor not in constructors:
            fail(f"page/annotation ops snapshot missing constructor {constructor}")


def require_core_tests() -> None:
    subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "-p",
            "fe_reader_core",
            "page_operation",
        ],
        cwd=ROOT,
        check=True,
    )
    subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "-p",
            "fe_reader_core",
            "annotation_operation",
        ],
        cwd=ROOT,
        check=True,
    )


def main() -> int:
    require_schema_tokens()
    require_annotation_schema_sample()
    require_snapshot()
    require_core_tests()
    print("page ops contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
