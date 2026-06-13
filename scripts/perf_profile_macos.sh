#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-native.startup}"
OUT_DIR="artifacts/perf/platform/macos"
mkdir -p "$OUT_DIR"

case "$SCENARIO" in
  native.startup)
    echo "Instrument Fe Reader with Instruments Time Profiler, Allocations, and Energy Log."
    ;;
  file.open)
    echo "Instrument file-open and sandbox/bookmark access with Instruments."
    ;;
  tile.render)
    echo "Instrument tile rendering and Metal System Trace in Instruments."
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac

cat >"$OUT_DIR/summary.md" <<EOF
# macOS Profiling

- Scenario: $SCENARIO
- Status: advisory
- Primary tools: Instruments Time Profiler, Allocations, Energy Log, Metal System Trace.
EOF
