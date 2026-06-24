#!/usr/bin/env python3
"""Validate stable-channel release evidence requirements and waivers."""

from __future__ import annotations

import datetime as dt
import json
import os
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "target" / "release-evidence"
WAIVERS = ROOT / "packaging" / "release-waivers.yaml"
OUT = EVIDENCE / "stable-release-evidence.json"
STRICT_CHANNELS = {"stable", "lts", "store_submission"}

REQUIRED_STABLE_EVIDENCE = {
    "sbom": "target/release-evidence/sbom.cdx.json",
    "provenance": "target/release-evidence/provenance.json",
    "signing_readiness": "target/release-evidence/signing-readiness.json",
    "desktop_packaging_signing": "target/release-evidence/desktop-packaging-signing.json",
    "release_artifact_inventory": "target/release-evidence/release-artifact-inventory.json",
    "stable_reader_readiness": "target/release-evidence/stable-reader-readiness.json",
    "release_readiness": "target/release-evidence/release-readiness.json",
    "compatibility_corpus": "target/compatibility-corpus-report.json",
    "search_compatibility": "target/search-compatibility-report.json",
    "visual_regression": "target/visual-regression/text-search-fixture/comparison.json",
    "wave7_release_hardening": "target/release-evidence/wave7-release-hardening.json",
}

REQUIRED_WAIVER_FIELDS = {"id", "owner", "expires", "rationale", "rollback_path", "applies_to"}


def load_json(path: Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{path.relative_to(ROOT)} is invalid JSON: {exc}") from exc
    if not isinstance(value, dict):
        raise SystemExit(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def load_waivers() -> tuple[list[dict], list[str]]:
    doc = yaml.safe_load(WAIVERS.read_text(encoding="utf-8"))
    if not isinstance(doc, dict):
        raise SystemExit("packaging/release-waivers.yaml must contain a mapping")
    waivers = doc.get("waivers", [])
    if not isinstance(waivers, list):
        raise SystemExit("packaging/release-waivers.yaml waivers must be a list")
    failures: list[str] = []
    today = dt.date.today()
    for waiver in waivers:
        if not isinstance(waiver, dict):
            failures.append("waiver entries must be mappings")
            continue
        missing = REQUIRED_WAIVER_FIELDS - set(waiver)
        if missing:
            failures.append(f"waiver {waiver.get('id', '<unknown>')} missing {sorted(missing)}")
        try:
            expires = dt.date.fromisoformat(str(waiver.get("expires", "")))
        except ValueError:
            failures.append(f"waiver {waiver.get('id', '<unknown>')} has invalid expires date")
        else:
            if expires < today:
                failures.append(f"waiver {waiver.get('id', '<unknown>')} is expired")
        applies_to = waiver.get("applies_to")
        if not isinstance(applies_to, list) or not applies_to:
            failures.append(f"waiver {waiver.get('id', '<unknown>')} must list applies_to checks")
    return waivers, failures


def status_for(path: Path) -> str | None:
    if not path.exists() or path.suffix != ".json":
        return None
    data = load_json(path)
    status = data.get("status")
    return str(status) if status is not None else None


def main() -> int:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
    strict = channel in STRICT_CHANNELS
    waivers, failures = load_waivers()
    checks: list[dict] = []

    for name, rel_path in REQUIRED_STABLE_EVIDENCE.items():
        path = ROOT / rel_path
        exists = path.exists()
        status = status_for(path) if exists else None
        checks.append({"name": name, "path": rel_path, "exists": exists, "status": status or "present"})
        if strict and not exists:
            failures.append(f"missing stable release evidence: {rel_path}")
        if strict and status in {"blocked", "fail", "failed", "error"}:
            failures.append(f"stable release evidence {rel_path} has blocking status {status}")

    if strict and status_for(ROOT / "target/release-evidence/signing-readiness.json") != "pass":
        failures.append("stable release requires signing-readiness status pass")
    if strict and status_for(ROOT / "target/release-evidence/desktop-packaging-signing.json") != "pass":
        failures.append("stable release requires desktop-packaging-signing status pass")
    if strict and status_for(ROOT / "target/release-evidence/release-artifact-inventory.json") != "pass":
        failures.append("stable release requires release-artifact-inventory status pass")
    if strict and status_for(ROOT / "target/release-evidence/stable-reader-readiness.json") != "pass":
        failures.append("stable release requires stable-reader-readiness status pass")

    report = {
        "check": "stable_release_evidence",
        "channel": channel,
        "status": "fail" if failures else "pass",
        "strict_channels": sorted(STRICT_CHANNELS),
        "checks": checks,
        "waiver_file": "packaging/release-waivers.yaml",
        "waiver_count": len(waivers),
        "failures": failures,
    }
    OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"stable release evidence failure: {failure}")
        raise SystemExit(1)
    print("stable release evidence: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
