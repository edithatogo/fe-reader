#!/usr/bin/env bash
set -euo pipefail

bash scripts/sbom_audit.sh
bash scripts/generate_provenance_attestation.sh
bash scripts/signing_readiness_check.sh
bash scripts/release_readiness_check.sh
bash scripts/release_evidence_check.sh
python3 scripts/release_provenance_check.py
bash scripts/api_compat_check.sh
bash scripts/differential_oracle_smoke.sh
python3 scripts/accessibility_audit_smoke.py
bash scripts/cache_workspace_smoke.sh
bash scripts/optimization_oracle_smoke.sh
bash scripts/gpu_frontier_smoke.sh
bash scripts/toolchain_experiment_smoke.sh
python3 scripts/wave7_release_hardening_smoke.py

echo "wave7 release hardening smoke passed"
