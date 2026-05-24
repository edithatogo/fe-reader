#!/usr/bin/env bash
set -euo pipefail
if [[ -x target/debug/fe-reader ]]; then
  target/debug/fe-reader --help >/dev/null
else
  echo "fe-reader binary not built yet; smoke skip"
fi
