#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/frontier-reports
STATUS=advisory
DETAIL='pinned cargo-fuzz/nightly unavailable; no campaign executed'
RESULT=0
if command -v rustup >/dev/null 2>&1 && \
  rustup run nightly-2026-11-19 cargo fuzz --version >/dev/null 2>&1; then
  if bash fuzz/run.sh "${FUZZ_SECONDS:-30}"; then
    STATUS=pass
    DETAIL='pdf_bytes corpus and bounded mutation campaign executed successfully'
  else
    RESULT=$?
    STATUS=fail
    DETAIL="pdf_bytes campaign failed with exit $RESULT; inspect fuzz/target/evidence and fuzz/artifacts"
  fi
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
echo "fuzz smoke: $STATUS; target/frontier-reports/fuzz-smoke.json"
exit "$RESULT"
