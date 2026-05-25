#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

cargo build -p fe_reader_c_abi

case "$(uname -s)" in
  Darwin)
    lib="target/debug/libfe_reader_c_abi.dylib"
    nm -gU "$lib" | awk '{print $NF}' | sed 's/^_//' | sort > target/fe_reader_c_abi.symbols
    ;;
  Linux)
    lib="target/debug/libfe_reader_c_abi.so"
    nm -D --defined-only "$lib" | awk '{print $NF}' | sort > target/fe_reader_c_abi.symbols
    ;;
  *)
    echo "C ABI symbol check skipped on unsupported host $(uname -s)" >&2
    exit 0
    ;;
esac

expected_symbols=(
  fe_reader_c_abi_contract_json
  fe_reader_c_abi_plan_noop_contract
  fe_reader_c_abi_supports_apply
  fe_reader_c_abi_supports_plan_only
  fe_reader_c_abi_version_major
  fe_reader_c_abi_version_minor
  fe_reader_c_abi_version_patch
)

for symbol in "${expected_symbols[@]}"; do
  if ! grep -qx "$symbol" target/fe_reader_c_abi.symbols; then
    echo "missing C ABI symbol: $symbol" >&2
    echo "exported symbols:" >&2
    cat target/fe_reader_c_abi.symbols >&2
    exit 1
  fi
done

python3 - <<'PY'
import json
from pathlib import Path

snapshot = Path("contracts/snapshots/c-abi/fe_reader_c_abi.facade.json")
data = json.loads(snapshot.read_text(encoding="utf-8"))
exports = {entry["name"] for entry in data["exports"]}
symbols = set(Path("target/fe_reader_c_abi.symbols").read_text(encoding="utf-8").splitlines())
missing = sorted(exports - symbols)
if missing:
    raise SystemExit(f"C ABI snapshot exports missing from library: {missing}")
if data["mutation_policy"] != "read_only_or_plan_only":
    raise SystemExit("C ABI mutation policy must stay read_only_or_plan_only in Wave 0")
if data["abi_version"]["major"] != 0:
    raise SystemExit("C ABI major version must remain 0 during preview bootstrap")
PY

echo "C ABI snapshot check passed"
