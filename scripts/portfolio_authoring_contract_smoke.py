#!/usr/bin/env python3
"""Validate the PDF portfolio/collection authoring contract.

The smoke keeps the feature at the contract boundary: inspect and extract are
baseline capabilities, while authoring simple portfolios is represented as a
provider-level workflow with explicit metadata and associated-file evidence.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs/pdf-format-feature-plan.md"
PREP = ROOT / "docs/metadata-standards-preflight.md"
CORPUS = ROOT / "docs/compatibility-corpus-governance.md"
FIXTURES = ROOT / "fixtures/corpus/attachments-portfolios"
REPORT = ROOT / "target/feature-reports/portfolio-authoring-contract-smoke.json"


def fail(message: str) -> None:
    raise SystemExit(f"portfolio authoring contract smoke failed: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def main() -> int:
    for path in [DOC, PREP, CORPUS]:
        require(path.exists(), f"missing required doc: {path.relative_to(ROOT)}")

    doc = DOC.read_text(encoding="utf-8")
    for token in [
        "Portfolios/collections | Inspect and extract | Author simple portfolios",
        "Portfolios/collections.",
        "Associated-file workflows",
    ]:
        require(token in doc or token in PREP.read_text(encoding="utf-8"), f"missing portfolio token: {token}")

    preflight = PREP.read_text(encoding="utf-8")
    for token in [
        "provenance manifests",
        "Attachments/associated files",
        "Portfolios/collections.",
    ]:
        require(token in preflight, f"preflight doc missing token: {token}")

    corpus = CORPUS.read_text(encoding="utf-8")
    for token in [
        "attachments-portfolios/",
        "Every bug fix involving a PDF parser/render/conversion issue should add a fixture or generator.",
    ]:
        require(token in corpus, f"corpus governance missing token: {token}")

    require(FIXTURES.exists(), "expected portfolio fixture directory to exist")

    report = {
        "check": "portfolio_authoring_contract",
        "status": "pass",
        "portfolio_capability": "contract_only",
        "authoring_policy": "provider_level workflow with metadata and associated-file evidence",
        "inspect_extract_baseline": True,
        "simple_portfolio_authoring": False,
        "fixture_directory": "fixtures/corpus/attachments-portfolios",
        "limitations": [
            "No portfolio writer is added to fe_reader_core.",
            "Simple portfolio authoring remains a future provider implementation.",
        ],
    }
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"portfolio authoring contract smoke: {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
