#!/usr/bin/env bash
set -euo pipefail

cargo test -q -p fe_reader_conversion
cargo test -q -p fe_reader_updates
python3 scripts/release_matrix_check.py
bash scripts/sbom_audit.sh
bash scripts/generate_provenance_attestation.sh
bash scripts/signing_readiness_check.sh
bash scripts/release_evidence_check.sh
python3 scripts/wave4_distribution_smoke.py

echo "wave4 distribution smoke passed"
