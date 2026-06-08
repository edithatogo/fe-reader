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

assert payload["summary"]["fingerprint"]["sha256_hex"] == expected[0]["document_sha256"]
assert payload["text"]["extraction"]["adapter"] == "lopdf"
assert payload["text"]["extraction"]["precise_geometry"] is False
assert payload["text"]["extraction"]["error"] is None
assert len(payload["text"]["extraction"]["spans"]) == 1
assert len(payload["hits"]) == 1
assert payload["hits"][0]["text"] == expected[0]["text"]
assert payload["hits"][0]["page_index"] == expected[0]["page_index"]
assert payload["hits"][0]["char_offset"] == 3

if jsonschema is not None:
    schema = json.loads(Path("schemas/search-index.schema.json").read_text(encoding="utf-8"))
    for record in expected:
        jsonschema.validate(record, schema)
PY

echo "search smoke completed"
