#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
TARGETS="${FE_RELEASE_TARGETS:-windows,macos,linux,android,ios}"
python3 - "$CHANNEL" "$TARGETS" <<'PY'
import hashlib
import json
import os
import sys
from pathlib import Path

channel, targets_raw = sys.argv[1:3]
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
all_platforms = list(required_secrets)
requested_platforms = [
    token.strip().lower()
    for token in targets_raw.replace(";", ",").replace(" ", ",").split(",")
    if token.strip()
]
if not requested_platforms or requested_platforms == ["all"]:
    requested_platforms = all_platforms
unknown_platforms = sorted(set(requested_platforms) - set(all_platforms))
selected_required_secrets = {
    platform: required_secrets[platform]
    for platform in all_platforms
    if platform in requested_platforms
}
provided_secret_refs = {
    platform: [name for name in names if os.environ.get(name, "").strip()]
    for platform, names in required_secrets.items()
}
status = "config_error" if unknown_platforms else status
if unknown_platforms:
    detail = "Unknown release signing target platform requested"
if channel in strict_channels:
    missing = {
        platform: [name for name in names if not os.environ.get(name, "").strip()]
        for platform, names in selected_required_secrets.items()
    }
    missing = {platform: names for platform, names in missing.items() if names}
    if unknown_platforms:
        pass
    elif missing:
        status = "blocked"
        detail = "Strict release channel requires signing/notarization secret references for configured target platforms"
    else:
        status = "pass"
        detail = "Required signing/notarization secret references are present for configured target platforms"

report = {
    "check": "signing_readiness",
    "channel": channel,
    "configured_target_platforms": requested_platforms,
    "status": status,
    "detail": detail,
    "required_files": files,
    "required_secret_names": required_secrets,
    "required_secret_names_for_configured_targets": selected_required_secrets,
    "provided_secret_names": provided_secret_refs,
    "unknown_target_platforms": unknown_platforms,
}
Path("target/release-evidence/signing-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
if status in {"blocked", "config_error"}:
    raise SystemExit(detail)
PY
echo "signing readiness: target/release-evidence/signing-readiness.json"
