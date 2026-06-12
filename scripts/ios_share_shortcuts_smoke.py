#!/usr/bin/env python3
"""Validate iOS share extension and Shortcuts contract anchors."""

from __future__ import annotations

import json
import re
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

    if "import AppIntents" not in ios:
        fail("iOS App Intents contract missing import AppIntents")
    for intent in ("FeOpenDocumentIntent", "FeExtractPageTextIntent", "FePlanWorkflowIntent"):
        if intent not in ios:
            fail(f"iOS App Intents contract missing {intent}")
    if "IntentFile" not in ios or "ReturnsValue<String>" not in ios:
        fail("iOS App Intents contract missing file/value intent support")
    for guard in ("patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"):
        if guard not in ios:
            fail(f"iOS App Intents contract missing guard {guard}")
    if any(forbidden in ios for forbidden in ("func apply", "ApplyPatch", "approvedForApply = true")):
        fail("iOS contract exposes forbidden apply shape")
    titles = re.findall(r'static var title: LocalizedStringResource = "([^"]+)"', ios)
    if titles != ["Open Document in Fe Reader", "Extract PDF Page Text", "Plan Fe Reader Workflow"]:
        fail("iOS intent titles drifted")
    if "func perform() async throws -> some IntentResult & ReturnsValue<String>" not in ios:
        fail("iOS plan intent must return a value")

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
                "intent_titles": titles,
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
