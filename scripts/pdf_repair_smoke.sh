#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/recovery-reports
cat > target/recovery-reports/smoke.json <<'JSON'
{"plan_id":"smoke","original_sha256":"unknown","mode":"RepairPlanOnly","actions":[],"write_policy":"PlanOnly","warnings":["Repair implementation not materialised yet."]}
JSON
echo "pdf repair smoke: target/recovery-reports/smoke.json"
