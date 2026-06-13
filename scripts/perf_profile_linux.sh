#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-cli.inspect.small_text_pdf}"
OUT_DIR="artifacts/perf/platform/linux"
mkdir -p "$OUT_DIR" artifacts/perf/flamegraphs

case "$SCENARIO" in
  cli.inspect.small_text_pdf)
    if command -v cargo-flamegraph >/dev/null 2>&1 || command -v cargo >/dev/null 2>&1; then
      cargo flamegraph --bin fe-reader -- inspect fixtures/corpus/sample.pdf --json || true
      if [[ -f flamegraph.svg ]]; then
        mv flamegraph.svg artifacts/perf/flamegraphs/${SCENARIO}.svg
      fi
    fi
    ;;
  heaptrack.render_tile)
    if command -v heaptrack >/dev/null 2>&1; then
      heaptrack cargo run -p fe_reader_cli --bin fe-reader -- render-tile fixtures/corpus/sample.pdf --page 1 --tile 0,0,512,512 --scale 2.0 --out /tmp/fe-render-tile.png || true
    fi
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac

cat >"$OUT_DIR/summary.md" <<EOF
# Linux Profiling

- Scenario: $SCENARIO
- Status: advisory
- Notes: use perf, cargo flamegraph, heaptrack, and Valgrind on Linux CI or a local profiling workstation.
EOF
