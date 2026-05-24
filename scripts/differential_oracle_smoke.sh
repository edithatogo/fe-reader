#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/oracle-reports
cat > target/oracle-reports/smoke.json <<'JSON'
{"run_id":"smoke","fixture_id":"none","operation":"SyntaxValidity","tools":[],"comparison":{"status":"OracleUnavailable","notes":["Install qpdf/veraPDF/PDFium/Poppler/MuPDF/Ghostscript to enable full oracle runs."]}}
JSON
echo "differential oracle smoke: target/oracle-reports/smoke.json"
