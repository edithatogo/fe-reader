#!/usr/bin/env bash
set -euo pipefail

cargo test -q -p fe_reader_config frontier
cargo test -q -p fe_reader_render_gpu
bash scripts/gpu_frontier_smoke.sh
bash scripts/toolchain_experiment_smoke.sh
python3 scripts/wave6_frontier_optional_smoke.py

echo "wave6 frontier optional smoke passed"
