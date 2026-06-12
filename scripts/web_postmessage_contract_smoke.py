#!/usr/bin/env python3
"""Validate the Web/PWA postMessage contract stays read-only or plan-only."""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "contracts/web/postmessage-contract.md"
DOC = ROOT / "docs/web-version.md"


def fail(message: str) -> None:
    raise SystemExit(f"web postMessage contract smoke failed: {message}")


def main() -> int:
    contract = CONTRACT.read_text(encoding="utf-8")
    doc = DOC.read_text(encoding="utf-8")

    match = re.search(r"```json\s*(\{.*?\})\s*```", contract, re.S)
    if not match:
        fail("web postMessage contract must contain a json code block")
    parsed_contract = json.loads(match.group(1))
    if parsed_contract.get("fe_reader_protocol") != "0.1":
        fail("web postMessage contract protocol drifted")
    if parsed_contract.get("origin") != "browser-extension|web-local|self-hosted":
        fail("web postMessage surface drifted")
    allowed_ops_contract = parsed_contract.get("operation", "")
    allowed_ops_contract = [item.strip() for item in allowed_ops_contract.split("|")]
    if allowed_ops_contract != ["open", "inspect", "plan_workflow", "plan_redaction", "plan_conversion"]:
        fail("web postMessage allowed operations drifted")
    allowed_risks_contract = [item.strip() for item in parsed_contract.get("risk", "").split("|")]
    if allowed_risks_contract != ["read_only", "plan_only"]:
        fail("web postMessage allowed risk levels drifted")

    for token in (
        "fe_reader_protocol",
        "browser-extension|web-local|self-hosted",
        "open|inspect|plan_workflow|plan_redaction|plan_conversion",
        "read_only|plan_only",
        "must not apply write changes",
        "No hidden background upload",
    ):
        if token not in contract:
            fail(f"contract missing token: {token}")

    for token in (
        "Browser file access requires user action",
        "never imply it can persist arbitrary local access without user grant",
        "must not bypass the operation-intent safety model",
    ):
        if token not in doc:
            fail(f"web plan missing safety token: {token}")

    messages = [
        {
            "fe_reader_protocol": "0.1",
            "message_id": "00000000-0000-4000-8000-000000000001",
            "origin": "web-local",
            "operation": "inspect",
            "risk": "read_only",
            "payload": {"document_ref": "user-granted-file-handle"},
        },
        {
            "fe_reader_protocol": "0.1",
            "message_id": "00000000-0000-4000-8000-000000000002",
            "origin": "browser-extension",
            "operation": "plan_conversion",
            "risk": "plan_only",
            "payload": {"provider": "source_pipeline_contract"},
        },
    ]
    allowed_origins = {"browser-extension", "web-local", "self-hosted"}
    allowed_operations = {"open", "inspect", "plan_workflow", "plan_redaction", "plan_conversion"}
    allowed_risks = {"read_only", "plan_only"}
    for message in messages:
        if message["fe_reader_protocol"] != "0.1":
            fail("unexpected protocol version")
        if not isinstance(message.get("payload"), dict):
            fail("message payload must be an object")
        if message["origin"] not in allowed_origins:
            fail(f"unexpected origin: {message['origin']}")
        if message["operation"] not in allowed_operations:
            fail(f"unexpected operation: {message['operation']}")
        if message["risk"] not in allowed_risks:
            fail(f"unexpected risk: {message['risk']}")
        if message["message_id"] == "00000000-0000-4000-8000-000000000001" and message["payload"].get("document_ref") != "user-granted-file-handle":
            fail("web-local inspect sample payload drifted")
        if message["message_id"] == "00000000-0000-4000-8000-000000000002" and message["payload"].get("provider") != "source_pipeline_contract":
            fail("browser-extension plan sample payload drifted")
        if message["origin"] == "web-local" and message["operation"] != "inspect":
            fail("web-local sample must be inspect only")
        if message["origin"] == "browser-extension" and message["operation"] not in {"inspect", "plan_conversion"}:
            fail("browser-extension sample drifted")

    denied_operations = {"apply_patch", "export_converted_output", "upload_document"}
    if denied_operations & allowed_operations:
        fail("direct apply/export/upload operation leaked into allowed web operations")

    report_path = ROOT / "target/web-reports/postmessage-contract-smoke.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(
            {
                "check": "web_postmessage_contract_smoke",
                "status": "pass",
                "allowed_operations": sorted(allowed_operations),
                "allowed_risks": sorted(allowed_risks),
                "denied_operations": sorted(denied_operations),
                "sample_message_count": len(messages),
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"web postMessage contract smoke: {report_path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
