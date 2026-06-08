#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping redaction verification smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_redaction secure_redaction

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

recipe_path = Path("fixtures/corpus/redaction/secure-redaction-smoke.recipe.json")
schema_path = Path("schemas/redaction-recipe.schema.json")
recipe = json.loads(recipe_path.read_text(encoding="utf-8"))
assert recipe["security_level"] == "SanitizedRewrite"
assert recipe["requires_human_review"] is True
assert "no_incremental_append" in recipe["verification"]
assert recipe["regions"][0]["region"].startswith("bbox:")

if jsonschema is not None:
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    jsonschema.validate(recipe, schema)
PY

echo "redaction verification smoke passed"
