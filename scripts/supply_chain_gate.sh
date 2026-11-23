#!/usr/bin/env bash
set -euo pipefail

# These are required, not best-effort: missing tools must never report success.
cargo metadata --locked --no-deps --format-version 1 >/dev/null
cargo deny check
cargo audit

# Dependency audit coverage remains advisory until a signed-off vet ledger exists.
mkdir -p target/release-evidence
if cargo vet --version >/dev/null 2>&1; then
  if cargo vet check --store-path supply-chain --output-format=json > target/release-evidence/cargo-vet-report.json; then
    echo 'cargo-vet audit ledger accepted'
  else
    echo '::warning::cargo-vet ledger incomplete; no dependency-audit clearance claimed' >&2
  fi
else
  echo '::warning::cargo-vet not installed; dependency-audit coverage unverified' >&2
fi
