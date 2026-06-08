#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None


def fail(message: str) -> None:
    print(f"active content firewall smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    finding = {
        "finding_id": "active-content:javascript-action:smoke",
        "kind": "JavaScriptAction",
        "risk": "High",
        "default_action": "DisableByDefault",
        "object_ref": "1 0 R",
    }
    if finding["default_action"] not in {"DisableByDefault", "RequireUserApproval", "Block"}:
        fail("active content must not be executable by default")
    if finding["kind"] == "JavaScriptAction" and finding["risk"] not in {"High", "Critical"}:
        fail("JavaScript active content smoke must be high risk")
    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/active-content-finding.schema.json").read_text())
        jsonschema.validate(finding, schema)
    print("active content firewall smoke passed")


if __name__ == "__main__":
    main()
