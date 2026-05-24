#!/usr/bin/env bash
set -euo pipefail

echo "== Fe Reader perf smoke =="
if command -v cargo >/dev/null 2>&1; then
  cargo bench --workspace --no-run || true
fi
if command -v hyperfine >/dev/null 2>&1 && command -v fe-reader >/dev/null 2>&1; then
  hyperfine --warmup 1 'fe-reader --version' || true
else
  echo "hyperfine or fe-reader not installed; perf smoke advisory skip"
fi
