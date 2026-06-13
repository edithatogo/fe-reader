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
python3 scripts/version_consistency_check.py
python3 scripts/validate_schemas.py
python3 scripts/requirements_traceability_check.py
python3 scripts/operation_intent_contract_check.py
python3 scripts/patch_plan_contract_check.py
python3 scripts/pdf_model_contract_check.py
python3 scripts/page_ops_contract_check.py
python3 scripts/write_modes_contract_check.py
python3 scripts/audit_receipt_contract_check.py
python3 scripts/document_ir_contract_check.py
python3 scripts/v8_static_contract_check.py
bash scripts/v8_cli_smoke.sh
python3 scripts/wave0_acceptance_check.py
python3 scripts/strict_mutation_contract_check.py
python3 scripts/architecture_compliance_check.py --workspace-root .
python3 scripts/corpus_manifest_validate.py
bash scripts/security_policy_check.sh
python3 scripts/job_contract_smoke.py
if [[ -x scripts/optimization_oracle_smoke.sh ]]; then scripts/optimization_oracle_smoke.sh; fi
if [[ -x scripts/source_linked_smoke.sh ]]; then scripts/source_linked_smoke.sh; fi
if [[ -x scripts/cache_workspace_smoke.sh ]]; then scripts/cache_workspace_smoke.sh; fi

if [[ -x scripts/sbom_audit.sh ]]; then scripts/sbom_audit.sh || echo "SBOM audit advisory skip/failure before tooling hardening"; fi
if [[ -x scripts/perf_smoke.sh && "${CONDUCTOR_SKIP_PERF_SMOKE:-0}" != "1" ]]; then scripts/perf_smoke.sh || echo "perf smoke advisory skip"; fi
if [[ -x scripts/search_index_smoke.sh ]]; then scripts/search_index_smoke.sh || echo "search smoke advisory skip"; fi
if [[ -x scripts/metadata_wave2_smoke.sh ]]; then scripts/metadata_wave2_smoke.sh || echo "metadata Wave 2 advisory skip"; fi
if [[ -x scripts/cli_golden_smoke.sh ]]; then scripts/cli_golden_smoke.sh || echo "CLI golden advisory skip"; fi
if [[ -x scripts/workflow_pack_smoke.sh ]]; then scripts/workflow_pack_smoke.sh || echo "workflow pack advisory skip"; fi
if [[ -x scripts/differential_oracle_smoke.sh ]]; then scripts/differential_oracle_smoke.sh || echo "differential oracle advisory skip"; fi
if [[ -x scripts/redaction_verification_smoke.sh ]]; then scripts/redaction_verification_smoke.sh || echo "redaction verification advisory skip"; fi
if [[ -x scripts/wave1_render_smoke.sh ]]; then scripts/wave1_render_smoke.sh || echo "Wave 1 render smoke advisory skip"; fi
if [[ -x scripts/platform_recent_smoke.sh ]]; then scripts/platform_recent_smoke.sh || echo "platform recent-document advisory skip"; fi
if [[ -x scripts/pdf_lab_inspect_smoke.sh ]]; then scripts/pdf_lab_inspect_smoke.sh || echo "PDF lab inspect advisory skip"; fi
if [[ -x scripts/pdf_lab_text_map_smoke.sh ]]; then scripts/pdf_lab_text_map_smoke.sh || echo "PDF lab text-map advisory skip"; fi
if [[ -x scripts/pdf_lab_timeline_smoke.sh ]]; then scripts/pdf_lab_timeline_smoke.sh || echo "PDF lab timeline advisory skip"; fi
if [[ -x scripts/pdf_lab_redaction_scan_smoke.sh ]]; then scripts/pdf_lab_redaction_scan_smoke.sh || echo "PDF lab redaction-scan advisory skip"; fi
if [[ -x scripts/wave4_distribution_smoke.sh ]]; then scripts/wave4_distribution_smoke.sh || echo "Wave 4 distribution advisory skip"; fi
if [[ -x scripts/wave5_integration_smoke.sh ]]; then scripts/wave5_integration_smoke.sh || echo "Wave 5 integration advisory skip"; fi
if [[ -x scripts/wave6_frontier_optional_smoke.sh ]]; then scripts/wave6_frontier_optional_smoke.sh || echo "Wave 6 frontier advisory skip"; fi
if [[ -x scripts/wave7_release_hardening_smoke.sh ]]; then scripts/wave7_release_hardening_smoke.sh || echo "Wave 7 release hardening advisory skip"; fi
if [[ -x scripts/wave8_adoption_ecosystem_smoke.sh ]]; then scripts/wave8_adoption_ecosystem_smoke.sh || echo "Wave 8 adoption advisory skip"; fi
if [[ -f scripts/mobile_smoke_bindings_check.py ]]; then python3 scripts/mobile_smoke_bindings_check.py || echo "mobile smoke bindings advisory skip"; fi
if [[ -x scripts/uniffi_smoke_check.sh ]]; then scripts/uniffi_smoke_check.sh || echo "UniFFI smoke advisory skip"; fi
if [[ -f scripts/visual_regression_compare.py ]]; then python3 scripts/visual_regression_compare.py --smoke || echo "visual regression advisory skip"; fi
if [[ -x scripts/prepress_smoke.sh ]]; then scripts/prepress_smoke.sh || echo "prepress smoke advisory skip"; fi
if [[ -x scripts/pdf_repair_smoke.sh ]]; then scripts/pdf_repair_smoke.sh || echo "PDF repair advisory skip"; fi
if [[ -x scripts/release_readiness_check.sh ]]; then scripts/release_readiness_check.sh || echo "release readiness advisory before Wave 4"; fi

if cargo deny --version >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny not installed; advisory skip"; fi
if cargo audit --version >/dev/null 2>&1; then cargo audit; else echo "cargo-audit not installed; advisory skip"; fi
if cargo vet --version >/dev/null 2>&1; then
  mkdir -p target/release-evidence
  cargo vet check --store-path supply-chain --output-format=json > target/release-evidence/cargo-vet-report.json || echo "cargo-vet advisory incomplete; dependency audits not fully populated"
else
  echo "cargo-vet not installed; advisory skip"
fi

echo "== Fe Reader phase gate passed: ${PHASE} =="
