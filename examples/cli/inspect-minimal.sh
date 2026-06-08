#!/usr/bin/env bash
set -euo pipefail

cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json
