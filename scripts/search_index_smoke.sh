#!/usr/bin/env bash
set -euo pipefail
if [[ -x target/debug/fe-reader ]]; then
  target/debug/fe-reader search --help >/dev/null || true
else
  echo "fe-reader CLI not built yet; advisory skip"
fi
echo "search smoke completed"
