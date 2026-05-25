#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
SOURCE_COMMIT="$(git rev-parse --verify HEAD)"
TOOLCHAIN="$(rustc --version 2>/dev/null || printf 'rustc-unavailable')"
RELEASE_ID="${FE_RELEASE_ID:-dev-smoke}"
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
python3 - "$SOURCE_COMMIT" "$TOOLCHAIN" "$RELEASE_ID" "$CHANNEL" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

source_commit, toolchain, release_id, channel = sys.argv[1:5]
if not source_commit or source_commit == "unknown":
    raise SystemExit("source_commit must be resolved")
if not toolchain or toolchain == "unknown":
    raise SystemExit("toolchain must be resolved")
if channel not in {"dev", "nightly", "preview", "beta", "stable", "lts", "store_submission"}:
    raise SystemExit(f"unsupported release channel: {channel}")

inputs = [
    Path("packaging/package-matrix.yaml"),
    Path("packaging/release-channels.yaml"),
    Path("packaging/codesigning.md"),
    Path("schemas/release-evidence.schema.json"),
]
artifacts = []
for path in inputs:
    data = path.read_bytes()
    artifacts.append(
        {
            "path": str(path),
            "sha256": hashlib.sha256(data).hexdigest(),
            "kind": "release_contract_input",
            "bytes": len(data),
        }
    )

evidence = {
    "release_id": release_id,
    "channel": channel,
    "source_commit": source_commit,
    "toolchain": toolchain,
    "artifacts": artifacts,
    "checks": [
        {
            "name": "release_evidence_contract",
            "status": "pass",
            "detail": "source commit, toolchain, channel, and release contract inputs recorded",
        }
    ],
}
out = Path("target/release-evidence/evidence.smoke.json")
out.write_text(json.dumps(evidence, sort_keys=True) + "\n", encoding="utf-8")
PY
echo "release evidence smoke: target/release-evidence/evidence.smoke.json"
