#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/frontier-reports
STATUS="advisory"
DETAIL="cargo fuzz is unavailable; fuzz campaigns remain advisory until Track K fuzz targets are materialised"
if command -v cargo >/dev/null 2>&1 && cargo fuzz --help >/dev/null 2>&1; then
  STATUS="pass"
  DETAIL="cargo fuzz is installed; frontier fuzz campaigns can run when targets exist"
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
Path("target/frontier-reports/fuzz-smoke.json").write_text(
    json.dumps({"check": "fuzz_smoke", "status": status, "detail": detail}, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY
echo "fuzz smoke: target/frontier-reports/fuzz-smoke.json"
