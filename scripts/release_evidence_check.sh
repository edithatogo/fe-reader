#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
SOURCE_COMMIT="$(git rev-parse --verify HEAD)"
TOOLCHAIN="$(rustc --version 2>/dev/null || printf 'rustc-unavailable')"
python3 - "$SOURCE_COMMIT" "$TOOLCHAIN" <<'PY'
import json
import sys
from pathlib import Path

source_commit, toolchain = sys.argv[1], sys.argv[2]
if not source_commit or source_commit == "unknown":
    raise SystemExit("source_commit must be resolved")
if not toolchain or toolchain == "unknown":
    raise SystemExit("toolchain must be resolved")

evidence = {
    "release_id": "dev-smoke",
    "channel": "dev",
    "source_commit": source_commit,
    "toolchain": toolchain,
    "artifacts": [],
}
out = Path("target/release-evidence/evidence.smoke.json")
out.write_text(json.dumps(evidence, sort_keys=True) + "\n", encoding="utf-8")
PY
echo "release evidence smoke: target/release-evidence/evidence.smoke.json"
