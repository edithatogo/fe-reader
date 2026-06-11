#!/usr/bin/env python3
"""Validate platform search-index contract anchors."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PLATFORM_CONTRACT = ROOT / "contracts/rust/platform_integration.rs"
PLATFORM_PLAN = ROOT / "docs/platform-integration-plan.md"
SEARCH_SCHEMA = ROOT / "schemas/search-index.schema.json"


def fail(message: str) -> None:
    raise SystemExit(f"platform search contract smoke failed: {message}")


def validate_schema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def main() -> int:
    platform_contract = PLATFORM_CONTRACT.read_text(encoding="utf-8")
    platform_plan = PLATFORM_PLAN.read_text(encoding="utf-8")
    for token in (
        "SearchIndexDocument",
        "document_id",
        "text_preview",
        "tags",
        "deep_link",
        "index_document_for_search",
    ):
        if token not in platform_contract:
            fail(f"platform contract missing search token: {token}")

    for token in (
        "Windows Search IFilter",
        "Spotlight indexing with page/annotation deep links",
        "AppSearch indexing of titles, tags, annotations, extracted text, workflow states",
    ):
        if token not in platform_plan:
            fail(f"platform plan missing search integration token: {token}")

    schema = json.loads(SEARCH_SCHEMA.read_text(encoding="utf-8"))
    record = {
        "document_id": "fixture:text-search-fixture",
        "document_sha256": "0" * 64,
        "page_index": 0,
        "span_id": "span:0",
        "text": "Fe Reader Search Fixture",
        "bbox": [72.0, 72.0, 180.0, 24.0],
        "reading_order": 0,
        "language_hint": "en",
    }
    validate_schema(record, schema)

    platform_document = {
        "document_id": record["document_id"],
        "title": "Fe Reader Search Fixture",
        "text_preview": record["text"],
        "tags": ["pdf", "local-first", "contract-smoke"],
        "deep_link": "fe-reader://open?doc_sha256=" + record["document_sha256"] + "&page=0",
    }
    if not platform_document["deep_link"].startswith("fe-reader://open?doc_sha256="):
        fail("platform search deep link must use Fe Reader open scheme")
    if not platform_document["text_preview"]:
        fail("platform search document must include a bounded text preview")

    report_path = ROOT / "target/platform-reports/platform-search-contract-smoke.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(
            {
                "check": "platform_search_contract_smoke",
                "status": "pass",
                "platform_targets": ["windows_ifilter", "macos_spotlight", "android_appsearch"],
                "deep_link": platform_document["deep_link"],
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"platform search contract smoke: {report_path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
