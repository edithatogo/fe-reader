#!/usr/bin/env python3
"""Validate iOS share extension and Shortcuts contract anchors."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
IOS = ROOT / "contracts/platform/ios-appintents/FeReaderAppIntents.swift"
PLAN = ROOT / "docs/platform-integration-plan.md"
AUTOMATION = ROOT / "docs/platform-automation-deepening.md"


def fail(message: str) -> None:
    raise SystemExit(f"iOS share/Shortcuts smoke failed: {message}")


def main() -> int:
    ios = IOS.read_text(encoding="utf-8")
    plan = PLAN.read_text(encoding="utf-8")
    automation = AUTOMATION.read_text(encoding="utf-8")

    for token in (
        "import AppIntents",
        "FeOpenDocumentIntent",
        "FeExtractPageTextIntent",
        "FePlanWorkflowIntent",
        "IntentFile",
        "ReturnsValue<String>",
    ):
        if token not in ios:
            fail(f"iOS App Intents contract missing {token}")

    for guard in ("patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"):
        if guard not in ios:
            fail(f"iOS App Intents contract missing guard {guard}")

    for forbidden in ("func apply", "ApplyPatch", "approvedForApply = true"):
        if forbidden in ios:
            fail(f"iOS contract exposes forbidden apply shape: {forbidden}")

    for token in (
        "Share/open-in support",
        "Printing/share sheet",
        "App Intents/Shortcuts",
        "Share extension",
    ):
        if token not in plan:
            fail(f"platform plan missing iOS share/Shortcuts token: {token}")

    for token in (
        "App Intents and Shortcuts for user-approved workflows",
        "Document browser and share sheet integration",
        "No silent background mutation",
    ):
        if token not in automation:
            fail(f"automation plan missing iOS safety token: {token}")

    report_path = ROOT / "target/platform-reports/ios-share-shortcuts-smoke.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(
            {
                "check": "ios_share_shortcuts_smoke",
                "status": "pass",
                "mutation_policy": "read_only_or_plan_only",
                "supports_apply": False,
                "surfaces": ["share_extension", "shortcuts", "app_intents"],
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"iOS share/Shortcuts smoke: {report_path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
