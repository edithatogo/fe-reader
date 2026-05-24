#!/usr/bin/env bash
set -euo pipefail
REQUIRED=(packaging/package-matrix.yaml packaging/release-channels.yaml packaging/codesigning.md schemas/update-manifest.schema.json)
for f in "${REQUIRED[@]}"; do
  [[ -f "$f" ]] || { echo "missing release file: $f" >&2; exit 1; }
done
if [[ -f target/security/sbom.cdx.json ]]; then
  python3 -m json.tool target/security/sbom.cdx.json >/dev/null
else
  echo "SBOM not present yet; advisory before release"
fi
echo "release readiness placeholder passed"
