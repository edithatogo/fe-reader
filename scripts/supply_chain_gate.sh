#!/usr/bin/env bash
set -euo pipefail

# These are required, not best-effort: missing tools must never report success.
cargo metadata --locked --no-deps --format-version 1 >/dev/null
cargo deny check
cargo audit

# Dependency audit coverage remains advisory until a signed-off vet ledger exists.
mkdir -p target/release-evidence
if cargo vet check --store-path supply-chain --output-format=json > target/release-evidence/cargo-vet-report.json; then
  echo 'cargo-vet audit ledger accepted'
else
  echo '::warning::cargo-vet audit ledger incomplete; no dependency-audit clearance claimed' >&2
fi
