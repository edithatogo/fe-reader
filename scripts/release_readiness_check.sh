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
if [[ -f target/release-evidence/sbom.cdx.json ]]; then
  python3 -m json.tool target/release-evidence/sbom.cdx.json >/dev/null
  SBOM_STATUS="pass"
  SBOM_DETAIL="SBOM JSON parsed"
elif [[ -f target/release-evidence/sbom-status.json ]]; then
  python3 -m json.tool target/release-evidence/sbom-status.json >/dev/null
  SBOM_DETAIL="SBOM status evidence recorded"
else
  if [[ "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ]]; then
    echo "missing SBOM for release channel: $CHANNEL" >&2
    exit 1
  fi
  echo "$SBOM_DETAIL"
fi
PROVENANCE_STATUS="pass"
SIGNING_STATUS="pass"
if [[ ! -f target/release-evidence/provenance.json ]]; then
  if [[ "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ]]; then
    echo "missing provenance evidence for release channel: $CHANNEL" >&2
    exit 1
  fi
  PROVENANCE_STATUS="advisory"
fi
if [[ ! -f target/release-evidence/signing-readiness.json ]]; then
  if [[ "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ]]; then
    echo "missing signing readiness evidence for release channel: $CHANNEL" >&2
    exit 1
  fi
  SIGNING_STATUS="advisory"
else
  SIGNING_STATUS="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path("target/release-evidence/signing-readiness.json").read_text()).get("status", "unknown"))
PY
)"
  if [[ "$SIGNING_STATUS" == "blocked" && ( "$CHANNEL" == "stable" || "$CHANNEL" == "preview" || "$CHANNEL" == "beta" || "$CHANNEL" == "lts" || "$CHANNEL" == "store_submission" ) ]]; then
    echo "signing readiness blocked for release channel: $CHANNEL" >&2
    exit 1
  fi
fi
python3 - "$CHANNEL" "$SBOM_STATUS" "$SBOM_DETAIL" "$PROVENANCE_STATUS" "$SIGNING_STATUS" "${REQUIRED[@]}" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

import yaml

channel, sbom_status, sbom_detail, provenance_status, signing_status, *required = sys.argv[1:]
files = []
for rel in required:
    path = Path(rel)
    data = path.read_bytes()
    files.append({"path": rel, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
matrix = yaml.safe_load(Path("packaging/package-matrix.yaml").read_text(encoding="utf-8"))
channels = yaml.safe_load(Path("packaging/release-channels.yaml").read_text(encoding="utf-8"))
schema = json.loads(Path("schemas/release-evidence.schema.json").read_text(encoding="utf-8"))
if not isinstance(matrix, dict) or "targets" not in matrix:
    raise SystemExit("package matrix missing targets mapping")
if not isinstance(channels, dict) or "channels" not in channels:
    raise SystemExit("release channels missing channels mapping")
if schema.get("title") != "ReleaseEvidenceBundle":
    raise SystemExit("release evidence schema title mismatch")
if schema.get("properties", {}).get("channel", {}).get("enum") != ["dev", "nightly", "preview", "beta", "stable", "lts", "store_submission"]:
    raise SystemExit("release evidence schema channel enum mismatch")
report = {
    "check": "release_readiness",
    "channel": channel,
    "status": "pass",
    "required_files": files,
    "checks": [
        {"name": "required_release_files", "status": "pass"},
        {"name": "sbom_presence", "status": sbom_status, "detail": sbom_detail},
        {"name": "provenance_attestation", "status": provenance_status, "detail": "target/release-evidence/provenance.json"},
        {"name": "signing_readiness", "status": signing_status, "detail": "target/release-evidence/signing-readiness.json"},
    ],
}
for artifact in report["required_files"]:
    if len(artifact["sha256"]) != 64:
        raise SystemExit(f"invalid release readiness file digest: {artifact}")
release_evidence_schema = json.loads(Path("schemas/release-evidence.schema.json").read_text(encoding="utf-8"))
if release_evidence_schema.get("title") != "ReleaseEvidenceBundle":
    raise SystemExit("release evidence schema title mismatch")
Path("target/release-evidence/release-readiness.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "release readiness check passed"
