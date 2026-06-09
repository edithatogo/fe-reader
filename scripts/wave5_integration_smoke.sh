#!/usr/bin/env bash
set -euo pipefail

cargo test -q -p fe_reader_mcp
cargo test -q -p fe_reader_plugin_host
cargo test -q -p fe_reader_platform native_automation
cargo test -q -p fe_reader_config enterprise_policy
python3 scripts/wave5_integration_smoke.py

echo "wave5 integration smoke passed"
