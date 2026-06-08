#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = ROOT / "fixtures/expected/rendered/text-search-fixture/metadata.json"
TARGET_DIR = ROOT / "target/visual-regression/text-search-fixture"

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None


def fail(message: str) -> None:
    print(f"visual regression smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def deterministic_placeholder_bytes(metadata: dict[str, object]) -> bytes:
    if metadata.get("backend") != "null":
        fail("Wave2 smoke can only synthesize bytes for the deterministic null backend")
    if metadata.get("pixel_format") != "rgba8":
        fail("Wave2 smoke placeholder only supports rgba8")
    byte_len = metadata.get("byte_len")
    if not isinstance(byte_len, int) or byte_len <= 0:
        fail(f"byte_len must be a positive integer, got {byte_len!r}")
    return bytes(byte_len)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--smoke", action="store_true")
    args = parser.parse_args()

    if not args.smoke:
        print("visual regression comparison requires --smoke in Wave 0")
        return

    expected = json.loads(EXPECTED.read_text(encoding="utf-8"))
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "xtask", "--", "render-smoke"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    actual = json.loads(result.stdout)

    for key in ("fixture_id", "page_index", "backend", "width", "height", "pixel_format", "byte_len"):
        if actual.get(key) != expected.get(key):
            fail(f"{key} expected {expected.get(key)!r}, got {actual.get(key)!r}")
    if actual.get("status") != "pass":
        fail("render smoke status must be pass")
    if not isinstance(actual.get("cache_key"), str) or "fixture:text-search-fixture" not in actual["cache_key"]:
        fail("render smoke cache key must include fixture id")

    actual_bytes = deterministic_placeholder_bytes(actual)
    actual_sha256 = hashlib.sha256(actual_bytes).hexdigest()
    if actual_sha256 != expected.get("artifact_sha256"):
        fail(f"artifact_sha256 expected {expected.get('artifact_sha256')!r}, got {actual_sha256!r}")

    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    artifact_path = TARGET_DIR / "page-0001@smoke.rgba"
    report_path = TARGET_DIR / "comparison.json"
    artifact_path.write_bytes(actual_bytes)

    report = {
        "fixture_id": actual["fixture_id"],
        "page_index": actual["page_index"],
        "status": actual["status"],
        "max_delta": actual["max_delta"],
        "changed_pixels": actual["changed_pixels"],
    }
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/visual-regression-report.schema.json").read_text())
        jsonschema.validate(report, schema)

    print(f"visual regression smoke passed: {report_path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
