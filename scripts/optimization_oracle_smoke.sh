#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
doc = (root / "docs/pdf-optimization-linearization-compression.md").read_text(encoding="utf-8")
contract = (root / "contracts/rust/pdf_optimization.rs").read_text(encoding="utf-8")
schema = json.loads((root / "schemas/pdf-optimization-plan.schema.json").read_text(encoding="utf-8"))

for token in [
    "Reduce file size without changing visible output.",
    "Emit an optimisation receipt.",
    "Never invalidate a signature without a warning and receipt.",
]:
    if token not in doc:
        raise SystemExit(f"optimization oracle smoke failure: missing doc token {token}")

for token in [
    "PdfOptimisationPlan",
    "PdfOptimisationReceipt",
    "PdfOptimiser",
    "WebDeliveryLinearized",
]:
    if token not in contract:
        raise SystemExit(f"optimization oracle smoke failure: missing contract token {token}")

if schema.get("title") != "Fe Reader PDF Optimization Plan":
    raise SystemExit("optimization oracle smoke failure: wrong schema title")
required = schema.get("required", [])
for token in ["plan_id", "document_sha256", "level"]:
    if token not in required:
        raise SystemExit(f"optimization oracle smoke failure: schema missing required field {token}")
if schema.get("properties", {}).get("level", {}).get("enum") != [
    "inspect_only",
    "safe_rewrite",
    "size_optimise",
    "aggressive",
    "web_delivery_linearized",
]:
    raise SystemExit("optimization oracle smoke failure: level enum drifted")

receipt = {
    "operation": "pdf_optimization",
    "input_sha256": "a" * 64,
    "output_sha256": "b" * 64,
    "level": "safe_rewrite",
    "bytes_before": 10_485_760,
    "bytes_after": 7_340_032,
    "linearized": True,
    "visible_regression_max_delta": 0.0,
    "signatures_preserved": True,
    "objects_removed": 241,
    "streams_deduplicated": 12,
    "warnings": [],
}
for token in [
    "operation",
    "input_sha256",
    "output_sha256",
    "bytes_before",
    "bytes_after",
    "linearized",
    "signatures_preserved",
]:
    if token not in receipt:
        raise SystemExit(f"optimization oracle smoke failure: receipt missing token {token}")
print("optimization oracle smoke: ok")
PY
