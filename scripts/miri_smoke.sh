#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/frontier-reports
STATUS="advisory"
DETAIL="cargo miri is unavailable; install nightly miri to run this frontier check"
if command -v cargo >/dev/null 2>&1 && cargo miri --version >/dev/null 2>&1; then
  if cargo miri test -p fe_reader_core; then
    STATUS="pass"
    DETAIL="cargo miri test -p fe_reader_core passed"
  else
    DETAIL="cargo miri test -p fe_reader_core failed in advisory frontier lane"
  fi
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
Path("target/frontier-reports/miri-smoke.json").write_text(
    json.dumps({"check": "miri_smoke", "status": status, "detail": detail}, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY
echo "miri smoke: target/frontier-reports/miri-smoke.json"
