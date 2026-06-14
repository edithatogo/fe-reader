#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs" / "release-readiness-v7-checklist.md"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"release readiness v7 check failed: {message}")


text = DOC.read_text(encoding="utf-8")

require("- [ ]" not in text, "checklist contains unchecked items")
for token in [
    "cargo metadata --format-version=1",
    "cargo test --workspace --all-targets",
    "cargo run -q -p fe_reader_cli -- doctor",
    "fixtures/minimal/minimal.pdf",
    "python3 scripts/validate_schemas.py",
    "python3 scripts/architecture_compliance_check.py",
    "scripts/strict_contract_check.py",
    "contracts/rust/error_taxonomy.rs",
    "contracts/rust/operation_transaction.rs",
    "schemas/operation-transaction.schema.json",
    "fixtures/corpus/manifest.json",
    "python3 scripts/wave0_first_30_prs_check.py",
    "python3 scripts/frontier_intelligence_governance_check.py",
    "python3 scripts/strict_mutation_contract_check.py",
    "bash scripts/platform_recent_smoke.sh",
]:
    require(token in text, f"missing evidence token: {token}")

for rel in [
    "contracts/rust/error_taxonomy.rs",
    "contracts/rust/operation_transaction.rs",
    "schemas/operation-transaction.schema.json",
    "fixtures/corpus/manifest.json",
    "docs/wave0-first-30-prs.md",
]:
    require((ROOT / rel).exists(), f"missing referenced evidence file: {rel}")

print("release readiness v7 checklist: ok")
