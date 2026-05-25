#!/usr/bin/env python3
"""Validate the Wave 0 core OperationIntent JSON contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/operation-intent.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.operation_intent.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"operation intent contract check failed: {message}")


def validate_with_jsonschema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def require_cli_intent() -> None:
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
    intent = payload.get("intent")
    if not isinstance(intent, dict):
        fail("CLI inspect output missing intent object")

    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    validate_with_jsonschema(intent, schema)

    expected = {
        "source": "cli",
        "kind": "inspect",
        "label": "inspect",
        "risk_level": "read_only",
        "requires_review": False,
    }
    for key, value in expected.items():
        if intent.get(key) != value:
            fail(f"CLI inspect intent {key} expected {value!r}, got {intent.get(key)!r}")

    if intent.get("document_fingerprint") != payload.get("summary", {}).get("fingerprint"):
        fail("CLI inspect intent fingerprint must match inspected document summary")


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        "document_mutation",
        "high_risk",
        "requires_review",
        "document_fingerprint",
    ):
        if token not in schema_text:
            fail(f"operation intent schema missing token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"operation intent snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_core",
        "stability": "preview",
        "phase": "A1",
        "contract": "operation_intent",
        "mutation_policy": "intent_to_plan_only_until_review_apply_verify_receipt",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"operation intent snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for type_name in ("OperationIntent", "OperationSource", "OperationKind", "RiskLevel"):
        if type_name not in public_types:
            fail(f"operation intent snapshot missing public type {type_name}")

    constructors = set(snapshot.get("constructors", []))
    for constructor in (
        "OperationIntent::new",
        "OperationIntent::read_only",
        "OperationIntent::mutation",
        "OperationIntent::high_risk",
    ):
        if constructor not in constructors:
            fail(f"operation intent snapshot missing constructor {constructor}")


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_cli_intent()
    print("operation intent contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
