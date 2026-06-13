#!/usr/bin/env bash
set -euo pipefail

if [[ -f templates/policy/default-security-policy.json ]]; then
  python3 -m json.tool templates/policy/default-security-policy.json >/dev/null
fi
if grep -R "execute_pdf_javascript.*true" -n docs contracts schemas templates 2>/dev/null; then
  echo "PDF JavaScript must not be enabled by default" >&2
  exit 1
fi

python3 - <<'PY'
import json
import re
from pathlib import Path
import xml.etree.ElementTree as ET

root = Path.cwd()
policy = json.loads((root / "templates/policy/default-security-policy.json").read_text())
assert policy["policy_version"] == "0.1.0"
assert policy["default_decision"] == "require_interactive_approval"
assert policy.get("rules") and isinstance(policy["rules"], list)

rules = policy["rules"]

def has_rule(match, decision):
    return any(rule.get("match") == match and rule.get("decision") == decision for rule in rules)

assert has_rule({"risk_class": "ReadOnly"}, "allow")
assert has_rule({"source": "Plugin", "risk_class": "HighMutation"}, "deny")
assert has_rule({"risk_class": "ExternalExecution"}, "deny")
assert has_rule({"risk_class": "SecureRedaction"}, "require_interactive_approval")
for kind in [
    "JavaScript",
    "Launch",
    "RemoteUri",
    "RichMedia",
    "EmbeddedExecutable",
    "SubmitForm",
]:
    assert has_rule({"risk_class": "PdfActiveContent", "kind": kind}, "deny")
assert policy.get("rules")[0].get("reason") == "read-only operations are allowed by default"

mcp_policy = json.loads(json.dumps(__import__("yaml").safe_load((root / "contracts/mcp/server-policy.yaml").read_text())))
assert mcp_policy["default_mode"] == "read_only"
mutation_policy = mcp_policy["mutation_policy"]
assert mutation_policy["destructive_tools_require"] == [
    "document_hash_match",
    "patch_plan_id",
    "user_approval_token",
    "policy_allow_rule",
]
assert mutation_policy["disabled_by_default"] == [
    "fe.apply_approved_patch",
    "fe.apply_approved_conversion",
    "fe.export_document",
    "fe.delete_pages",
]

platform_contracts = [
    root / "contracts/platform/windows-com/FeReaderAutomation.idl",
    root / "contracts/platform/linux-dbus/org.fereader.FeReader1.xml",
]
for path in platform_contracts:
    text = path.read_text()
    lowered = text.lower()
    assert "patch plan" in lowered or "patch_plan" in lowered, f"{path} missing patch plan contract"
    assert "approval token" in lowered or "approval_token" in lowered or "approvaltoken" in lowered, f"{path} missing approval token contract"

idl = (root / "contracts/platform/windows-com/FeReaderAutomation.idl").read_text()
for symbol in ["OpenDocument", "ReadPageText", "GetMetadata", "PlanWorkflow", "PlanRedaction", "ApplyApprovedPatch", "PlanConversion", "ApplyApprovedConversion"]:
    assert symbol in idl, f"missing Windows COM symbol: {symbol}"

mac_docs = (root / "docs/platform-integration-plan.md").read_text()
assert "AppleScript dictionary (`.sdef`)." in mac_docs, "missing macOS AppleScript contract note"
assert "Use `contracts/platform/windows-com/FeReaderAutomation.idl`." in mac_docs, "missing macOS automation cross-reference"

dbus = (root / "contracts/platform/linux-dbus/org.fereader.FeReader1.xml").read_text()
for symbol in ["ApplyApprovedPatch", "patch_plan_id", "approval_token", "json_receipt"]:
    assert symbol in dbus, f"missing Linux D-Bus approval contract symbol: {symbol}"
PY

echo "security policy check passed"
