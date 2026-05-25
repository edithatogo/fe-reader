#!/usr/bin/env python3
"""Validate the Wave 0 core PatchPlan JSON contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/patch-plan.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.patch_plan.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"patch plan contract check failed: {message}")


def validate_with_jsonschema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def require_cli_plan() -> None:
    output = subprocess.check_output(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "inspect",
            "fixtures/minimal/minimal.pdf",
            "--json",
        ],
        cwd=ROOT,
        text=True,
    )
    payload = json.loads(output)
    plan = payload.get("plan")
    if not isinstance(plan, dict):
        fail("CLI inspect output missing plan object")

    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    validate_with_jsonschema(plan, schema)

    expected = {
        "write_mode": "no_write",
        "risk_level": "read_only",
        "approved_for_apply": False,
        "operations": [{"op": "noop"}],
    }
    for key, value in expected.items():
        if plan.get(key) != value:
            fail(f"CLI inspect plan {key} expected {value!r}, got {plan.get(key)!r}")

    if plan.get("intent_id") != payload.get("intent", {}).get("intent_id"):
        fail("CLI inspect plan must reference the emitted intent id")
    if plan.get("document_id") != payload.get("intent", {}).get("document_id"):
        fail("CLI inspect plan must reference the emitted document id")


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        "approved_for_apply",
        "no_write",
        "full_rewrite",
        "document_mutation",
        "redact_region",
    ):
        if token not in schema_text:
            fail(f"patch plan schema missing token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"patch plan snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_core",
        "stability": "preview",
        "phase": "A2",
        "contract": "patch_plan",
        "mutation_policy": "draft_plans_are_unapproved_until_policy_review",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"patch plan snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for type_name in ("PatchPlan", "PatchOperation", "WriteMode", "OperationIntent"):
        if type_name not in public_types:
            fail(f"patch plan snapshot missing public type {type_name}")


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_cli_plan()
    print("patch plan contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
