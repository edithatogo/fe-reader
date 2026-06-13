#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "fixtures" / "corpus" / "manifest.json"
SEARCH_EXPECTED = ROOT / "fixtures" / "expected" / "search" / "text-search-fixture.search-index.json"
OUT_DIR = ROOT / "target"
JSON_OUT = OUT_DIR / "search-compatibility-report.json"
MD_OUT = OUT_DIR / "search-compatibility-report.md"


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    expected_records = json.loads(SEARCH_EXPECTED.read_text(encoding="utf-8"))
    fixture_ids = [item.get("fixture_id") or item.get("id") for item in manifest.get("fixtures", [])]
    search_record = expected_records[0] if expected_records else {}
    normalized_search_record = {
        **search_record,
        "text": str(search_record.get("text", "")).rstrip("\n"),
    }
    report = {
        "report_kind": "search-compatibility",
        "manifest_version": manifest.get("manifest_version"),
        "fixture_count": len(manifest.get("fixtures", [])),
        "fixture_ids": fixture_ids,
        "search_fixture_id": "text-search-fixture",
        "search_hits": len(expected_records),
        "search_record": normalized_search_record,
        "search_query": "Reader",
        "supported_text_classes": [
            "basic",
            "rtl_cjk_complex_text_placeholder",
            "malformed_adversarial",
        ],
    }

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    JSON_OUT.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    md_lines = [
        "# Search Compatibility Report",
        "",
        f"- Manifest version: {report['manifest_version']}",
        f"- Corpus fixtures: {report['fixture_count']}",
        f"- Search query: {report['search_query']}",
        f"- Search hits: {report['search_hits']}",
        "",
        "## Search Record",
        "",
        "| Field | Value |",
        "| --- | --- |",
    ]
    for key in ("document_id", "document_sha256", "page_index", "span_id", "text", "reading_order"):
        value = str(normalized_search_record.get(key, "")).replace("\n", " ")
        md_lines.append(f"| {key} | {value} |")
    md_lines.append("")
    md_lines.append("## Supported Text Classes")
    md_lines.append("")
    for item in report["supported_text_classes"]:
        md_lines.append(f"- {item}")
    md_lines.append("")
    MD_OUT.write_text("\n".join(md_lines), encoding="utf-8")
    print(f"search compatibility report: {JSON_OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
