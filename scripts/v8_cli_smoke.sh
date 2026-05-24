#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping CLI smoke in this environment"
  exit 0
fi
cargo run -p fe_reader_cli -- doctor
cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json
cargo run -p fe_reader_cli -- policy plugin
cargo run -p fe_reader_cli -- policy external-tool
