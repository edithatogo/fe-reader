#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
failures: list[str] = []


def require(path: str) -> str:
    full = ROOT / path
    if not full.exists():
        failures.append(f"missing required mutation contract: {path}")
        return ""
    return full.read_text(encoding="utf-8", errors="replace")


automation = require("contracts/rust/automation_surface.rs")
if automation:
    enum_match = re.search(r"enum\s+AutomationResult\s*\{(?P<body>.*?)\n\}", automation, re.S)
    if enum_match and "Receipt" in enum_match.group("body"):
        failures.append("AutomationResult must not return receipts from submit_intent")
    for token in [
        "apply_approved_patch",
        "document_sha256_before",
        "policy_allow_rule",
        "FeApprovalToken",
        "FeAuditReceipt",
    ]:
        if token not in automation:
            failures.append(f"automation apply contract missing token: {token}")

core_types = require("contracts/rust/core_types.rs")
if core_types:
    for token in [
        "document_sha256_before",
        "policy_allow_rule",
        "FeVerificationReport",
        "FeAuditReceipt",
    ]:
        if token not in core_types:
            failures.append(f"core mutation contract missing token: {token}")

plugin_host = require("contracts/rust/plugin_host.rs")
if plugin_host:
    for token in ["document_sha256", "policy_approval", "FeApprovalToken", "PluginProposal"]:
        if token not in plugin_host:
            failures.append(f"plugin host contract missing token: {token}")
    if "FeAuditReceipt" in plugin_host or "Receipt" in plugin_host:
        failures.append("plugin host contract must not expose audit receipts")

mcp_tools_path = ROOT / "contracts/mcp/tools.manifest.json"
if mcp_tools_path.exists():
    tools = json.loads(mcp_tools_path.read_text(encoding="utf-8"))
    if tools.get("default_mode") != "read_only":
        failures.append("MCP tools manifest must default to read_only")
    for tool in tools.get("tools", []):
        name = tool.get("name", "")
        risk = tool.get("risk", "")
        description = tool.get("description", "").lower()
        if risk in {"destructive", "external_disclosure_possible"}:
            failures.append(f"MCP tool {name} is directly {risk}; use plan-only or disabled approved apply")
        if "plan" in name and "plan" not in risk and "plan" not in description:
            failures.append(f"MCP planning tool {name} must be explicitly plan-only")
        if name == "fe.apply_approved_patch" and risk != "approved_mutation_disabled_by_default":
            failures.append("approved patch tool risk drifted")
        if name == "fe.plan_conversion" and risk != "external_disclosure_plan_only":
            failures.append("conversion planning tool risk drifted")
    names = {tool.get("name") for tool in tools.get("tools", [])}
    if "fe.convert_document" in names:
        failures.append("MCP conversion must not expose fe.convert_document directly")
else:
    failures.append("missing MCP tools manifest")

mcp_policy = require("contracts/mcp/server-policy.yaml")
if mcp_policy:
    policy = yaml.safe_load(mcp_policy)
    if not isinstance(policy, dict):
        failures.append("MCP server policy must parse as yaml mapping")
    else:
        if policy.get("default_mode") != "read_only":
            failures.append("MCP server policy must default to read_only")
        mutation = policy.get("mutation_policy", {})
        if not isinstance(mutation, dict):
            failures.append("MCP server policy missing mutation_policy mapping")
        else:
            required = mutation.get("destructive_tools_require", [])
            if required != ["document_hash_match", "patch_plan_id", "user_approval_token", "policy_allow_rule"]:
                failures.append("MCP server policy destructive tool requirements drifted")
            disabled = mutation.get("disabled_by_default", [])
            for tool_name in ["fe.apply_approved_patch", "fe.apply_approved_conversion", "fe.export_document", "fe.delete_pages"]:
                if tool_name not in disabled:
                    failures.append(f"MCP server policy missing disabled tool: {tool_name}")

windows_com = require("contracts/platform/windows-com/FeReaderAutomation.idl")
if windows_com:
    if "ConvertDocument" in windows_com:
        failures.append("COM contract must not expose direct ConvertDocument result path")
    for token in ["PlanConversion", "ApplyApprovedConversion", "documentSha256Before", "policyAllowRule", "approvalToken", "jsonAuditReceipt"]:
        if token not in windows_com:
            failures.append(f"COM conversion contract missing token: {token}")

web_contract = require("contracts/web/postmessage-contract.md")
if web_contract:
    if "destructive" in web_contract:
        failures.append("web postMessage contract must not allow destructive risk")
    for token in ["plan_conversion", "read-only or plan-only", "explicit user approval", "No hidden background upload"]:
        if token not in web_contract:
            failures.append(f"web postMessage contract missing token: {token}")

android = require("contracts/platform/android-intents/AndroidManifest.contract.xml")
if android:
    for token in ["read-only or plan-only", "patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"]:
        if token not in android:
            failures.append(f"Android automation contract missing token: {token}")

ios = require("contracts/platform/ios-appintents/FeReaderAppIntents.swift")
if ios:
    if "FeApplyWorkflowIntent" in ios:
        failures.append("iOS planning intent must not be named Apply without approval semantics")
    for token in ["FePlanWorkflowIntent", "patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"]:
        if token not in ios:
            failures.append(f"iOS App Intents contract missing token: {token}")

transaction_schema = require("schemas/operation-transaction.schema.json")
if transaction_schema:
    for token in ["allOf", "plan_generated", "approved", "applying", "verifying", "committed", "minLength"]:
        if token not in transaction_schema:
            failures.append(f"operation transaction schema missing conditional plan_id token: {token}")

if failures:
    print("STRICT MUTATION CONTRACT CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("strict mutation contract check passed")
