#!/usr/bin/env python3
"""Validate the Wave 0 Document IR and transformation graph contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DOC_SCHEMA = ROOT / "schemas/document-ir.schema.json"
PASS_SCHEMA = ROOT / "schemas/transformation-pass.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_ir.document_ir.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"document IR contract check failed: {message}")


def require_schema_tokens() -> None:
    doc_text = DOC_SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"custom"',
        '"annotations"',
        '"images"',
        '"form_fields"',
        '"optional_content_refs"',
        '"font_ref"',
        '"unicode_confidence"',
        '"additionalProperties": false',
    ):
        if token not in doc_text:
            fail(f"document IR schema missing token {token}")

    pass_text = PASS_SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"inputs"',
        '"outputs"',
        '"parameters"',
        '"ReadOnly"',
        '"FullSanitizingRewrite"',
        '"additionalProperties": false',
    ):
        if token not in pass_text:
            fail(f"transformation graph schema missing token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"document IR snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_ir",
        "stability": "preview",
        "phase": "AA0",
        "contract": "document_ir_transformation_graph",
        "mutation_policy": "passive_ir_only_no_parsing_rendering_or_apply_execution",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for token in (
        "DocumentIr",
        "PageIr",
        "TextSpanIr",
        "AnnotationIr",
        "ImageIr",
        "FormFieldIr",
        "TransformationGraph",
        "TransformationPassSpec",
        "PassMaturity",
        "PolicyRisk",
        "TransformationWriteMode",
    ):
        if token not in public_types:
            fail(f"snapshot missing public type {token}")


def require_schema_smoke() -> None:
    subprocess.run(
        ["python3", "scripts/ir_schema_smoke.py"],
        cwd=ROOT,
        check=True,
    )


def require_core_tests() -> None:
    subprocess.run(
        ["cargo", "test", "-q", "-p", "fe_reader_ir"],
        cwd=ROOT,
        check=True,
    )


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_schema_smoke()
    require_core_tests()
    print("document IR contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
