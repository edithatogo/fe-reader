#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/security
mkdir -p target/release-evidence
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
SBOM_PATH="target/release-evidence/sbom.cdx.json"
STATUS="advisory"
DETAIL="cargo-cyclonedx not installed; SBOM generation skipped for dev bootstrap"
if command -v cargo-cyclonedx >/dev/null 2>&1; then
  cargo cyclonedx --format json --output-file "$SBOM_PATH" || exit 1
  python3 -m json.tool "$SBOM_PATH" >/dev/null
  cp "$SBOM_PATH" target/security/sbom.cdx.json
  STATUS="pass"
  DETAIL="CycloneDX SBOM generated"
else
  if [[ "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ]]; then
    echo "cargo-cyclonedx missing for release channel: $CHANNEL" >&2
    exit 1
  fi
  echo "$DETAIL"
fi
if command -v cargo-deny >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny not installed; advisory skip"; fi
if command -v cargo >/dev/null 2>&1; then
  cargo metadata --locked --format-version=1 > target/release-evidence/cargo-metadata.json
fi
python3 - "$STATUS" "$DETAIL" "$SBOM_PATH" "$CHANNEL" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

status, detail, sbom_path, channel = sys.argv[1:5]
report = {
    "check": "sbom_audit",
    "channel": channel,
    "status": status,
    "detail": detail,
    "sbom_path": sbom_path if Path(sbom_path).exists() else None,
}
if Path(sbom_path).exists():
    data = Path(sbom_path).read_bytes()
    report["sha256"] = hashlib.sha256(data).hexdigest()
    report["bytes"] = len(data)
Path("target/release-evidence/sbom-status.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "sbom audit check completed"
