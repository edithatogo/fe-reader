#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
baseline_path = root / "fixtures/expected/prepress/text-search-fixture.prepress.json"
target_path = root / "target/prepress-reports/text-search-fixture.prepress.json"
schema_path = root / "schemas/color-prepress-report.schema.json"

baseline = json.loads(baseline_path.read_text(encoding="utf-8"))

required = ("document_id", "output_intents", "colour_findings", "font_findings", "page_box_findings")
missing = [key for key in required if key not in baseline]
if missing:
    raise SystemExit(f"prepress smoke failure: missing keys {missing}")
if baseline["output_intents"] != []:
    raise SystemExit("prepress smoke failure: Wave2 baseline must not claim output intent support")
if not any(finding.get("category") == "device_rgb_placeholder" for finding in baseline["colour_findings"]):
    raise SystemExit("prepress smoke failure: expected explicit placeholder colour finding")
if not baseline["font_findings"]:
    raise SystemExit("prepress smoke failure: expected at least one font finding")
if not baseline["page_box_findings"]:
    raise SystemExit("prepress smoke failure: expected at least one page box finding")

page_box = baseline["page_box_findings"][0]
media_box = page_box.get("media_box", {})
for key in ("x0", "y0", "x1", "y1"):
    if not isinstance(media_box.get(key), int | float):
        raise SystemExit(f"prepress smoke failure: media_box.{key} must be numeric")
if media_box["x1"] <= media_box["x0"] or media_box["y1"] <= media_box["y0"]:
    raise SystemExit("prepress smoke failure: media box must be non-empty")
if page_box.get("crop_box", {}).get("source") != "defaults_to_media_box":
    raise SystemExit("prepress smoke failure: expected conservative crop box fallback")

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

if jsonschema is not None:
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    jsonschema.validate(baseline, schema)

target_path.parent.mkdir(parents=True, exist_ok=True)
target_path.write_text(json.dumps(baseline, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"prepress smoke: {target_path.relative_to(root)}")
PY
