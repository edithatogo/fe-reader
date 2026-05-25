#!/usr/bin/env bash
set -euo pipefail

PHASE="unknown"
AUTO_FIX="false"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --phase) PHASE="$2"; shift 2 ;;
    --auto-fix) AUTO_FIX="true"; shift ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

echo "== Fe Reader phase gate: ${PHASE} =="

if [[ "$AUTO_FIX" == "true" ]]; then
  cargo fmt --all || true
fi

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python3 scripts/strict_contract_check.py
python3 scripts/repository_ci_cd_check.py
python3 scripts/frontier_ci_check.py
python3 scripts/ci_policy_check.py
python3 scripts/validate_schemas.py
python3 scripts/v8_static_contract_check.py
bash scripts/v8_cli_smoke.sh
python3 scripts/wave0_acceptance_check.py
python3 scripts/strict_mutation_contract_check.py
python3 scripts/architecture_compliance_check.py --workspace-root .
python3 scripts/corpus_manifest_validate.py
bash scripts/security_policy_check.sh

if [[ -x scripts/sbom_audit.sh ]]; then scripts/sbom_audit.sh || echo "SBOM audit advisory skip/failure before tooling hardening"; fi
if [[ -x scripts/perf_smoke.sh ]]; then scripts/perf_smoke.sh || echo "perf smoke advisory skip"; fi
if [[ -x scripts/search_index_smoke.sh ]]; then scripts/search_index_smoke.sh || echo "search smoke advisory skip"; fi
if [[ -f scripts/mobile_smoke_bindings_check.py ]]; then python3 scripts/mobile_smoke_bindings_check.py || echo "mobile smoke bindings advisory skip"; fi
if [[ -f scripts/visual_regression_compare.py ]]; then python3 scripts/visual_regression_compare.py --smoke || echo "visual regression advisory skip"; fi
if [[ -x scripts/release_readiness_check.sh ]]; then scripts/release_readiness_check.sh || echo "release readiness advisory before Wave 4"; fi

if command -v cargo-deny >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny not installed; advisory skip"; fi
if command -v cargo-audit >/dev/null 2>&1; then cargo audit; else echo "cargo-audit not installed; advisory skip"; fi
if command -v cargo-vet >/dev/null 2>&1; then cargo vet; else echo "cargo-vet not installed; advisory skip"; fi

echo "== Fe Reader phase gate passed: ${PHASE} =="
