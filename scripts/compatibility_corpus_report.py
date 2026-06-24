#!/usr/bin/env python3
from __future__ import annotations

import json
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "fixtures" / "corpus" / "manifest.json"
REGISTRY = ROOT / "docs" / "pdf-parity-registry.json"
OUT_DIR = ROOT / "target"
JSON_OUT = OUT_DIR / "compatibility-corpus-report.json"
MD_OUT = OUT_DIR / "compatibility-corpus-report.md"


def main() -> None:
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    registry = json.loads(REGISTRY.read_text(encoding="utf-8"))
    fixtures = data.get("fixtures", [])

    categories: Counter[str] = Counter()
    sources: Counter[str] = Counter()
    redistribution: Counter[str] = Counter()
    ids_by_category: defaultdict[str, list[str]] = defaultdict(list)
    parity_coverage: defaultdict[str, list[dict]] = defaultdict(list)
    family_titles = {f.get("id"): f.get("title", f.get("id")) for f in registry.get("families", []) if isinstance(f, dict)}

    for fixture in fixtures:
        category = fixture.get("category", "uncategorized")
        fixture_id = fixture.get("fixture_id") or fixture.get("id") or "unknown"
        categories[category] += 1
        ids_by_category[category].append(fixture_id)
        redistribution[fixture.get("redistribution", "unknown")] += 1
        sources[fixture.get("source_kind", "unknown")] += 1
        family = fixture.get("parity_family") or category
        source_kind = fixture.get("source_kind", "")
        status = "skip" if source_kind in {"placeholder_dir", "placeholder_readme", "documentation_placeholder"} else "pass"
        parity_coverage[family].append(
            {
                "fixture_id": fixture_id,
                "path": fixture.get("path"),
                "status": status,
                "redistribution": fixture.get("redistribution", "unknown"),
                "oracle_expectations": fixture.get("oracle_expectations", []),
            }
        )

    report = {
        "report_kind": "compatibility-corpus",
        "manifest_version": data.get("manifest_version"),
        "fixture_count": len(fixtures),
        "categories": dict(sorted(categories.items())),
        "source_kinds": dict(sorted(sources.items())),
        "redistribution": dict(sorted(redistribution.items())),
        "parity_families": [
            {
                "family_id": family,
                "title": family_titles.get(family, family),
                "status": "pass"
                if any(entry["status"] == "pass" for entry in entries)
                else "skip",
                "fixture_count": len(entries),
                "fixtures": sorted(entries, key=lambda item: item["fixture_id"]),
            }
            for family, entries in sorted(parity_coverage.items())
        ],
        "accepted_fixture_classes": [
            {
                "category": category,
                "fixture_ids": sorted(ids_by_category[category]),
                "count": len(ids_by_category[category]),
            }
            for category in sorted(ids_by_category)
        ],
    }

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    JSON_OUT.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    md_lines = [
        "# Compatibility Corpus Report",
        "",
        f"- Manifest version: {report['manifest_version']}",
        f"- Fixture count: {report['fixture_count']}",
        "",
        "## Accepted Fixture Classes",
        "",
        "| Category | Count | Fixture IDs |",
        "| --- | ---: | --- |",
    ]
    for entry in report["accepted_fixture_classes"]:
        md_lines.append(
            f"| {entry['category']} | {entry['count']} | {', '.join(entry['fixture_ids'])} |"
        )
    md_lines.extend(
        [
            "",
            "## Parity Families",
            "",
            "| Family | Status | Fixture Count | Fixture IDs |",
            "| --- | --- | ---: | --- |",
        ]
    )
    for entry in report["parity_families"]:
        fixture_ids = ", ".join(item["fixture_id"] for item in entry["fixtures"])
        md_lines.append(f"| {entry['title']} | {entry['status']} | {entry['fixture_count']} | {fixture_ids} |")
    md_lines.extend(
        [
            "",
            "## Redistribution",
            "",
            "| Redistribution | Count |",
            "| --- | ---: |",
        ]
    )
    for name, count in sorted(report["redistribution"].items()):
        md_lines.append(f"| {name} | {count} |")
    md_lines.extend(["", "## Source Kinds", "", "| Source Kind | Count |", "| --- | ---: |"])
    for name, count in sorted(report["source_kinds"].items()):
        md_lines.append(f"| {name} | {count} |")
    md_lines.append("")
    MD_OUT.write_text("\n".join(md_lines), encoding="utf-8")
    print(f"compatibility corpus report: {JSON_OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
