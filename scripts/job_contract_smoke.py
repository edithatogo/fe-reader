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
    print(f"job contract smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    subprocess.run(
        ["cargo", "test", "-q", "-p", "fe_reader_jobs", "--all-targets"],
        cwd=ROOT,
        check=True,
    )
    sample = {
        "job_id": "job:wave0-smoke",
        "kind": "contract_smoke",
        "state": "Completed",
        "progress": {
            "completed_units": 1,
            "total_units": 1,
            "message": "contract smoke completed",
        },
        "resource_limits": {
            "max_wall_time_ms": 30000,
            "max_memory_mib": 1024,
        },
    }
    if sample["progress"]["completed_units"] > sample["progress"]["total_units"]:
        fail("sample progress is inconsistent")
    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/job-run.schema.json").read_text())
        jsonschema.validate(sample, schema)
    print("job contract smoke passed")


if __name__ == "__main__":
    main()
