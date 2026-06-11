#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping platform recent-document smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_platform
recent_json="$(cargo run -q -p fe_reader_cli -- platform recent-smoke --json)"
automation_json="$(cargo run -q -p fe_reader_cli -- platform automation-smoke --json)"

RECENT_JSON="$recent_json" AUTOMATION_JSON="$automation_json" python3 - <<'PY'
from __future__ import annotations

import json
import os

required_guards = {
    "document_hash_match",
    "patch_plan_id",
    "policy_evaluation",
    "approval_token_or_interactive_confirmation",
    "audit_receipt_emission",
}

recent = json.loads(os.environ["RECENT_JSON"])
assert recent["contract"] == "platform_recent_document"
assert recent["status"] == "pass"
assert recent["mutation_applied"] is False
assert recent["receipt_count"] == 5
assert {receipt["platform"] for receipt in recent["receipts"]} == {
    "windows",
    "macos",
    "linux",
    "android",
    "ios",
}
for receipt in recent["receipts"]:
    assert receipt["operation"] == "register_recent_document"
    assert receipt["decision"] == "default_denied"
    assert receipt["applied"] is False
    assert set(receipt["required_guards"]) == required_guards

automation = json.loads(os.environ["AUTOMATION_JSON"])
assert automation["contract"] == "native_automation"
assert automation["status"] == "pass"
assert automation["mutation_applied"] is False
assert automation["receipt_count"] == 12
assert {receipt["surface"] for receipt in automation["receipts"]} == {
    "windows_com",
    "macos_apple_script",
    "macos_app_intent",
    "linux_dbus",
    "android_intent",
    "ios_app_intent",
}
for receipt in automation["receipts"]:
    assert receipt["operation"] in {"automation_read", "automation_plan"}
    expected_decision = "read_only" if receipt["operation"] == "automation_read" else "default_denied"
    assert receipt["decision"] == expected_decision
    assert receipt["applied"] is False
    assert set(receipt["required_guards"]) == required_guards
PY

echo "platform recent/automation smoke passed"
