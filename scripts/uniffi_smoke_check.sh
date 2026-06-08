#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping UniFFI smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_uniffi --all-targets
cargo build -q -p fe_reader_uniffi

out_dir="target/uniffi-smoke"
mkdir -p "$out_dir"

if command -v uniffi-bindgen >/dev/null 2>&1; then
  lib_path="target/debug/libfe_reader_uniffi.dylib"
  if [[ ! -f "$lib_path" ]]; then
    lib_path="target/debug/libfe_reader_uniffi.so"
  fi
  if [[ ! -f "$lib_path" ]]; then
    lib_path="target/debug/fe_reader_uniffi.dll"
  fi
  if [[ ! -f "$lib_path" ]]; then
    echo "UniFFI library artifact not found after build" >&2
    exit 1
  fi
  uniffi-bindgen generate --library "$lib_path" --language swift --out-dir "$out_dir/swift"
  uniffi-bindgen generate --library "$lib_path" --language python --out-dir "$out_dir/python"
  uniffi-bindgen generate --library "$lib_path" --language ruby --out-dir "$out_dir/ruby"
else
  echo "uniffi-bindgen not installed; validating existing target/uniffi-smoke outputs"
fi

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
snapshot = json.loads((root / "contracts/snapshots/uniffi/fe_reader_uniffi.facade.json").read_text())
assert snapshot["surface"] == "uniffi"
assert snapshot["mutation_policy"] == "read_only_or_plan_only"
assert "swift" in snapshot["languages"]
assert "python" in snapshot["languages"]
assert "ruby" in snapshot["languages"]
facade = snapshot["facades"][0]
functions = {entry["name"]: entry["policy"] for entry in facade["functions"]}
assert functions["binding_info"] == "read_only"
assert functions["create_read_only_intent"] == "read_only"
assert functions["draft_noop_plan"] == "plan_only_no_write"

expected = [
    root / "target/uniffi-smoke/swift/fe_reader_uniffi.swift",
    root / "target/uniffi-smoke/python/fe_reader_uniffi.py",
    root / "target/uniffi-smoke/ruby/fe_reader_uniffi.rb",
]
missing = [str(path) for path in expected if not path.is_file()]
if missing:
    raise SystemExit(f"missing UniFFI smoke outputs: {missing}")

for path in expected:
    text = path.read_text(encoding="utf-8", errors="replace")
    assert "binding" in text.lower() or "FeBindingInfo" in text
    assert "draft" in text.lower() or "FePatchPlan" in text
PY

echo "UniFFI smoke check completed"
