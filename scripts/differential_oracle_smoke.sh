#!/usr/bin/env bash
set -euo pipefail

mkdir -p target/oracle-reports

cargo test -q -p fe_reader_compat --all-targets

cat > target/oracle-reports/wave1-render-smoke.json <<'JSON'
{
  "run_id": "wave1-render-smoke",
  "fixture_id": "basic-text-search-fixture",
  "operation": "render_visual_similarity",
  "tools": [
    {
      "tool": "external-render-oracle",
      "version": null,
      "exit_code": null,
      "normalized_output_sha256": null,
      "warnings": [
        "external rendering oracle not configured; null render contract used for smoke"
      ]
    }
  ],
  "comparison": {
    "status": "oracle_unavailable",
    "disagreement_class": "known_feature_gap",
    "notes": [
      "Wave 1 captures fixture-linked oracle report shape before enabling external tools"
    ]
  }
}
JSON

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

root = Path.cwd()
report_path = root / "target/oracle-reports/wave1-render-smoke.json"
schema_path = root / "schemas/differential-test-report.schema.json"
report = json.loads(report_path.read_text(encoding="utf-8"))
schema = json.loads(schema_path.read_text(encoding="utf-8"))
if jsonschema is not None:
    jsonschema.validate(report, schema)
assert report["fixture_id"] == "basic-text-search-fixture"
assert report["operation"] == "render_visual_similarity"
assert report["tools"], "oracle smoke must include at least one tool result"
assert report["comparison"]["status"] == "oracle_unavailable"
PY

echo "differential oracle smoke: target/oracle-reports/wave1-render-smoke.json"
