#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

required_files=(
  "Cargo.toml"
  "rust-toolchain.toml"
  "crates/fe_reader_core/src/lib.rs"
  "crates/fe_reader_cli/src/main.rs"
  "contracts/rust/operation_transaction.rs"
  "contracts/rust/error_taxonomy.rs"
  "contracts/rust/data_migration.rs"
  "schemas/operation-transaction.schema.json"
  "schemas/error-taxonomy.schema.json"
  "schemas/file-format-version.schema.json"
  "docs/wave0-first-30-prs.md"
  "docs/implementation-stop-rule.md"
  "docs/wave0-contract-acceptance-tests.md"
)

for f in "${required_files[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "missing required Wave 0 file: $f" >&2
    exit 1
  fi
done

if command -v python3 >/dev/null 2>&1; then
  python3 scripts/validate_schemas.py
fi

if command -v cargo >/dev/null 2>&1; then
  cargo metadata --format-version=1 >/dev/null
  cargo fmt --all -- --check
  cargo test --workspace --all-targets
else
  echo "cargo not found; skipped Rust compile checks"
fi

echo "Wave 0 bootstrap scaffold check completed"

python3 scripts/v8_static_contract_check.py
bash scripts/v8_cli_smoke.sh
python3 scripts/wave0_acceptance_check.py
