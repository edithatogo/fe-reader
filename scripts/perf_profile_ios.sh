#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-pencil.latency}"
OUT_DIR="artifacts/perf/platform/ios"
mkdir -p "$OUT_DIR"

case "$SCENARIO" in
  pencil.latency)
    echo "Profile Pencil latency, file coordination, and memory pressure with Xcode Instruments."
    ;;
  metal.render)
    echo "Capture Metal System Trace and Time Profiler output for native rendering."
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac

cat >"$OUT_DIR/summary.md" <<EOF
# iOS Profiling

- Scenario: $SCENARIO
- Status: advisory
- Primary tools: Xcode Instruments, Metal System Trace, Time Profiler, Allocations, Energy Log.
EOF
