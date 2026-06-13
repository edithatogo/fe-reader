#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-scroll.latency}"
OUT_DIR="artifacts/perf/platform/android"
mkdir -p "$OUT_DIR"

case "$SCENARIO" in
  scroll.latency)
    echo "Profile scroll and annotation latency with Perfetto and Android Studio Profiler."
    ;;
  simpleperf.render)
    echo "Capture a simpleperf trace for render and bridge hot paths."
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac

cat >"$OUT_DIR/summary.md" <<EOF
# Android Profiling

- Scenario: $SCENARIO
- Status: advisory
- Primary tools: Android Studio Profiler, Perfetto, simpleperf, systrace.
EOF
