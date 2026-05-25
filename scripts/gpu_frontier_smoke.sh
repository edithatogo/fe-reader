#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/frontier-reports
STATUS="advisory"
DETAIL="GPU frontier features are advisory until visual and performance baselines exist"
if command -v cargo >/dev/null 2>&1; then
  if cargo check -p fe_reader_render_gpu --features vello_overlay,skia_experiment; then
    STATUS="pass"
    DETAIL="GPU frontier feature compile check passed"
  else
    DETAIL="GPU frontier feature compile check failed in advisory frontier lane"
  fi
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
Path("target/frontier-reports/gpu-frontier-smoke.json").write_text(
    json.dumps({"check": "gpu_frontier_smoke", "status": status, "detail": detail}, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY
echo "gpu frontier smoke: target/frontier-reports/gpu-frontier-smoke.json"
