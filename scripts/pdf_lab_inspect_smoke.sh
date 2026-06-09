#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping PDF lab inspect smoke in this environment"
  exit 0
fi

payload="$(cargo run -q -p fe_reader_cli -- lab inspect fixtures/minimal/minimal.pdf --json)"
printf '%s\n' "$payload"

LAB_JSON="$payload" python3 - <<'PY'
from __future__ import annotations

import json
import os
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

payload = json.loads(os.environ["LAB_JSON"])
lab = payload["lab"]

assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"]["custom"] == "lab_inspect"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["intent"]["requires_review"] is False
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["approved_for_apply"] is False
assert payload["plan"]["operations"] == [{"op": "noop"}]
assert lab["mode"] == "object_page_graph"
assert lab["document_sha256"] == payload["summary"]["fingerprint"]["sha256_hex"]
assert lab["error"] is None
assert lab["object_count"] > 0
assert len(lab["pages"]) == 1
assert lab["pages"][0]["effective_box"]["width"] > 0
assert lab["pages"][0]["effective_box"]["height"] > 0
assert any(finding["code"] == "object_page_graph_smoke" for finding in lab["findings"])

if jsonschema is not None:
    schema = json.loads(Path("schemas/pdf-lab-session.schema.json").read_text(encoding="utf-8"))
    jsonschema.validate(lab, schema)
PY

echo "PDF lab inspect smoke completed"
