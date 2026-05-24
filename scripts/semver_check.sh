#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo-semver-checks >/dev/null 2>&1; then
  echo "cargo-semver-checks not installed; advisory during bootstrap" >&2
  exit 0
fi
cargo semver-checks check-release --workspace
