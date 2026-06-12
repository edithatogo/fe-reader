#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping search smoke in this environment"
  exit 0
fi

fixture="fixtures/corpus/basic/text-search-fixture.pdf"
expected="fixtures/expected/search/text-search-fixture.search-index.json"
payload="$(cargo run -q -p fe_reader_cli -- search "$fixture" Reader --json)"

SEARCH_JSON="$payload" EXPECTED_PATH="$expected" python3 - <<'PY'
import json
import os
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

payload = json.loads(os.environ["SEARCH_JSON"])
expected = json.loads(Path(os.environ["EXPECTED_PATH"]).read_text(encoding="utf-8"))
schema = json.loads(Path("schemas/search-index.schema.json").read_text(encoding="utf-8"))

if schema.get("title") != "Fe Reader Search Index Record":
    raise SystemExit("search smoke failed: schema title drifted")
if schema.get("additionalProperties") is not False:
    raise SystemExit("search smoke failed: schema must reject additional properties")
if schema.get("required") != ["document_id", "document_sha256", "page_index", "span_id", "text", "bbox"]:
    raise SystemExit("search smoke failed: required fields drifted")

assert payload["summary"]["fingerprint"]["sha256_hex"] == expected[0]["document_sha256"]
assert payload["text"]["extraction"]["adapter"] == "lopdf"
assert payload["text"]["extraction"]["precise_geometry"] is False
assert payload["text"]["extraction"]["error"] is None
assert len(payload["text"]["extraction"]["spans"]) == 1
assert payload["index_records"] == expected
assert len(payload["hits"]) == 1
assert payload["hits"][0]["text"] == expected[0]["text"]
assert payload["hits"][0]["page_index"] == expected[0]["page_index"]
assert [
    payload["hits"][0]["bbox"]["x"],
    payload["hits"][0]["bbox"]["y"],
    payload["hits"][0]["bbox"]["width"],
    payload["hits"][0]["bbox"]["height"],
] == expected[0]["bbox"]
assert payload["hits"][0]["char_offset"] == 3
assert expected[0]["reading_order"] == 0
assert len(expected[0]["bbox"]) == 4
assert expected[0]["document_id"].startswith("sha256:")

if jsonschema is not None:
    for record in expected:
        jsonschema.validate(record, schema)
PY

echo "search smoke completed"
