#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None


def fail(message: str) -> None:
    print(f"quality dashboard smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    subprocess.run(
        ["cargo", "test", "-q", "-p", "fe_reader_quality_dashboard", "--all-targets"],
        cwd=ROOT,
        check=True,
    )
    dashboard = {
        "dashboard_version": "0.1.0",
        "generated_at": "1970-01-01T00:00:00Z",
        "reports": [
            {
                "kind": "schemas",
                "status": "pass",
                "artifact": "scripts/validate_schemas.py",
            },
            {
                "kind": "search_index",
                "status": "pass",
                "artifact": "scripts/search_index_smoke.sh",
            },
        ],
    }
    if not all(report["artifact"] for report in dashboard["reports"]):
        fail("all report artifacts must be non-empty local paths")
    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/public-quality-dashboard.schema.json").read_text())
        jsonschema.validate(dashboard, schema)
    print("quality dashboard smoke passed")


if __name__ == "__main__":
    main()
