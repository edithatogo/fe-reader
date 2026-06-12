#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/security
mkdir -p target/release-evidence
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
SBOM_PATH="target/release-evidence/sbom.cdx.json"
STATUS="advisory"
DETAIL="cargo-cyclonedx not installed; SBOM generation skipped for dev bootstrap"
if cargo cyclonedx --version >/dev/null 2>&1; then
  find crates -type f \( -name '*.cdx.json' -o -name 'sbom.cdx.json' \) -delete
  cargo cyclonedx --format json --all-features || exit 1
  if [[ -f crates/fe_reader_cli/fe_reader_cli.cdx.json ]]; then
    cp crates/fe_reader_cli/fe_reader_cli.cdx.json "$SBOM_PATH"
  else
    FIRST_SBOM="$(find crates -type f -name '*.cdx.json' | sort | head -n 1)"
    if [[ -z "$FIRST_SBOM" ]]; then
      echo "cargo-cyclonedx completed but no CycloneDX JSON file was found" >&2
      exit 1
    fi
    cp "$FIRST_SBOM" "$SBOM_PATH"
  fi
  find crates -type f \( -name '*.cdx.json' -o -name 'sbom.cdx.json' \) -delete
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
if cargo deny --version >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny not installed; advisory skip"; fi
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
    sbom = json.loads(Path(sbom_path).read_text(encoding="utf-8"))
    if sbom.get("bomFormat") != "CycloneDX":
        raise SystemExit("SBOM missing CycloneDX bomFormat")
    if not sbom.get("specVersion"):
        raise SystemExit("SBOM missing specVersion")
    metadata = sbom.get("metadata", {})
    component = metadata.get("component", {})
    for key in ["type", "name", "bom-ref"]:
        if not component.get(key):
            raise SystemExit(f"SBOM metadata component missing {key}")
    components = sbom.get("components", [])
    if not isinstance(components, list) or not components:
        raise SystemExit("SBOM missing components")
    report["sha256"] = hashlib.sha256(Path(sbom_path).read_bytes()).hexdigest()
    report["bytes"] = len(Path(sbom_path).read_bytes())
    report["bomFormat"] = sbom.get("bomFormat")
    report["specVersion"] = sbom.get("specVersion")
    report["componentCount"] = len(components)
Path("target/release-evidence/sbom-status.json").write_text(
    json.dumps(report, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "sbom audit check completed"
