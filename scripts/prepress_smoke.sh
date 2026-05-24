#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/prepress-reports
cat > target/prepress-reports/smoke.json <<'JSON'
{"document_id":"none","output_intents":[],"colour_findings":[],"font_findings":[],"page_box_findings":[]}
JSON
echo "prepress smoke: target/prepress-reports/smoke.json"
