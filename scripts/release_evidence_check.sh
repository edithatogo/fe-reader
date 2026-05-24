#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
cat > target/release-evidence/evidence.smoke.json <<'JSON'
{"release_id":"dev-smoke","channel":"dev","source_commit":"unknown","toolchain":"unknown","artifacts":[]}
JSON
echo "release evidence smoke: target/release-evidence/evidence.smoke.json"
