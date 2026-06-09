#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping metadata Wave 2 smoke in this environment"
  exit 0
fi

fixture="fixtures/minimal/minimal.pdf"

metadata_json="$(cargo run -q -p fe_reader_cli -- metadata "$fixture" --json)"
diff_json="$(cargo run -q -p fe_reader_cli -- metadata-diff "$fixture" "$fixture" --json)"
scrub_json="$(cargo run -q -p fe_reader_cli -- metadata-scrub "$fixture" --profile clean-share --plan-only --json)"

METADATA_JSON="$metadata_json" DIFF_JSON="$diff_json" SCRUB_JSON="$scrub_json" python3 - <<'PY'
from __future__ import annotations

import json
import os

metadata = json.loads(os.environ["METADATA_JSON"])
assert metadata["intent"]["risk_level"] == "read_only"
assert metadata["plan"]["write_mode"] == "no_write"
assert metadata["plan"]["approved_for_apply"] is False
assert metadata["snapshot"]["snapshot_version"] == 1
assert metadata["snapshot"]["summary"] == metadata["metadata"]

diff = json.loads(os.environ["DIFF_JSON"])
assert diff["intent"]["risk_level"] == "read_only"
assert diff["plan"]["write_mode"] == "no_write"
assert diff["plan"]["approved_for_apply"] is False
assert diff["diff"]["before"]["snapshot_version"] == 1
assert diff["diff"]["after"]["snapshot_version"] == 1
assert diff["diff"]["changes"] == []

scrub = json.loads(os.environ["SCRUB_JSON"])
assert scrub["intent"]["risk_level"] == "document_mutation"
assert scrub["plan"]["write_mode"] == "sanitizing_rewrite"
assert scrub["plan"]["approved_for_apply"] is False
assert scrub["plan_only"] is True
assert scrub["plan"]["operations"] == [
    {"op": "set_metadata", "key": "metadata_scrub_mode", "value": "clean_share"}
]
PY

if cargo run -q -p fe_reader_cli -- metadata-scrub "$fixture" --profile clean-share >/tmp/fe-reader-metadata-scrub-no-plan-only.out 2>&1; then
  echo "metadata-scrub without --plan-only must fail in Wave 2" >&2
  exit 1
fi

cargo test -q -p fe_reader_metadata metadata_

echo "metadata Wave 2 smoke passed"
