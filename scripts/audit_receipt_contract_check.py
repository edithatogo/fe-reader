#!/usr/bin/env python3
"""Validate the Wave 0 audit receipt contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/audit-receipt.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_core.audit_receipt.preview.json"


def fail(message: str) -> None:
    raise SystemExit(f"audit receipt contract check failed: {message}")


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"receipt_id"',
        '"intent_id"',
        '"plan_id"',
        '"transaction_id"',
        '"document_id"',
        '"document_fingerprint_before"',
        '"document_fingerprint_after"',
        '"write_mode"',
        '"risk_level"',
        '"verification_status"',
        '"created_at_utc"',
        '"not_required"',
        '"passed"',
        '"failed"',
        "^[a-f0-9]{64}$",
    ):
        if token not in schema_text:
            fail(f"audit receipt schema missing token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"audit receipt snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_core",
        "stability": "preview",
        "phase": "A6",
        "contract": "audit_receipt",
        "mutation_policy": "receipts_bind_intent_plan_transaction_verification_and_document_hashes",
        "schema": "schemas/audit-receipt.schema.json",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"audit receipt snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for token in (
        "AuditReceipt",
        "OperationReceipt",
        "ReceiptId",
        "VerificationStatus",
        "TransactionJournal",
        "PatchPlan",
        "OperationIntent",
        "DocumentFingerprint",
    ):
        if token not in public_types:
            fail(f"audit receipt snapshot missing public type {token}")


def require_schema_sample() -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return

    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    sample = {
        "receipt_id": "receipt-1",
        "intent_id": "intent-1",
        "plan_id": "plan-1",
        "transaction_id": "transaction-1",
        "document_id": "document-1",
        "write_mode": "sanitizing_rewrite",
        "risk_level": "high_risk",
        "document_fingerprint_before": {
            "sha256_hex": "0" * 64,
            "byte_len": 12,
        },
        "document_fingerprint_after": {
            "sha256_hex": "1" * 64,
            "byte_len": 8,
        },
        "verification_status": "passed",
        "created_at_utc": "2026-05-26T00:00:00Z",
        "summary": "verification passed",
    }
    jsonschema.validate(sample, schema)


def require_core_tests() -> None:
    subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "-p",
            "fe_reader_core",
            "receipt",
        ],
        cwd=ROOT,
        check=True,
    )


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_schema_sample()
    require_core_tests()
    print("audit receipt contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
