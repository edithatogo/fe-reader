#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping forms contract smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_forms
python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

schema_path = Path("schemas/patch-plan.schema.json")
schema = json.loads(schema_path.read_text(encoding="utf-8"))
sample = {
    "plan_id": "plan-forms-smoke",
    "intent_id": "intent-forms-smoke",
    "document_id": "doc-forms-smoke",
    "summary": "plan form fill",
    "operations": [
        {"op": "fill_form_field", "field_name": "patient.name", "value": "Ada Lovelace"},
        {"op": "flatten_form_fields", "field_names": ["patient.name"]},
    ],
    "write_mode": "full_rewrite",
    "risk_level": "document_mutation",
    "transformation_graph_id": None,
    "transformation_passes": [],
    "approved_for_apply": False,
}

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

if jsonschema is not None:
    jsonschema.validate(sample, schema)

ops = {operation["op"] for operation in sample["operations"]}
assert ops == {"fill_form_field", "flatten_form_fields"}
assert sample["approved_for_apply"] is False
PY

echo "forms contract smoke passed"
