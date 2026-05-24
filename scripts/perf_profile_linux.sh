#!/usr/bin/env bash
set -euo pipefail
SCENARIO="${1:-cli.inspect.small_text_pdf}"
mkdir -p artifacts/perf/flamegraphs
case "$SCENARIO" in
  cli.inspect.small_text_pdf)
    cargo flamegraph --bin fe-reader -- inspect fixtures/corpus/sample.pdf --json
    mv flamegraph.svg artifacts/perf/flamegraphs/${SCENARIO}.svg
    ;;
  *)
    echo "Unknown scenario: $SCENARIO" >&2
    exit 2
    ;;
esac
