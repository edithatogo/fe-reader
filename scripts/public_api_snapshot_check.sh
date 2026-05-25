#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo-public-api >/dev/null 2>&1; then
  echo "cargo-public-api not installed; advisory during bootstrap" >&2
  exit 0
fi
mkdir -p target/public-api
for crate in fe_reader_core fe_reader_pdf_model fe_reader_security fe_reader_cli fe_reader_uniffi fe_reader_c_abi; do
  cargo public-api -p "$crate" > "target/public-api/${crate}.txt" || true
done
echo "public API snapshots written to target/public-api"
