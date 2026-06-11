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
mcp_apply_policy="$(cargo run -q -p fe_reader_cli -- policy apply --source mcp)"
web_network_policy="$(cargo run -q -p fe_reader_cli -- policy network --source web)"
plugin_load_policy="$(cargo run -q -p fe_reader_cli -- policy plugin --source plugin)"
automation_policy="$(cargo run -q -p fe_reader_cli -- policy automation --source automation)"
PLUGIN_POLICY="$plugin_policy" EXTERNAL_POLICY="$external_policy" APPLY_POLICY="$apply_policy" MCP_APPLY_POLICY="$mcp_apply_policy" WEB_NETWORK_POLICY="$web_network_policy" PLUGIN_LOAD_POLICY="$plugin_load_policy" AUTOMATION_POLICY="$automation_policy" python3 - <<'PY'
import json
import os

plugin = json.loads(os.environ["PLUGIN_POLICY"])
external = json.loads(os.environ["EXTERNAL_POLICY"])
apply = json.loads(os.environ["APPLY_POLICY"])
mcp_apply = json.loads(os.environ["MCP_APPLY_POLICY"])
web_network = json.loads(os.environ["WEB_NETWORK_POLICY"])
plugin_load = json.loads(os.environ["PLUGIN_LOAD_POLICY"])
automation = json.loads(os.environ["AUTOMATION_POLICY"])

assert plugin["allowed"] is False
assert plugin["requires_review"] is True
assert "plugins are disabled" in plugin["reason"]

assert external["allowed"] is False
assert external["requires_review"] is True
assert "external tools are disabled" in external["reason"]

assert apply["allowed"] is True
assert apply["requires_review"] is True

assert mcp_apply["allowed"] is True
assert mcp_apply["requires_review"] is True
assert "requires review" in mcp_apply["reason"]

assert web_network["allowed"] is False
assert web_network["requires_review"] is True
assert "network access is disabled" in web_network["reason"]

assert plugin_load["allowed"] is False
assert plugin_load["requires_review"] is True
assert "plugins are disabled" in plugin_load["reason"]

assert automation["allowed"] is True
assert automation["requires_review"] is True
PY
printf '%s\n' "$plugin_policy"
printf '%s\n' "$external_policy"
printf '%s\n' "$apply_policy"
printf '%s\n' "$mcp_apply_policy"
printf '%s\n' "$web_network_policy"
printf '%s\n' "$plugin_load_policy"
printf '%s\n' "$automation_policy"

if cargo run -q -p fe_reader_cli -- policy read --source unknown >/tmp/fe-reader-policy-unknown.out 2>&1; then
  echo "unknown policy source must fail closed" >&2
  exit 1
fi

metadata_json="$(cargo run -q -p fe_reader_cli -- metadata fixtures/minimal/minimal.pdf --json)"
printf '%s\n' "$metadata_json"
METADATA_JSON="$metadata_json" python3 - <<'PY'
import json
import os

payload = json.loads(os.environ["METADATA_JSON"])
assert {"intent", "plan", "summary", "metadata"}.issubset(payload)
assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"] == "inspect"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["approved_for_apply"] is False
assert payload["metadata"]["parser_error"] is None
assert payload["metadata"]["xmp_metadata_present"] is False
assert "Root" in payload["metadata"]["trailer_keys"]
if "snapshot" in payload:
    assert payload["snapshot"]["snapshot_version"] == 1
    assert payload["snapshot"]["summary"] == payload["metadata"]
PY

journal_dir="$(mktemp -d "${TMPDIR:-/tmp}/fe-reader-journal.XXXXXX")"
journal_path="$journal_dir/journal.json"
journal_json="$(cargo run -q -p fe_reader_cli -- journal plan fixtures/minimal/minimal.pdf --out "$journal_path" --json)"
journal_inspect_json="$(cargo run -q -p fe_reader_cli -- journal inspect "$journal_path" --json)"
journal_recoveries_json="$(cargo run -q -p fe_reader_cli -- journal recoveries "$journal_dir" --json)"
printf '%s\n' "$journal_json"
JOURNAL_JSON="$journal_json" JOURNAL_INSPECT_JSON="$journal_inspect_json" JOURNAL_RECOVERIES_JSON="$journal_recoveries_json" JOURNAL_PATH="$journal_path" python3 - <<'PY'
import json
import os
from pathlib import Path

payload = json.loads(os.environ["JOURNAL_JSON"])
inspect_payload = json.loads(os.environ["JOURNAL_INSPECT_JSON"])
recoveries_payload = json.loads(os.environ["JOURNAL_RECOVERIES_JSON"])
path = Path(os.environ["JOURNAL_PATH"])
assert path.is_file()
assert set(payload) == {"intent", "plan", "summary", "journal", "journal_path"}
assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"] == "plan_mutation"
assert payload["intent"]["risk_level"] == "document_mutation"
assert payload["intent"]["requires_review"] is True
assert payload["plan"]["approved_for_apply"] is False
assert payload["plan"]["operations"] == [{"op": "noop"}]
entries = payload["journal"]["entries"]
assert [entry["sequence"] for entry in entries] == [0, 1]
assert entries[0]["phase"] == "intent_received"
assert entries[1]["phase"] == "plan_generated"
assert entries[1]["plan_id"] == payload["plan"]["plan_id"]
assert json.loads(path.read_text()) == payload["journal"]
assert inspect_payload["journal"] == payload["journal"]
assert inspect_payload["latest"]["phase"] == "plan_generated"
assert inspect_payload["recovery_required"] is False
assert recoveries_payload["recovery_required_count"] == 0
PY
rm -rf "$journal_dir"

search_json="$(cargo run -q -p fe_reader_cli -- search fixtures/corpus/basic/text-search-fixture.pdf Reader --json)"
printf '%s\n' "$search_json"
SEARCH_JSON="$search_json" python3 - <<'PY'
import json
import os

payload = json.loads(os.environ["SEARCH_JSON"])
assert set(payload) == {"intent", "plan", "summary", "text", "query", "index_records", "hits"}
assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"] == "search"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["approved_for_apply"] is False
assert payload["query"]["text"] == "Reader"
assert payload["text"]["extraction"]["adapter"] == "lopdf"
assert payload["text"]["extraction"]["precise_geometry"] is False
assert payload["summary"]["fingerprint"]["sha256_hex"] == "f7e2b4436614640779c890a882537d543cf4579ae6cc43ad5f43f193afa6cd7f"
assert len(payload["index_records"]) == 1
assert payload["index_records"][0]["bbox"] == [0.0, 0.0, 612.0, 792.0]
assert len(payload["hits"]) == 1
assert payload["hits"][0]["text"] == "Fe Reader Search Fixture\n"
assert payload["hits"][0]["bbox"]["width"] == 612.0
assert payload["hits"][0]["bbox"]["height"] == 792.0
assert payload["hits"][0]["char_offset"] == 3
PY
