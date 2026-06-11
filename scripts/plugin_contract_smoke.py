#!/usr/bin/env python3
"""Validate proposal-only plugin manifest and ABI policy."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/plugin-manifest.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/plugin-abi/fe_reader_plugin_host.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"plugin contract smoke failed: {message}")


def validate_schema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def main() -> int:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    manifest = {
        "plugin_id": "org.fereader.sample",
        "plugin_version": "0.1.0",
        "fe_plugin_api": "0.1-preview",
        "publisher": "Fe Reader Project",
        "license": "Apache-2.0 OR MIT",
        "capabilities": ["propose_annotations"],
        "network_access": False,
        "filesystem_access": False,
        "sha256": None,
        "signature": None,
    }
    validate_schema(manifest, schema)
    if manifest["network_access"] or manifest["filesystem_access"]:
        fail("proposal-only plugin manifest must not request network or filesystem access")

    snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    if snapshot.get("abi") != "proposal_only":
        fail("plugin ABI must remain proposal_only")
    if snapshot.get("mutation_policy") != "plugins_propose_patch_plans_only":
        fail("plugin mutation policy changed")
    denied = set(snapshot.get("denied_by_default", []))
    for token in ("network_access", "filesystem_access", "direct_apply", "audit_receipt_emission_by_plugin"):
        if token not in denied:
            fail(f"plugin ABI snapshot must deny {token}")

    guards = set(snapshot.get("required_mutation_guards", []))
    for token in (
        "document_hash_match",
        "patch_plan_id",
        "policy_evaluation",
        "approval_token_or_interactive_confirmation",
        "audit_receipt_emission",
    ):
        if token not in guards:
            fail(f"plugin ABI snapshot missing mutation guard {token}")

    subprocess.run(["cargo", "test", "-q", "-p", "fe_reader_plugin_host"], cwd=ROOT, check=True)
    print("plugin contract smoke passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
