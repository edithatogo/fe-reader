#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping CLI smoke in this environment"
  exit 0
fi
cargo run -p fe_reader_cli -- doctor
inspect_json="$(cargo run -q -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json)"
printf '%s\n' "$inspect_json"
INSPECT_JSON="$inspect_json" python3 - <<'PY'
import json
import os

payload = json.loads(os.environ["INSPECT_JSON"])
assert payload["plan"]["write_mode"] == "no_write"
assert payload["summary"]["parser"]["adapter"] == "lopdf"
assert payload["summary"]["parser"]["page_count"] == 1
PY
cargo run -p fe_reader_cli -- policy plugin
cargo run -p fe_reader_cli -- policy external-tool
