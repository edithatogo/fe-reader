#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/security
if command -v cargo-cyclonedx >/dev/null 2>&1; then
  cargo cyclonedx --format json --output-file target/security/sbom.cdx.json || exit 1
else
  echo "cargo-cyclonedx not installed; advisory skip"
fi
if command -v cargo-deny >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny not installed; advisory skip"; fi
echo "sbom audit check completed"
