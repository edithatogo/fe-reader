#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping Wave 1 render smoke in this environment"
  exit 0
fi

render_json="$(cargo run -q -p fe_reader_cli -- render-tile fixtures/corpus/basic/text-search-fixture.pdf --page 0 --x 0 --y 0 --width 16 --height 12 --scale 1 --json)"
printf '%s\n' "$render_json"
RENDER_JSON="$render_json" python3 - <<'PY'
import json
import os
from pathlib import Path

payload = json.loads(os.environ["RENDER_JSON"])
expected = json.loads(Path("fixtures/expected/rendered/wave1-render-tile/metadata.json").read_text())

assert set(payload) == {"intent", "plan", "summary", "render"}
assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"] == "render"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["intent"]["requires_review"] is False
assert payload["intent"]["document_fingerprint"] == payload["summary"]["fingerprint"]
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["risk_level"] == "read_only"
assert payload["plan"]["approved_for_apply"] is False
assert payload["plan"]["operations"] == [{"op": "noop"}]

render = payload["render"]
tile = render["tile"]
request = render["request"]
assert render["status"] == expected["status"]
assert render["backend"] == expected["backend"]
assert request["document_ref"] == f"sha256:{expected['sha256_hex']}"
assert request["page_index"] == expected["page_index"]
assert request["tile_rect"] == expected["tile_rect"]
assert request["scale"] == expected["scale"]
assert request["rotation_degrees"] == 0
assert request["color_mode"] == "normal"
assert request["acceleration"] == "cpu_only"
assert tile["width"] == expected["width"]
assert tile["height"] == expected["height"]
assert tile["pixel_format"] == expected["pixel_format"]
assert tile["byte_len"] == expected["byte_len"]
assert tile["all_zero"] is True
assert expected["sha256_hex"] in tile["cache_key"]
assert payload["summary"]["fingerprint"]["sha256_hex"] == expected["sha256_hex"]
assert payload["summary"]["parser"]["adapter"] == "lopdf"
PY
