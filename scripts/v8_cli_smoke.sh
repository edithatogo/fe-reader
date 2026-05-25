#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping CLI smoke in this environment"
  exit 0
fi
doctor_output="$(cargo run -q -p fe_reader_cli -- doctor)"
printf '%s\n' "$doctor_output"
case "$doctor_output" in
  *"core:"*"pdf_model:"*"security:"*) ;;
  *) echo "doctor output missing crate identities" >&2; exit 1 ;;
esac

inspect_json="$(cargo run -q -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json)"
printf '%s\n' "$inspect_json"
INSPECT_JSON="$inspect_json" python3 - <<'PY'
import json
import os

payload = json.loads(os.environ["INSPECT_JSON"])
assert set(payload) == {"intent", "plan", "summary"}
assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"] == "inspect"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["intent"]["requires_review"] is False
assert payload["intent"]["document_fingerprint"] == payload["summary"]["fingerprint"]
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["risk_level"] == "read_only"
assert payload["plan"]["approved_for_apply"] is False
assert payload["plan"]["operations"] == [{"op": "noop"}]
assert payload["summary"]["parser"]["adapter"] == "lopdf"
assert payload["summary"]["parser"]["page_count"] == 1
assert payload["summary"]["parser"]["encrypted"] is False
assert payload["summary"]["parser"]["error"] is None
assert payload["summary"]["parser"]["version"] == "1.5"
assert "Root" in payload["summary"]["parser"]["trailer_keys"]
PY

plugin_policy="$(cargo run -q -p fe_reader_cli -- policy plugin)"
external_policy="$(cargo run -q -p fe_reader_cli -- policy external-tool)"
apply_policy="$(cargo run -q -p fe_reader_cli -- policy apply)"
PLUGIN_POLICY="$plugin_policy" EXTERNAL_POLICY="$external_policy" APPLY_POLICY="$apply_policy" python3 - <<'PY'
import json
import os

plugin = json.loads(os.environ["PLUGIN_POLICY"])
external = json.loads(os.environ["EXTERNAL_POLICY"])
apply = json.loads(os.environ["APPLY_POLICY"])

assert plugin["allowed"] is False
assert plugin["requires_review"] is True
assert "plugins are disabled" in plugin["reason"]

assert external["allowed"] is False
assert external["requires_review"] is True
assert "external tools are disabled" in external["reason"]

assert apply["allowed"] is True
assert apply["requires_review"] is True
PY
printf '%s\n' "$plugin_policy"
printf '%s\n' "$external_policy"
printf '%s\n' "$apply_policy"
