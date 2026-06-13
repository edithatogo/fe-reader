#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-wasm.load}"
OUT_DIR="artifacts/perf/platform/web"
mkdir -p "$OUT_DIR"

case "$SCENARIO" in
  wasm.load)
    echo "Capture Chromium performance traces for WASM load and startup."
    ;;
  tile.compositing)
    echo "Capture browser performance panel traces for tile compositing."
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac

cat >"$OUT_DIR/summary.md" <<EOF
# Web Profiling

- Scenario: $SCENARIO
- Status: advisory
- Primary tools: Chrome Performance panel, Lighthouse, WebGPU capture tools.
EOF
