#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs/job-scheduler-power-thermal.md"

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
    doc = DOC.read_text(encoding="utf-8")
    for token in [
        "JobSpec -> queued -> running -> cancelling | paused | completed | failed -> retained receipt",
        "interactive latency",
        "BatterySaver",
        "schemas/job-run.schema.json",
        "schemas/power-budget.schema.json",
    ]:
        if token not in doc:
            fail(f"job scheduler doc missing token: {token}")
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
        power_schema = json.loads((ROOT / "schemas/power-budget.schema.json").read_text())
        jsonschema.validate(
            {
                "scenario_id": "smoke",
                "platform": "local",
                "mode": "Balanced",
                "max_peak_memory_mb": 1024,
                "max_cpu_percent_avg": 50,
                "max_wall_time_ms": 30000,
            },
            power_schema,
        )
    print("job contract smoke passed")


if __name__ == "__main__":
    main()
