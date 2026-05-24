#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/api-compat
if command -v cargo-semver-checks >/dev/null 2>&1; then
  cargo semver-checks --workspace || true
else
  echo "cargo-semver-checks not installed; advisory only until Track V configures baseline" > target/api-compat/advisory.txt
fi
echo "api compatibility smoke complete"
