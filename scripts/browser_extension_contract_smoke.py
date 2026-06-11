#!/usr/bin/env python3
"""Validate browser-extension contract boundaries."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WEB_CONTRACT = ROOT / "contracts/web/postmessage-contract.md"
EXTERNAL_DOC = ROOT / "docs/external-application-integrations.md"
WEB_DOC = ROOT / "docs/web-version.md"


def fail(message: str) -> None:
    raise SystemExit(f"browser extension contract smoke failed: {message}")


def main() -> int:
    web_contract = WEB_CONTRACT.read_text(encoding="utf-8")
    external_doc = EXTERNAL_DOC.read_text(encoding="utf-8")
    web_doc = WEB_DOC.read_text(encoding="utf-8")

    for token in (
        "origin\": \"browser-extension|web-local|self-hosted",
        "operation\": \"open|inspect|plan_workflow|plan_redaction|plan_conversion",
        "risk\": \"read_only|plan_only",
        "must not apply write changes",
        "No hidden background upload",
    ):
        if token not in web_contract:
            fail(f"postMessage contract missing browser token: {token}")

    for token in (
        "Browser extension",
        "Send current PDF/download to Fe Reader or Web Local",
        "Extension identifies embedded PDF or link to PDF",
        "Web extension cannot mutate local files directly",
        "hand off to native app",
    ):
        if token not in external_doc:
            fail(f"external integration doc missing browser token: {token}")

    if "Browser extension must not bypass the operation-intent safety model" not in web_doc:
        fail("web plan must preserve operation-intent safety model for browser extension")

    sample_messages = [
        {
            "fe_reader_protocol": "0.1",
            "message_id": "00000000-0000-4000-8000-000000000101",
            "origin": "browser-extension",
            "operation": "inspect",
            "risk": "read_only",
            "payload": {"source": "embedded-pdf-link"},
        },
        {
            "fe_reader_protocol": "0.1",
            "message_id": "00000000-0000-4000-8000-000000000102",
            "origin": "browser-extension",
            "operation": "plan_workflow",
            "risk": "plan_only",
            "payload": {"workflow_id": "review.extract_metadata"},
        },
    ]
    for message in sample_messages:
        if message["origin"] != "browser-extension":
            fail("browser smoke sample must use browser-extension origin")
        if message["risk"] not in {"read_only", "plan_only"}:
            fail("browser extension samples must stay read-only or plan-only")
        if message["operation"] in {"apply_patch", "export_converted_output", "upload_document"}:
            fail("browser extension sample contains forbidden direct mutation/export operation")

    report_path = ROOT / "target/web-reports/browser-extension-contract-smoke.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(
            {
                "check": "browser_extension_contract_smoke",
                "status": "pass",
                "origin": "browser-extension",
                "allowed_risks": ["plan_only", "read_only"],
                "direct_file_mutation": "denied",
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"browser extension contract smoke: {report_path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
