#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
STATUS="advisory"
DETAIL="cargo-semver-checks not installed; advisory during bootstrap"
if ! command -v cargo-semver-checks >/dev/null 2>&1; then
  echo "$DETAIL" >&2
else
  cargo semver-checks check-release --workspace
  STATUS="pass"
  DETAIL="cargo semver-checks release comparison passed"
fi
python3 - "$STATUS" "$DETAIL" <<'PY'
import json
import sys
from pathlib import Path

status, detail = sys.argv[1:3]
report = {
    "check": "semver_check",
    "status": status,
    "detail": detail,
}
Path("target/release-evidence/semver-status.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
PY
