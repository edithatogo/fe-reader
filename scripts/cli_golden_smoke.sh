#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping CLI golden smoke in this environment"
  exit 0
fi

inspect_json="$(cargo run -q -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json)"
scrub_json="$(cargo run -q -p fe_reader_cli -- metadata-scrub fixtures/minimal/minimal.pdf --profile clean-share --plan-only --json)"
text_map_json="$(cargo run -q -p fe_reader_cli -- lab text-map fixtures/corpus/basic/text-search-fixture.pdf --page 0 --json)"

INSPECT_JSON="$inspect_json" SCRUB_JSON="$scrub_json" TEXT_MAP_JSON="$text_map_json" python3 - <<'PY'
from __future__ import annotations

import json
import os
from pathlib import Path

expected = json.loads(Path("fixtures/expected/cli/wave2-golden.json").read_text(encoding="utf-8"))


def require_subset(actual: object, expected_subset: object, path: str = "$") -> None:
    if isinstance(expected_subset, dict):
        if not isinstance(actual, dict):
            raise AssertionError(f"{path} expected object")
        for key, value in expected_subset.items():
            if key not in actual:
                raise AssertionError(f"{path}.{key} missing")
            require_subset(actual[key], value, f"{path}.{key}")
        return
    if isinstance(expected_subset, list):
        if actual != expected_subset:
            raise AssertionError(f"{path} expected {expected_subset!r}, got {actual!r}")
        return
    if actual != expected_subset:
        raise AssertionError(f"{path} expected {expected_subset!r}, got {actual!r}")


inspect = json.loads(os.environ["INSPECT_JSON"])
scrub = json.loads(os.environ["SCRUB_JSON"])
text_map = json.loads(os.environ["TEXT_MAP_JSON"])

require_subset(inspect, expected["inspect_minimal"], "$.inspect_minimal")
require_subset(scrub, expected["metadata_scrub_clean_share"], "$.metadata_scrub_clean_share")
require_subset(text_map, expected["lab_text_map_basic"], "$.lab_text_map_basic")

assert text_map["text_map"]["document_sha256"] == text_map["summary"]["fingerprint"]["sha256_hex"]
assert text_map["text_map"]["content_streams"]
assert text_map["text_map"]["content_streams"][0]["byte_len"] > 0
assert text_map["text_map"]["text_map"]
assert text_map["text_map"]["text_map"][0]["text"] == "Fe Reader Search Fixture\n"
assert text_map["text_map"]["text_map"][0]["geometry_confidence"] == "page_fallback"
assert any(
    finding["code"] == "content_stream_text_map_smoke"
    for finding in text_map["text_map"]["findings"]
)
PY

echo "CLI golden smoke passed"
