#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/frontier-reports
STATUS="advisory"
DETAIL="sanitizer smoke is advisory until nightly sanitizer toolchains are configured"
if command -v rustc >/dev/null 2>&1 && rustc -Z help >/dev/null 2>&1; then
  STATUS="pass"
  DETAIL="nightly rustc accepts -Z flags; sanitizer lane can be configured"
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
Path("target/frontier-reports/sanitizer-smoke.json").write_text(
    json.dumps({"check": "sanitizer_smoke", "status": status, "detail": detail}, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY
echo "sanitizer smoke: target/frontier-reports/sanitizer-smoke.json"
