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
required_secrets = {
    "windows": ["FE_WINDOWS_SIGNING_CERT", "FE_WINDOWS_SIGNING_PASSWORD"],
    "macos": ["FE_MACOS_DEVELOPER_ID_CERT", "FE_MACOS_NOTARY_PROFILE"],
    "linux": ["FE_LINUX_SIGNING_KEY"],
    "android": ["FE_ANDROID_UPLOAD_KEYSTORE", "FE_ANDROID_UPLOAD_KEYSTORE_PASSWORD"],
    "ios": ["FE_IOS_DISTRIBUTION_CERT", "FE_IOS_APPSTORE_CONNECT_KEY"],
}
provided_secret_refs = {
    platform: [name for name in names if name in __import__("os").environ]
    for platform, names in required_secrets.items()
}
if channel in strict_channels:
    missing = {
        platform: [name for name in names if name not in __import__("os").environ]
        for platform, names in required_secrets.items()
    }
    missing = {platform: names for platform, names in missing.items() if names}
    if missing:
        status = "blocked"
        detail = "Strict release channel requires signing/notarization secret references before publication"
    else:
        status = "pass"
        detail = "Required signing/notarization secret references are present"

report = {
    "check": "signing_readiness",
    "channel": channel,
    "status": status,
    "detail": detail,
    "required_files": files,
    "required_secret_names": required_secrets,
    "provided_secret_names": provided_secret_refs,
}
Path("target/release-evidence/signing-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
if status == "blocked":
    raise SystemExit(detail)
PY
echo "signing readiness: target/release-evidence/signing-readiness.json"
