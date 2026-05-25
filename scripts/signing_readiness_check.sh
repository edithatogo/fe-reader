#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
python3 - "$CHANNEL" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

channel = sys.argv[1]
required = ["packaging/codesigning.md", "schemas/update-manifest.schema.json", "packaging/release-channels.yaml"]
files = []
for rel in required:
    path = Path(rel)
    data = path.read_bytes()
    files.append({"path": rel, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})

strict_channels = {"preview", "beta", "stable", "lts", "store_submission"}
status = "advisory"
detail = "Signing keys and notarization credentials are not required for dev bootstrap"
if channel in strict_channels:
    status = "waived"
    detail = "Release channel requires real signing/notarization before publication; Wave 0 records readiness contracts only"

report = {
    "check": "signing_readiness",
    "channel": channel,
    "status": status,
    "detail": detail,
    "required_files": files,
}
Path("target/release-evidence/signing-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "signing readiness: target/release-evidence/signing-readiness.json"
