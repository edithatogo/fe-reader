#!/usr/bin/env bash
set -euo pipefail
REQUIRED=(packaging/package-matrix.yaml packaging/release-channels.yaml packaging/codesigning.md schemas/update-manifest.schema.json)
mkdir -p target/release-evidence
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
for f in "${REQUIRED[@]}"; do
  [[ -f "$f" ]] || { echo "missing release file: $f" >&2; exit 1; }
done
SBOM_STATUS="advisory"
SBOM_DETAIL="SBOM not present yet; deferred to Track AO for dev bootstrap"
if [[ -f target/security/sbom.cdx.json ]]; then
  python3 -m json.tool target/security/sbom.cdx.json >/dev/null
  SBOM_STATUS="pass"
  SBOM_DETAIL="SBOM JSON parsed"
else
  if [[ "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ]]; then
    echo "missing SBOM for release channel: $CHANNEL" >&2
    exit 1
  fi
  echo "$SBOM_DETAIL"
fi
python3 - "$CHANNEL" "$SBOM_STATUS" "$SBOM_DETAIL" "${REQUIRED[@]}" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

channel, sbom_status, sbom_detail, *required = sys.argv[1:]
files = []
for rel in required:
    path = Path(rel)
    data = path.read_bytes()
    files.append({"path": rel, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
report = {
    "check": "release_readiness",
    "channel": channel,
    "status": "pass",
    "required_files": files,
    "checks": [
        {"name": "required_release_files", "status": "pass"},
        {"name": "sbom_presence", "status": sbom_status, "detail": sbom_detail},
        {"name": "provenance_attestation", "status": "advisory", "detail": "deferred to Track AO"},
    ],
}
Path("target/release-evidence/release-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "release readiness check passed"
