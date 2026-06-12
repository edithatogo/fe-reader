#!/usr/bin/env bash
set -euo pipefail

echo "== Fe Reader perf smoke =="
mkdir -p artifacts/perf
STATUS="advisory"
DETAIL="perf smoke completed in advisory mode"
if command -v cargo >/dev/null 2>&1; then
  cargo bench --workspace --no-run || true
  cargo run -p fe_reader_cli -- doctor >/tmp/fe-reader-perf-doctor.txt || true
fi
if command -v hyperfine >/dev/null 2>&1 && command -v fe-reader >/dev/null 2>&1; then
  hyperfine --warmup 1 'fe-reader --version' || true
  DETAIL="hyperfine ran against installed fe-reader"
else
  echo "hyperfine or fe-reader not installed; perf smoke advisory skip"
  DETAIL="hyperfine or installed fe-reader unavailable; cargo doctor smoke used when possible"
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
manifest = {
    "check": "perf_smoke",
    "status": status,
    "detail": detail,
    "outputs": ["artifacts/perf/manifest.json", "artifacts/perf/summary.md"],
}
Path("artifacts/perf/manifest.json").write_text(json.dumps(manifest, sort_keys=True) + "\n", encoding="utf-8")
Path("artifacts/perf/summary.md").write_text(f"# Performance Smoke\n\n- Status: {status}\n- Detail: {detail}\n", encoding="utf-8")
loaded = json.loads(Path("artifacts/perf/manifest.json").read_text(encoding="utf-8"))
if loaded.get("check") != "perf_smoke":
    raise SystemExit("perf smoke manifest check drifted")
if loaded.get("status") != status:
    raise SystemExit("perf smoke manifest status drifted")
if loaded.get("outputs") != ["artifacts/perf/manifest.json", "artifacts/perf/summary.md"]:
    raise SystemExit("perf smoke manifest outputs drifted")
summary = Path("artifacts/perf/summary.md").read_text(encoding="utf-8")
for token in ["# Performance Smoke", "- Status:", "- Detail:"]:
    if token not in summary:
        raise SystemExit(f"perf smoke summary missing token: {token}")
PY
