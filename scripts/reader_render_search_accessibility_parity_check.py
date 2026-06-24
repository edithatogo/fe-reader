#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
OUT = EVIDENCE_DIR / "reader-render-search-accessibility-parity.json"
FAILURES: list[str] = []


def read_text(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        FAILURES.append(f"missing file: {rel}")
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def require_tokens(rel: str, tokens: list[str]) -> None:
    text = read_text(rel)
    for token in tokens:
        if token not in text:
            FAILURES.append(f"{rel} missing token: {token}")


def require_json(rel: str) -> dict:
    text = read_text(rel)
    if not text:
        return {}
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        FAILURES.append(f"{rel} invalid JSON: {exc}")
        return {}
    if not isinstance(data, dict):
        FAILURES.append(f"{rel} must contain a mapping")
        return {}
    return data


require_tokens(
    "README.md",
    [
        "docs/reader-render-search-accessibility-parity-contract.md",
        "scripts/reader_render_search_accessibility_parity_check.py",
    ],
)
for rel in [
    "docs/reader-render-search-accessibility-parity-contract.md",
    "docs/stable-reader-readiness.md",
    "docs/pdf-baseline-parity-matrix.md",
    "docs/pdf-parity-registry.md",
]:
    require_tokens(
        rel,
        [
            "reader/render/search/accessibility parity",
            "reader-render-search-accessibility-parity",
        ] if rel == "docs/reader-render-search-accessibility-parity-contract.md" else [
            "docs/reader-render-search-accessibility-parity-contract.md",
        ],
    )

stable_reader = require_json("target/release-evidence/stable-reader-readiness.json")
release_readiness = require_json("target/release-evidence/release-readiness.json")
accessibility = require_json("target/accessibility-reports/smoke.json")
search = require_json("target/search-compatibility-report.json")
visual = require_json("target/visual-regression/text-search-fixture/comparison.json")

for name, data in [
    ("stable_reader", stable_reader),
    ("release_readiness", release_readiness),
]:
    if data.get("status") != "pass":
        FAILURES.append(f"{name} must pass")

for name, data in [("accessibility", accessibility), ("search", search), ("visual", visual)]:
    if not data:
        FAILURES.append(f"missing {name} evidence")

if accessibility.get("status") not in {"pass", "ok"}:
    FAILURES.append("accessibility smoke must pass")
if search.get("report_kind") != "search-compatibility":
    FAILURES.append("search compatibility report kind drifted")
if search.get("search_fixture_id") != "text-search-fixture":
    FAILURES.append("search compatibility report must cover the accepted text fixture")
if not search.get("fixture_count", 0):
    FAILURES.append("search compatibility report must include fixtures")
if visual.get("status") not in {"pass", "ok"}:
    FAILURES.append("visual regression evidence must pass")

report = {
    "check": "reader_render_search_accessibility_parity",
    "status": "fail" if FAILURES else "pass",
    "evidence": [
        "target/release-evidence/stable-reader-readiness.json",
        "target/release-evidence/launch-qa.json",
        "target/release-evidence/release-readiness.json",
        "target/accessibility-reports/smoke.json",
        "target/search-compatibility-report.json",
        "target/visual-regression/text-search-fixture/comparison.json",
    ],
    "failures": FAILURES,
}
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
if FAILURES:
    print("READER/RENDER/SEARCH/ACCESSIBILITY PARITY CHECK FAILED")
    for failure in FAILURES:
        print(f" - {failure}")
    raise SystemExit(1)

print("reader/render/search/accessibility parity: pass")
