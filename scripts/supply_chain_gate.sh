#!/usr/bin/env bash
set -euo pipefail
status=0
if command -v cargo-deny >/dev/null 2>&1; then cargo deny check || status=$?; else echo "cargo-deny missing"; fi
if command -v cargo-audit >/dev/null 2>&1; then cargo audit || status=$?; else echo "cargo-audit missing"; fi
if command -v cargo-vet >/dev/null 2>&1; then cargo vet || status=$?; else echo "cargo-vet missing"; fi
exit $status
