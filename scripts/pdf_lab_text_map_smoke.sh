#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping PDF lab text-map smoke in this environment"
  exit 0
fi

payload="$(cargo run -q -p fe_reader_cli -- lab text-map fixtures/corpus/basic/text-search-fixture.pdf --page 0 --json)"
printf '%s\n' "$payload"

TEXT_MAP_JSON="$payload" python3 - <<'PY'
from __future__ import annotations

import json
import os

payload = json.loads(os.environ["TEXT_MAP_JSON"])
text_map = payload["text_map"]

assert payload["intent"]["source"] == "cli"
assert payload["intent"]["kind"]["custom"] == "lab_text_map"
assert payload["intent"]["risk_level"] == "read_only"
assert payload["intent"]["requires_review"] is False
assert payload["plan"]["write_mode"] == "no_write"
assert payload["plan"]["approved_for_apply"] is False
assert payload["plan"]["operations"] == [{"op": "noop"}]
assert text_map["mode"] == "text_map"
assert text_map["document_sha256"] == payload["summary"]["fingerprint"]["sha256_hex"]
assert text_map["error"] is None
assert text_map["page_index"] == 0
assert len(text_map["content_streams"]) == 1
assert text_map["content_streams"][0]["byte_len"] > 0
assert len(text_map["content_streams"][0]["sha256_hex"]) == 64
assert len(text_map["text_map"]) == 1
assert text_map["text_map"][0]["text"] == "Fe Reader Search Fixture\n"
assert text_map["text_map"][0]["geometry_confidence"] == "page_fallback"
assert any(finding["code"] == "content_stream_text_map_smoke" for finding in text_map["findings"])
PY

echo "PDF lab text-map smoke completed"
