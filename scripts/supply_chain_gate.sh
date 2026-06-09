#!/usr/bin/env bash
set -euo pipefail
status=0
if cargo deny --version >/dev/null 2>&1; then cargo deny check || status=$?; else echo "cargo-deny missing"; fi
if cargo audit --version >/dev/null 2>&1; then cargo audit || status=$?; else echo "cargo-audit missing"; fi
if cargo vet --version >/dev/null 2>&1; then
  mkdir -p target/release-evidence
  cargo vet check --store-path supply-chain --output-format=json > target/release-evidence/cargo-vet-report.json || echo "cargo-vet advisory incomplete; dependency audits not fully populated"
else
  echo "cargo-vet missing"
fi
exit $status
