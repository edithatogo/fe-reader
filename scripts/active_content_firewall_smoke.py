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
    if finding["finding_id"] != "active-content:javascript-action:smoke":
        fail("active content finding id drifted")
    if finding["kind"] != "JavaScriptAction":
        fail("active content kind drifted")
    if finding["risk"] not in {"High", "Critical"}:
        fail("JavaScript active content smoke must be high risk")
    if finding["default_action"] not in {"DisableByDefault", "RequireUserApproval", "Block"}:
        fail("active content must not be executable by default")
    if finding["default_action"] != "DisableByDefault":
        fail("JavaScript active content smoke must default to disabled")
    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/active-content-finding.schema.json").read_text())
        if schema.get("title") != "Fe Reader Active Content Finding":
            fail("active content schema title drifted")
        if schema.get("additionalProperties") is not False:
            fail("active content schema must reject additional properties")
        if schema.get("required") != ["finding_id", "kind", "risk", "default_action"]:
            fail("active content schema required fields drifted")
        if schema.get("properties", {}).get("risk", {}).get("enum") != ["Low", "Medium", "High", "Critical"]:
            fail("active content risk enum drifted")
        if schema.get("properties", {}).get("default_action", {}).get("enum") != [
            "AllowReadOnly",
            "DisableByDefault",
            "RequireUserApproval",
            "Block",
        ]:
            fail("active content default action enum drifted")
        jsonschema.validate(finding, schema)
    print("active content firewall smoke passed")


if __name__ == "__main__":
    main()
