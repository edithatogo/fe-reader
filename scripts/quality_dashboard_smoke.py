#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC_DASH = ROOT / "docs/open-quality-signals-dashboard.md"
DOC_PRIVACY = ROOT / "docs/privacy-preserving-quality-signals.md"
DOC_OBS = ROOT / "docs/privacy-diagnostics-observability.md"

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
    for path in [DOC_DASH, DOC_PRIVACY, DOC_OBS]:
        if not path.exists():
            fail(f"missing docs evidence file: {path.relative_to(ROOT)}")
    for token in [
        "A serious open PDF platform should publish evidence, not claims.",
        "No headline feature claim is accepted unless it links to a fixture, test, benchmark, compatibility report, or release note.",
    ]:
        if token not in DOC_DASH.read_text(encoding="utf-8"):
            fail(f"dashboard doc missing token: {token}")
    for token in [
        "local-first",
        "privacy-sensitive release evidence",
        "no document text",
        "prohibit path collection",
        "require signed support bundles",
    ]:
        if token not in DOC_PRIVACY.read_text(encoding="utf-8"):
            fail(f"privacy quality doc missing token: {token}")
    for token in [
        "support bundle",
        "local-first",
        "explicit user or managed-policy action",
        "tracing",
    ]:
        if token not in DOC_OBS.read_text(encoding="utf-8"):
            fail(f"observability doc missing token: {token}")
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
