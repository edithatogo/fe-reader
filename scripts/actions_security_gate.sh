#!/usr/bin/env bash
set -euo pipefail
status=0
if command -v zizmor >/dev/null 2>&1; then zizmor .github/workflows || status=$?; else echo "zizmor missing; install for hard gate"; fi
if command -v actionlint >/dev/null 2>&1; then actionlint || status=$?; else echo "actionlint missing; install for hard gate"; fi
exit $status
