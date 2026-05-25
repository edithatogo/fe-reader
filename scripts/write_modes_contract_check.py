#!/usr/bin/env python3
"""Validate the Wave 0 write-mode policy contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/patch-plan.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.write_modes.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"write modes contract check failed: {message}")


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"no_write"',
        '"incremental_append"',
        '"full_rewrite"',
        '"sanitizing_rewrite"',
        '"redact_region"',
        '"place_stamp"',
        'metadata_scrub_mode',
        '"delete_pages"',
    ):
        if token not in schema_text:
            fail(f"patch plan schema missing write-mode token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"write modes snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_core",
        "stability": "preview",
        "phase": "A5",
        "contract": "write_modes",
        "mutation_policy": "write_mode_is_derived_from_planned_operations",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"write modes snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    policy = snapshot.get("policy", {})
    expected_policy = {
        "noop": "no_write",
        "place_stamp": "incremental_append",
        "set_metadata": "incremental_append",
        "metadata_scrub_mode": "sanitizing_rewrite",
        "redact_region": "sanitizing_rewrite",
        "delete_pages": "full_rewrite",
        "rotate_pages": "full_rewrite",
        "reorder_pages": "full_rewrite",
    }
    for key, value in expected_policy.items():
        if policy.get(key) != value:
            fail(f"write mode policy {key} expected {value!r}, got {policy.get(key)!r}")


def require_core_tests() -> None:
    for test_filter in ("write_mode", "high_risk_redaction", "mutating_patch_plan"):
        subprocess.run(
            [
                "cargo",
                "test",
                "-q",
                "-p",
                "fe_reader_core",
                test_filter,
            ],
            cwd=ROOT,
            check=True,
        )


def require_metadata_tests() -> None:
    subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "-p",
            "fe_reader_metadata",
            "metadata_write_mode",
        ],
        cwd=ROOT,
        check=True,
    )


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_core_tests()
    require_metadata_tests()
    print("write modes contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
