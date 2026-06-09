#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping platform recent-document smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_platform
