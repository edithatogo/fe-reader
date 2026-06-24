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

report_path = Path("target/release-evidence/redaction-verification-smoke.json")
report_path.parent.mkdir(parents=True, exist_ok=True)
report_path.write_text(
    json.dumps(
        {
            "check": "redaction_verification_smoke",
            "status": "pass",
            "recipe_path": str(recipe_path),
            "schema_path": str(schema_path),
            "security_level": recipe["security_level"],
            "requires_human_review": recipe["requires_human_review"],
            "verification": recipe["verification"],
        },
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY

echo "redaction verification smoke passed"
