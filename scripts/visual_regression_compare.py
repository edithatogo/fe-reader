#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = ROOT / "fixtures/expected/rendered/text-search-fixture/metadata.json"

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None


def fail(message: str) -> None:
    print(f"visual regression smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


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

    if jsonschema is not None:
        schema = json.loads((ROOT / "schemas/visual-regression-report.schema.json").read_text())
        report = {
            "fixture_id": actual["fixture_id"],
            "page_index": actual["page_index"],
            "status": actual["status"],
            "max_delta": actual["max_delta"],
            "changed_pixels": actual["changed_pixels"],
        }
        jsonschema.validate(report, schema)

    print("visual regression smoke passed")


if __name__ == "__main__":
    main()
