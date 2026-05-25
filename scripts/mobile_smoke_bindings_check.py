#!/usr/bin/env python3
"""Validate Wave 0 mobile smoke binding contracts."""

from __future__ import annotations

import json
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ANDROID = ROOT / "contracts/platform/android-intents/AndroidManifest.contract.xml"
IOS = ROOT / "contracts/platform/ios-appintents/FeReaderAppIntents.swift"
SNAPSHOT = ROOT / "contracts/snapshots/mobile/FeReader.MobileSmoke.facade.json"
UNIFFI_SNAPSHOT = ROOT / "contracts/snapshots/uniffi/fe_reader_uniffi.facade.json"
CORE_MANIFEST = ROOT / "crates/fe_reader_core/Cargo.toml"
ANDROID_NS = "{http://schemas.android.com/apk/res/android}"


def fail(message: str) -> None:
    raise SystemExit(f"mobile smoke bindings check failed: {message}")


def attr(element: ET.Element, name: str) -> str | None:
    return element.attrib.get(f"{ANDROID_NS}{name}")


def require_android_contract() -> None:
    try:
        manifest = ET.parse(ANDROID).getroot()
    except ET.ParseError as exc:
        fail(f"Android contract XML is not parseable: {exc}")

    application = manifest.find("application")
    if application is None:
        fail("Android contract is missing <application>")

    actions = {attr(action, "name") for action in manifest.findall(".//action")}
    for required in {
        "android.intent.action.VIEW",
        "android.intent.action.SEND",
        "android.content.action.DOCUMENTS_PROVIDER",
    }:
        if required not in actions:
            fail(f"Android contract missing action {required}")

    mime_types = {attr(data, "mimeType") for data in manifest.findall(".//data")}
    if "application/pdf" not in mime_types:
        fail("Android contract must declare application/pdf data handling")

    matching_provider = None
    for provider in application.findall("provider"):
        if attr(provider, "authorities") == "org.fereader.documents":
            matching_provider = provider
            break
    if matching_provider is None:
        fail("Android contract missing Fe Reader documents provider authority")

    if attr(matching_provider, "grantUriPermissions") != "true":
        fail("Android documents provider must grant URI permissions")

    text = ANDROID.read_text(encoding="utf-8")
    for guard in ("patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"):
        if guard not in text:
            fail(f"Android contract missing mutation guard note: {guard}")


def require_ios_contract() -> None:
    text = IOS.read_text(encoding="utf-8")
    for symbol in (
        "import AppIntents",
        "FeOpenDocumentIntent",
        "FeExtractPageTextIntent",
        "FePlanWorkflowIntent",
        "IntentFile",
        "ReturnsValue<String>",
    ):
        if symbol not in text:
            fail(f"iOS App Intents contract missing {symbol}")

    for guard in ("patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"):
        if guard not in text:
            fail(f"iOS contract missing mutation guard note: {guard}")

    for forbidden in ("func apply", "ApplyPatch", "approvedForApply = true"):
        if forbidden in text:
            fail(f"iOS contract exposes forbidden Wave 0 apply shape: {forbidden}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"mobile snapshot is not valid JSON: {exc}")

    if snapshot.get("surface") != "mobile_smoke_bindings":
        fail("mobile snapshot surface must be mobile_smoke_bindings")
    if snapshot.get("stability") != "preview":
        fail("mobile snapshot stability must be preview")
    if snapshot.get("mutation_policy") != "read_only_or_plan_only":
        fail("mobile snapshot must remain read_only_or_plan_only")
    if snapshot.get("source_snapshot") != str(UNIFFI_SNAPSHOT.relative_to(ROOT)):
        fail("mobile snapshot must reference the UniFFI facade snapshot")
    if snapshot.get("supports_apply") is not False:
        fail("mobile snapshot must not support apply in Wave 0")
    if snapshot.get("supports_plan_only") is not True:
        fail("mobile snapshot must support plan-only smoke coverage")

    languages = set(snapshot.get("languages", []))
    for language in ("kotlin", "swift"):
        if language not in languages:
            fail(f"mobile snapshot missing language {language}")

    required_guards = set(snapshot.get("required_mutation_guards", []))
    for guard in ("patch_plan_id", "document_hash_match", "policy_allow_rule", "approval_token"):
        if guard not in required_guards:
            fail(f"mobile snapshot missing mutation guard {guard}")

    platforms = snapshot.get("platforms", {})
    if platforms.get("android", {}).get("contract") != str(ANDROID.relative_to(ROOT)):
        fail("mobile snapshot points at the wrong Android contract path")
    if platforms.get("ios", {}).get("contract") != str(IOS.relative_to(ROOT)):
        fail("mobile snapshot points at the wrong iOS contract path")


def require_uniffi_boundary() -> None:
    try:
        snapshot = json.loads(UNIFFI_SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"UniFFI snapshot is not valid JSON: {exc}")

    if snapshot.get("mutation_policy") != "read_only_or_plan_only":
        fail("UniFFI source snapshot must remain read_only_or_plan_only")

    languages = set(snapshot.get("languages", []))
    for language in ("kotlin", "swift"):
        if language not in languages:
            fail(f"UniFFI source snapshot missing mobile language {language}")

    source = CORE_MANIFEST.read_text(encoding="utf-8").lower()
    forbidden_core_deps = (
        "uniffi",
        "jni",
        "swift",
        "android",
        "appintents",
        "tauri",
        "pdfium",
        "mcp",
        "plugin",
    )
    for dependency in forbidden_core_deps:
        if dependency in source:
            fail(f"fe_reader_core manifest contains forbidden mobile/platform dependency token: {dependency}")


def main() -> int:
    require_android_contract()
    require_ios_contract()
    require_snapshot()
    require_uniffi_boundary()
    print("mobile smoke bindings check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
