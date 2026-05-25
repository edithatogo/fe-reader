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
from pathlib import Path

root = Path.cwd()
policy = json.loads((root / "templates/policy/default-security-policy.json").read_text())
assert policy["default_decision"] == "require_interactive_approval"

rules = policy["rules"]

def has_rule(match, decision):
    return any(rule.get("match") == match and rule.get("decision") == decision for rule in rules)

assert has_rule({"risk_class": "ReadOnly"}, "allow")
assert has_rule({"source": "Plugin", "risk_class": "HighMutation"}, "deny")
assert has_rule({"risk_class": "ExternalExecution"}, "deny")
assert has_rule({"risk_class": "SecureRedaction"}, "require_interactive_approval")

mcp_policy = (root / "contracts/mcp/server-policy.yaml").read_text()
for token in [
    "default_mode: read_only",
    "document_hash_match",
    "patch_plan_id",
    "user_approval_token",
    "policy_allow_rule",
    "fe.apply_approved_patch",
    "fe.apply_approved_conversion",
    "fe.export_document",
    "fe.delete_pages",
]:
    assert token in mcp_policy, f"missing MCP policy token: {token}"

platform_contracts = [
    root / "contracts/platform/windows-com/FeReaderAutomation.idl",
    root / "contracts/platform/macos-applescript/FeReader.sdef",
    root / "contracts/platform/linux-dbus/org.fereader.FeReader1.xml",
]
for path in platform_contracts:
    text = path.read_text()
    lowered = text.lower()
    assert "patch plan" in lowered or "patch_plan" in lowered, f"{path} missing patch plan contract"
    assert (
        "approval token" in lowered
        or "approval_token" in lowered
        or "approvaltoken" in lowered
    ), f"{path} missing approval token contract"
PY

echo "security policy check passed"
