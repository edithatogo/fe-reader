#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path.cwd()
MANIFEST = ROOT / "fixtures" / "corpus" / "manifest.json"
REGISTRY = ROOT / "docs" / "pdf-parity-registry.json"
FIXTURE_SCHEMA = ROOT / "schemas" / "test-fixture.schema.json"


def fail(message: str, code: int = 1) -> None:
    print(message, file=sys.stderr)
    sys.exit(code)


if not MANIFEST.exists():
    fail(
        "corpus manifest missing; creating directories is required before Wave 0 exits",
        code=0,
    )

try:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
except Exception as exc:
    fail(f"invalid corpus manifest: {exc}")

if manifest.get("manifest_version") != "0.1.0":
    fail("corpus manifest must declare manifest_version 0.1.0")
fixtures = manifest.get("fixtures")
if not isinstance(fixtures, list) or not fixtures:
    fail("corpus manifest must contain a non-empty fixtures array")

try:
    fixture_schema = json.loads(FIXTURE_SCHEMA.read_text(encoding="utf-8"))
except Exception as exc:
    fail(f"invalid fixture schema: {exc}")

try:
    registry = json.loads(REGISTRY.read_text(encoding="utf-8"))
except Exception as exc:
    fail(f"invalid parity registry: {exc}")

required = set(fixture_schema.get("required", []))
allowed = set(fixture_schema.get("properties", {}).keys()) | {
    "id",
    "category",
    "source_kind",
    "expected_behavior",
    "expected_search_index",
}
required_families = {
    family.get("id")
    for family in registry.get("families", [])
    if isinstance(family, dict) and family.get("id")
}
covered_families = set()

for item in fixtures:
    if not isinstance(item, dict):
        fail("each corpus manifest entry must be an object")
    if item.get("redistribution") == "forbidden":
        fail(f"forbidden fixture must not be committed: {item.get('fixture_id') or item.get('id')}")

    fixture_id = item.get("fixture_id") or item.get("id")
    if not fixture_id:
        fail("corpus manifest entries must include fixture_id or id")

    path = item.get("path")
    if not path:
        fail(f"corpus manifest entry {fixture_id} is missing path")
    if not (ROOT / path).exists():
        fail(f"corpus fixture path does not exist: {path}")

    if "fixture_id" in item:
        missing = [field for field in required if field not in item]
        if missing:
            fail(
                f"fixture {fixture_id} is missing required fields: {', '.join(sorted(missing))}"
            )
        extra = sorted(set(item) - allowed)
        if extra:
            fail(f"fixture {fixture_id} has unexpected fields: {', '.join(extra)}")
        parity_family = item.get("parity_family")
        if not parity_family:
            fail(f"fixture {fixture_id} must declare parity_family")
        covered_families.add(str(parity_family))

missing_families = sorted(required_families - covered_families)
if missing_families:
    fail(f"corpus manifest missing parity-family coverage for: {', '.join(missing_families)}")

print("corpus manifest check passed")
