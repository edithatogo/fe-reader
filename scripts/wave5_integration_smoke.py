#!/usr/bin/env python3
import json
import pathlib
import sys
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parents[1]


def read(rel):
    return (ROOT / rel).read_text(encoding="utf-8")


def load_json(rel):
    return json.loads(read(rel))


def require(condition, message, failures):
    if not condition:
        failures.append(message)


def check_mcp(failures):
    manifest = load_json("contracts/mcp/tools.manifest.json")
    snapshot = load_json("contracts/snapshots/mcp/fe_reader_mcp.tools.preview.json")
    names = {tool["name"] for tool in manifest["tools"]}
    snapshot_names = {tool["name"] for tool in snapshot["tools"]}
    for name in ["fe.open_document", "fe.read_page_text", "fe.get_metadata", "fe.search"]:
        require(name in names, f"MCP manifest missing read-only tool {name}", failures)
    require("fe.plan_conversion" in names, "MCP manifest missing plan_conversion", failures)
    require("fe.convert_document" not in names, "MCP manifest must not expose direct convert_document", failures)
    require(snapshot["mutation_policy"] == "read_only_or_plan_only", "MCP snapshot policy mismatch", failures)
    require(names == snapshot_names, "MCP snapshot and manifest tool names diverged", failures)


def check_platform_contracts(failures):
    com = read("contracts/platform/windows-com/FeReaderAutomation.idl")
    applescript = read("contracts/platform/macos-applescript/FeReader.sdef")
    dbus = read("contracts/platform/linux-dbus/org.fereader.FeReader1.xml")
    android = read("contracts/platform/android-intents/AndroidManifest.contract.xml")
    ios = read("contracts/platform/ios-appintents/FeReaderAppIntents.swift")
    for token in ["PlanWorkflow", "PlanRedaction", "ApplyApprovedPatch", "approvalToken"]:
        require(token in com, f"COM contract missing {token}", failures)
    for token in ["plan workflow", "apply approved patch", "approval token"]:
        require(token in applescript, f"AppleScript contract missing {token}", failures)
    ET.fromstring(dbus)
    for token in ["PlanWorkflow", "ApplyApprovedPatch", "approval_token"]:
        require(token in dbus, f"D-Bus contract missing {token}", failures)
    ET.fromstring(android)
    for token in ["patch_plan_id", "document_hash_match", "approval_token"]:
        require(token in android, f"Android contract missing {token}", failures)
    for token in ["FePlanWorkflowIntent", "policy_allow_rule", "approval_token"]:
        require(token in ios, f"iOS App Intents contract missing {token}", failures)


def check_web_and_plugin(failures):
    web = read("contracts/web/postmessage-contract.md")
    for token in [
        "plan_conversion",
        "read_only|plan_only",
        "must not apply write changes",
        "No hidden background upload",
    ]:
        require(token in web, f"web postMessage contract missing {token}", failures)
    plugin = load_json("contracts/snapshots/plugin-abi/fe_reader_plugin_host.preview.json")
    require(plugin["abi"] == "proposal_only", "plugin ABI snapshot must be proposal_only", failures)
    require("direct_apply" in plugin["denied_by_default"], "plugin ABI snapshot must deny direct apply", failures)


def check_application_integrations(failures):
    doc = read("docs/external-application-integrations.md")
    contract = read("contracts/rust/application_integration.rs")
    diagram = read("docs/mermaid/platform_integrations.mmd")
    for token in [
        "Zotero",
        "Obsidian/Logseq",
        "LibreOffice/OnlyOffice",
        "VS Code / JetBrains",
        "Email clients",
        "Nextcloud/WebDAV",
        "requires_user_grant",
        "High-risk mutation requires review and approval",
    ]:
        require(token in doc, f"external integration docs missing {token}", failures)
    for token in [
        "ApplicationIntegrationDescriptor",
        "IntegrationRequest",
        "IntegrationResponse",
        "requires_network",
        "requires_user_grant",
        "risk_notes",
    ]:
        require(token in contract, f"application integration contract missing {token}", failures)
    for token in ["Open Recent", "XDG portals", "DocumentsProvider", "App Intents"]:
        require(token in diagram, f"platform integration diagram missing {token}", failures)


def check_cli_and_sdk_policy(failures):
    cli = load_json("contracts/snapshots/cli/fe_reader_cli.commands.preview.json")
    require("inspect" in cli["commands"], "CLI snapshot missing inspect", failures)
    require("lab text-map" in cli["commands"], "CLI snapshot missing lab text-map", failures)
    sdk_doc = read("docs/developer-ecosystem-sdk.md")
    for token in ["plugin ABI version", "MCP tool manifest version", "CLI contract version", "platform automation version"]:
        require(token in sdk_doc, f"SDK versioning policy missing {token}", failures)


def check_enterprise_policy(failures):
    schema = load_json("schemas/enterprise-policy.schema.json")
    props = schema["properties"]
    for field in ["allow_plugins", "allow_mcp", "allow_external_converters", "disabled_surfaces"]:
        require(field in props, f"enterprise policy schema missing {field}", failures)
    sample = {
        "policy_version": "0.1",
        "managed": True,
        "disabled_surfaces": ["mcp", "plugins", "web_postmessage", "native_automation_apply"],
        "allow_plugins": False,
        "allow_mcp": False,
        "allow_external_converters": False,
        "telemetry_mode": "local_only",
    }
    require(not sample["allow_plugins"], "sample enterprise policy should disable plugins", failures)
    require("mcp" in sample["disabled_surfaces"], "sample enterprise policy should disable MCP", failures)


def main():
    failures = []
    try:
        check_mcp(failures)
        check_platform_contracts(failures)
        check_web_and_plugin(failures)
        check_application_integrations(failures)
        check_cli_and_sdk_policy(failures)
        check_enterprise_policy(failures)
    except Exception as exc:
        failures.append(str(exc))
    if failures:
        for failure in failures:
            print(f"wave5 integration failure: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print("wave5 integration smoke: ok")


if __name__ == "__main__":
    main()
